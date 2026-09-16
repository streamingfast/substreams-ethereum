//! The ABI file a contract is generated from.
//!
//! An ABI file names a contract's functions and events and the types of their
//! parameters. This reads one into the shapes the generator walks, and computes
//! the selector and topic hash a call or a log is matched by.

mod param_type;

use anyhow::{anyhow, Context};
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use std::collections::BTreeMap;
use std::io;

pub use param_type::{read_type, ParamType};

/// A function or event parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub kind: ParamType,
}

/// An event parameter, which a contract may put in a topic rather than the
/// data section.
#[derive(Debug, Clone)]
pub struct EventParam {
    pub name: String,
    pub kind: ParamType,
    pub indexed: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
}

impl Function {
    /// The four bytes a call to this function begins with.
    pub fn short_signature(&self) -> [u8; 4] {
        let kinds: Vec<_> = self.inputs.iter().map(|param| param.kind.clone()).collect();

        let mut out = [0u8; 4];
        out.copy_from_slice(&hash_signature(&self.name, &kinds)[..4]);
        out
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub inputs: Vec<EventParam>,
    pub anonymous: bool,
}

impl Event {
    /// The topic a log of this event carries in its first position.
    ///
    /// Every parameter takes part in the hash, whether the contract puts it in
    /// a topic or in the data section.
    pub fn signature(&self) -> [u8; 32] {
        let kinds: Vec<_> = self.inputs.iter().map(|param| param.kind.clone()).collect();

        hash_signature(&self.name, &kinds)
    }
}

/// The keccak hash of a name and its parameter types, as the signature a
/// selector or topic is taken over.
fn hash_signature(name: &str, kinds: &[ParamType]) -> [u8; 32] {
    let kinds: Vec<_> = kinds.iter().map(ParamType::canonical).collect();
    let signature = format!("{}({})", name, kinds.join(","));

    let mut out = [0u8; 32];
    out.copy_from_slice(&Keccak256::digest(signature.as_bytes()));
    out
}

/// A contract's functions and events, each keyed by name so overloads stay
/// together.
#[derive(Debug, Clone, Default)]
pub struct Contract {
    pub functions: BTreeMap<String, Vec<Function>>,
    pub events: BTreeMap<String, Vec<Event>>,
}

impl Contract {
    pub fn load<T: io::Read>(reader: T) -> Result<Self, anyhow::Error> {
        let entries: Vec<Entry> =
            serde_json::from_reader(reader).context("reading the ABI file")?;

        let mut contract = Contract::default();
        for entry in entries {
            match entry.kind.as_str() {
                "function" => {
                    let name = sanitize_name(entry.name.clone().unwrap_or_default());
                    let function = Function {
                        name: name.clone(),
                        inputs: params(entry.inputs)?,
                        outputs: params(entry.outputs)?,
                    };

                    contract.functions.entry(name).or_default().push(function);
                }
                "event" => {
                    let name = sanitize_name(entry.name.clone().unwrap_or_default());
                    let event = Event {
                        name: name.clone(),
                        inputs: event_params(entry.inputs)?,
                        anonymous: entry.anonymous,
                    };

                    contract.events.entry(name).or_default().push(event);
                }
                // A constructor, fallback, receive or error declares nothing the
                // generator emits.
                _ => {}
            }
        }

        Ok(contract)
    }
}

/// Strips a parameter list a non-spec-compliant ABI file may have written into
/// the name, which the hash is taken without.
fn sanitize_name(mut name: String) -> String {
    if let Some(at) = name.find('(') {
        name.truncate(at);
    }

    name
}

fn params(entries: Vec<ParamEntry>) -> Result<Vec<Param>, anyhow::Error> {
    entries
        .into_iter()
        .map(|entry| {
            Ok(Param {
                name: entry.name.clone().unwrap_or_default(),
                kind: kind_of(&entry)?,
            })
        })
        .collect()
}

fn event_params(entries: Vec<ParamEntry>) -> Result<Vec<EventParam>, anyhow::Error> {
    entries
        .into_iter()
        .map(|entry| {
            Ok(EventParam {
                name: entry.name.clone().unwrap_or_default(),
                kind: kind_of(&entry)?,
                indexed: entry.indexed,
            })
        })
        .collect()
}

/// Reads a parameter's type, filling a tuple's fields in from the components
/// the ABI file lists beside it.
fn kind_of(entry: &ParamEntry) -> Result<ParamType, anyhow::Error> {
    let mut kind = read_type(&entry.kind);

    if let Some(fields) = kind.inner_tuple_mut() {
        let components = entry
            .components
            .as_ref()
            .ok_or_else(|| anyhow!("a '{}' parameter declares no components", entry.kind))?;

        for component in components {
            fields.push(kind_of(component)?);
        }
    }

    Ok(kind)
}

/// One entry of the ABI file's top-level array.
#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "type")]
    kind: String,
    name: Option<String>,
    #[serde(default)]
    inputs: Vec<ParamEntry>,
    #[serde(default)]
    outputs: Vec<ParamEntry>,
    #[serde(default)]
    anonymous: bool,
}

