//! Checks the ABI reader against `ethabi` on the same files.
//!
//! A selector or topic hash that differs by one byte stops matching every call
//! or log it was generated for, with no error to notice, so the reader is pinned
//! against the library it replaced rather than against expectations written by
//! hand.

use std::collections::BTreeSet;
use substreams_ethereum_abigen::abi::{Contract, ParamType};

/// Every ABI the repository generates from, plus shapes a contract can declare
/// that those files happen not to cover.
fn abi_files() -> Vec<(&'static str, String)> {
    let mut files: Vec<(&'static str, String)> = Vec::new();

    for path in [
        "../abigen-tests/abi/tests.json",
        "../substreams-ethereum/examples/abi/erc721.json",
    ] {
        if let Ok(content) = std::fs::read_to_string(path) {
            files.push((Box::leak(path.to_string().into_boxed_str()), content));
        }
    }

    files.push((
        "inline: widths and arrays",
        r#"[
            {"type":"function","name":"widths","inputs":[
                {"name":"a","type":"uint8"},{"name":"b","type":"uint16"},
                {"name":"c","type":"uint24"},{"name":"d","type":"uint160"},
                {"name":"e","type":"int8"},{"name":"f","type":"int24"},
                {"name":"g","type":"int128"},{"name":"h","type":"int256"},
                {"name":"i","type":"bytes1"},{"name":"j","type":"bytes31"},
                {"name":"k","type":"uint"},{"name":"l","type":"int"}],
             "outputs":[]},
            {"type":"function","name":"arrays","inputs":[
                {"name":"a","type":"address[]"},{"name":"b","type":"address[3]"},
                {"name":"c","type":"uint256[][]"},{"name":"d","type":"uint256[2][]"},
                {"name":"e","type":"string[]"},{"name":"f","type":"bytes[2]"}],
             "outputs":[]}
        ]"#
        .to_string(),
    ));

    files.push((
        "inline: nested tuples",
        r#"[
            {"type":"function","name":"nested","inputs":[
                {"name":"a","type":"tuple","components":[
                    {"name":"x","type":"string"},
                    {"name":"y","type":"tuple","components":[
                        {"name":"p","type":"uint256"},
                        {"name":"q","type":"address[]"}]}]}],
             "outputs":[]},
            {"type":"function","name":"tupleArray","inputs":[
                {"name":"a","type":"tuple[]","components":[
                    {"name":"x","type":"uint256"},{"name":"y","type":"bytes"}]}],
             "outputs":[]},
            {"type":"function","name":"tupleFixedArray","inputs":[
                {"name":"a","type":"tuple[2]","components":[
                    {"name":"x","type":"bool"}]}],
             "outputs":[]},
            {"type":"event","name":"WithTuple","anonymous":false,"inputs":[
                {"indexed":true,"name":"who","type":"address"},
                {"indexed":false,"name":"what","type":"tuple","components":[
                    {"name":"x","type":"uint256"},{"name":"y","type":"string"}]}]}
        ]"#
        .to_string(),
    ));

    files.push((
        "inline: an enum parameter",
        r#"[
            {"type":"function","name":"withEnum","inputs":[
                {"name":"a","type":"MyEnum"},{"name":"b","type":"uint256"}],
             "outputs":[]}
        ]"#
        .to_string(),
    ));

    files
}

/// Our type rendered so it can be compared with `ethabi`'s.
fn ours(kind: &ParamType) -> String {
    kind.canonical()
}

fn theirs(kind: &ethabi::ParamType) -> String {
    ethabi::param_type::Writer::write(kind)
}

