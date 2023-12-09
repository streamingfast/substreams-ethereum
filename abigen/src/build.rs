use std::path::{Path, PathBuf};
use std::str;

use crate::{generate_abi_code, generate_abi_code_from_bytes, normalize_path};
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Abigen<'a> {
    /// The path where to find the source of the ABI JSON for the contract whose bindings
    /// are being generated.
    abi_path: PathBuf,

    /// The bytes of the ABI for the contract whose bindings are being generated.
    bytes: Option<&'a [u8]>,
}

impl<'a> Abigen<'a> {
    /// Creates a new builder for the given contract name and where the ABI JSON file can be found
    /// at `path`, which is relative to the your crate's root directory (where `Cargo.toml` file is located).
    pub fn new<S: AsRef<str>>(_contract_name: S, path: S) -> Result<Self, anyhow::Error> {
        let path = normalize_path(path.as_ref()).context("normalize path")?;

        Ok(Self {
            abi_path: path,
            bytes: None,
        })
    }

    /// Creates a new builder for the given contract name and where the ABI bytes can be found
    /// at 'abi_bytes'.
    pub fn from_bytes<S: AsRef<str>>(
        _contract_name: S,
        abi_bytes: &'a [u8],
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            abi_path: "".parse()?,
            bytes: Some(abi_bytes),
        })
    }

    pub fn generate(&self) -> Result<GeneratedBindings, anyhow::Error> {
        let tokens = match &self.bytes {
            None => generate_abi_code(self.abi_path.to_string_lossy()),
            Some(bytes) => generate_abi_code_from_bytes(bytes),
        }
        .context("generating abi code")?;

        let file = syn::parse_file(&tokens.to_string()).context("parsing generated code")?;

        let code = prettyplease::unparse(&file)
            .lines()
            .collect::<Vec<_>>()
            .join("\n");

        Ok(GeneratedBindings { code })
    }
}

pub struct GeneratedBindings {
    code: String,
}

impl GeneratedBindings {
    pub fn write_to_file<P: AsRef<Path>>(&self, p: P) -> Result<(), anyhow::Error> {
        let path = normalize_path(p.as_ref()).context("normalize path")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating directories for {}", parent.to_string_lossy()))?
        }

        std::fs::write(path, &self.code)
            .with_context(|| format!("writing file {}", p.as_ref().to_string_lossy()))
    }
}