/// One parameter of an entry, which carries its own components when it is a
/// tuple.
#[derive(Deserialize)]
struct ParamEntry {
    name: Option<String>,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    indexed: bool,
    components: Option<Vec<ParamEntry>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(json: &str) -> Contract {
        Contract::load(json.as_bytes()).expect("a valid ABI")
    }

    #[test]
    fn it_reads_a_function_and_its_selector() {
        let contract = load(
            r#"[{"type":"function","name":"transfer",
                 "inputs":[{"name":"to","type":"address"},{"name":"value","type":"uint256"}],
                 "outputs":[{"name":"","type":"bool"}]}]"#,
        );

        let function = &contract.functions["transfer"][0];
        assert_eq!(function.inputs.len(), 2);
        assert_eq!(function.outputs.len(), 1);
        assert_eq!(function.short_signature(), [0xa9, 0x05, 0x9c, 0xbb]);
    }

    #[test]
    fn it_reads_an_event_and_its_topic() {
        let contract = load(
            r#"[{"type":"event","name":"Transfer","anonymous":false,
                 "inputs":[{"indexed":true,"name":"from","type":"address"},
                           {"indexed":true,"name":"to","type":"address"},
                           {"indexed":false,"name":"value","type":"uint256"}]}]"#,
        );

        let event = &contract.events["Transfer"][0];
        assert_eq!(event.inputs.iter().filter(|p| p.indexed).count(), 2);

        // keccak("Transfer(address,address,uint256)")
        assert_eq!(
            hex::encode(event.signature()),
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
    }

    #[test]
    fn it_fills_a_tuple_from_its_components() {
        let contract = load(
            r#"[{"type":"function","name":"f",
                 "inputs":[{"name":"a","type":"tuple",
                            "components":[{"name":"x","type":"string"},
                                          {"name":"y","type":"uint256"}]}],
                 "outputs":[]}]"#,
        );

        let kind = &contract.functions["f"][0].inputs[0].kind;
        assert_eq!(
            *kind,
            ParamType::Tuple(vec![ParamType::String, ParamType::Uint(256)])
        );
        assert_eq!(kind.canonical(), "(string,uint256)");
    }

    #[test]
    fn it_fills_a_tuple_array_from_its_components() {
        let contract = load(
            r#"[{"type":"function","name":"f",
                 "inputs":[{"name":"a","type":"tuple[]",
                            "components":[{"name":"x","type":"string"}]}],
                 "outputs":[]}]"#,
        );

        let kind = &contract.functions["f"][0].inputs[0].kind;
        assert_eq!(kind.canonical(), "(string)[]");
        assert!(kind.is_dynamic());
    }

    #[test]
    fn it_keeps_overloads_of_the_same_name_together() {
        let contract = load(
            r#"[{"type":"function","name":"f","inputs":[{"name":"a","type":"uint256"}],"outputs":[]},
                {"type":"function","name":"f","inputs":[{"name":"a","type":"int128"}],"outputs":[]}]"#,
        );

        assert_eq!(contract.functions["f"].len(), 2);
        assert_ne!(
            contract.functions["f"][0].short_signature(),
            contract.functions["f"][1].short_signature()
        );
    }

    #[test]
    fn it_ignores_entries_that_generate_nothing() {
        let contract = load(
            r#"[{"type":"constructor","inputs":[]},
                {"type":"fallback"},
                {"type":"receive"},
                {"type":"error","name":"Bad","inputs":[]},
                {"type":"function","name":"f","inputs":[],"outputs":[]}]"#,
        );

        assert_eq!(contract.functions.len(), 1);
        assert!(contract.events.is_empty());
    }

    #[test]
    fn it_strips_a_parameter_list_written_into_the_name() {
        let contract = load(
            r#"[{"type":"event","name":"Transfer(address,address,uint256)","anonymous":false,
                 "inputs":[{"indexed":true,"name":"from","type":"address"},
                           {"indexed":true,"name":"to","type":"address"},
                           {"indexed":false,"name":"value","type":"uint256"}]}]"#,
        );

        let event = &contract.events["Transfer"][0];
        assert_eq!(
            hex::encode(event.signature()),
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
    }

    #[test]
    fn it_reports_a_tuple_that_declares_no_components() {
        let failed = Contract::load(
            r#"[{"type":"function","name":"f",
                 "inputs":[{"name":"a","type":"tuple"}],"outputs":[]}]"#
                .as_bytes(),
        );

        assert!(failed.is_err());
    }
}