#[test]
fn it_reads_every_type_as_ethabi_does() {
    for (label, content) in abi_files() {
        let ours_contract = Contract::load(content.as_bytes()).expect("our reader accepts it");
        let theirs_contract =
            ethabi::Contract::load(content.as_bytes()).expect("ethabi accepts it");

        let our_names: BTreeSet<_> = ours_contract.functions.keys().cloned().collect();
        let their_names: BTreeSet<_> = theirs_contract.functions.keys().cloned().collect();
        assert_eq!(our_names, their_names, "{}: function names", label);

        let our_events: BTreeSet<_> = ours_contract.events.keys().cloned().collect();
        let their_events: BTreeSet<_> = theirs_contract.events.keys().cloned().collect();
        assert_eq!(our_events, their_events, "{}: event names", label);

        for (name, functions) in &ours_contract.functions {
            let their_overloads = &theirs_contract.functions[name];
            assert_eq!(
                functions.len(),
                their_overloads.len(),
                "{}: {} overloads",
                label,
                name
            );

            for (ours_fn, theirs_fn) in functions.iter().zip(their_overloads) {
                let our_inputs: Vec<_> = ours_fn.inputs.iter().map(|p| ours(&p.kind)).collect();
                let their_inputs: Vec<_> =
                    theirs_fn.inputs.iter().map(|p| theirs(&p.kind)).collect();
                assert_eq!(our_inputs, their_inputs, "{}: {} inputs", label, name);

                let our_outputs: Vec<_> = ours_fn.outputs.iter().map(|p| ours(&p.kind)).collect();
                let their_outputs: Vec<_> =
                    theirs_fn.outputs.iter().map(|p| theirs(&p.kind)).collect();
                assert_eq!(our_outputs, their_outputs, "{}: {} outputs", label, name);
            }
        }

        for (name, events) in &ours_contract.events {
            let their_events = &theirs_contract.events[name];
            assert_eq!(
                events.len(),
                their_events.len(),
                "{}: {} overloads",
                label,
                name
            );

            for (ours_event, theirs_event) in events.iter().zip(their_events) {
                let our_inputs: Vec<_> = ours_event
                    .inputs
                    .iter()
                    .map(|p| (ours(&p.kind), p.indexed))
                    .collect();
                let their_inputs: Vec<_> = theirs_event
                    .inputs
                    .iter()
                    .map(|p| (theirs(&p.kind), p.indexed))
                    .collect();
                assert_eq!(our_inputs, their_inputs, "{}: {} inputs", label, name);

                assert_eq!(
                    ours_event.anonymous, theirs_event.anonymous,
                    "{}: {} anonymous",
                    label, name
                );
            }
        }
    }
}

#[test]
fn it_computes_every_selector_as_ethabi_does() {
    let mut checked = 0usize;

    for (label, content) in abi_files() {
        let ours_contract = Contract::load(content.as_bytes()).expect("our reader accepts it");
        let theirs_contract =
            ethabi::Contract::load(content.as_bytes()).expect("ethabi accepts it");

        for (name, functions) in &ours_contract.functions {
            for (ours_fn, theirs_fn) in functions.iter().zip(&theirs_contract.functions[name]) {
                assert_eq!(
                    ours_fn.short_signature(),
                    theirs_fn.short_signature(),
                    "{}: selector for {}",
                    label,
                    name
                );
                checked += 1;
            }
        }
    }

    assert!(checked > 20, "only {} selectors compared", checked);
}

#[test]
fn it_computes_every_topic_hash_as_ethabi_does() {
    let mut checked = 0usize;

    for (label, content) in abi_files() {
        let ours_contract = Contract::load(content.as_bytes()).expect("our reader accepts it");
        let theirs_contract =
            ethabi::Contract::load(content.as_bytes()).expect("ethabi accepts it");

        for (name, events) in &ours_contract.events {
            for (ours_event, theirs_event) in events.iter().zip(&theirs_contract.events[name]) {
                assert_eq!(
                    ours_event.signature(),
                    theirs_event.signature().to_fixed_bytes(),
                    "{}: topic hash for {}",
                    label,
                    name
                );
                checked += 1;
            }
        }
    }

    assert!(checked > 20, "only {} topic hashes compared", checked);
}

#[test]
fn it_agrees_on_which_types_are_dynamic() {
    for (label, content) in abi_files() {
        let ours_contract = Contract::load(content.as_bytes()).expect("our reader accepts it");
        let theirs_contract =
            ethabi::Contract::load(content.as_bytes()).expect("ethabi accepts it");

        for (name, functions) in &ours_contract.functions {
            for (ours_fn, theirs_fn) in functions.iter().zip(&theirs_contract.functions[name]) {
                for (ours_param, theirs_param) in ours_fn.inputs.iter().zip(&theirs_fn.inputs) {
                    assert_eq!(
                        ours_param.kind.is_dynamic(),
                        theirs_param.kind.is_dynamic(),
                        "{}: {} param {} dynamic",
                        label,
                        name,
                        ours_param.name
                    );
                }
            }
        }
    }
}

#[test]
fn it_covers_a_meaningful_number_of_shapes() {
    let (mut functions, mut events, mut params) = (0usize, 0usize, 0usize);

    for (_, content) in abi_files() {
        let contract = Contract::load(content.as_bytes()).expect("our reader accepts it");

        for overloads in contract.functions.values() {
            functions += overloads.len();
            for function in overloads {
                params += function.inputs.len() + function.outputs.len();
            }
        }

        for overloads in contract.events.values() {
            events += overloads.len();
            for event in overloads {
                params += event.inputs.len();
            }
        }
    }

    println!("compared {functions} functions, {events} events, {params} parameters");
    assert!(functions >= 20, "only {functions} functions");
    assert!(events >= 20, "only {events} events");
    assert!(params >= 100, "only {params} parameters");
}

/// Real contract ABIs, when a checkout that carries them is present.
///
/// The fixtures in this repository declare the shapes its own tests need. A
/// contract in the wild declares others, so the reader is also walked over
/// whatever real ABIs the machine has before the comparison is trusted.
fn corpus_files() -> Vec<(String, String)> {
    let mut files = Vec::new();

    let Some(home) = std::env::var_os("HOME") else {
        return files;
    };

    let roots = ["Documents/SF/optimism/packages/contracts-bedrock/snapshots/abi"];

    for root in roots {
        let path = std::path::Path::new(&home).join(root);
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };

            // Only a bare ABI array is a contract's interface; anything else is
            // a build artifact that happens to live beside one.
            if !content.trim_start().starts_with('[') {
                continue;
            }

            files.push((path.display().to_string(), content));
        }
    }

    files
}

#[test]
fn it_reads_real_contract_abis_as_ethabi_does() {
    let corpus = corpus_files();
    if corpus.is_empty() {
        eprintln!("no real ABI corpus on this machine, skipping");
        return;
    }

    let (mut files, mut functions, mut events, mut params) = (0usize, 0usize, 0usize, 0usize);

    for (label, content) in corpus {
        let Ok(theirs_contract) = ethabi::Contract::load(content.as_bytes()) else {
            continue;
        };
        let ours_contract = Contract::load(content.as_bytes())
            .unwrap_or_else(|err| panic!("{}: our reader rejected it: {}", label, err));

        files += 1;

        for (name, overloads) in &ours_contract.functions {
            let their_overloads = &theirs_contract.functions[name];
            assert_eq!(
                overloads.len(),
                their_overloads.len(),
                "{}: {} overloads",
                label,
                name
            );

            for (ours_fn, theirs_fn) in overloads.iter().zip(their_overloads) {
                let our_inputs: Vec<_> = ours_fn.inputs.iter().map(|p| ours(&p.kind)).collect();
                let their_inputs: Vec<_> =
                    theirs_fn.inputs.iter().map(|p| theirs(&p.kind)).collect();
                assert_eq!(our_inputs, their_inputs, "{}: {} inputs", label, name);

                let our_outputs: Vec<_> = ours_fn.outputs.iter().map(|p| ours(&p.kind)).collect();
                let their_outputs: Vec<_> =
                    theirs_fn.outputs.iter().map(|p| theirs(&p.kind)).collect();
                assert_eq!(our_outputs, their_outputs, "{}: {} outputs", label, name);

                assert_eq!(
                    ours_fn.short_signature(),
                    theirs_fn.short_signature(),
                    "{}: selector for {}",
                    label,
                    name
                );

                params += ours_fn.inputs.len() + ours_fn.outputs.len();
                functions += 1;
            }
        }

        for (name, overloads) in &ours_contract.events {
            let their_events = &theirs_contract.events[name];
            assert_eq!(
                overloads.len(),
                their_events.len(),
                "{}: {} overloads",
                label,
                name
            );

            for (ours_event, theirs_event) in overloads.iter().zip(their_events) {
                let our_inputs: Vec<_> = ours_event
                    .inputs
                    .iter()
                    .map(|p| (ours(&p.kind), p.indexed))
                    .collect();
                let their_inputs: Vec<_> = theirs_event
                    .inputs
                    .iter()
                    .map(|p| (theirs(&p.kind), p.indexed))
                    .collect();
                assert_eq!(our_inputs, their_inputs, "{}: {} inputs", label, name);

                assert_eq!(
                    ours_event.signature(),
                    theirs_event.signature().to_fixed_bytes(),
                    "{}: topic hash for {}",
                    label,
                    name
                );

                params += ours_event.inputs.len();
                events += 1;
            }
        }
    }

    println!(
        "real corpus: {files} files, {functions} functions, {events} events, {params} parameters"
    );
    assert!(files > 50, "only {files} real ABIs walked");
}
