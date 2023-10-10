use std::path::{Path, PathBuf};
use std::str;

use crate::{generate_abi_code, generate_abi_code_from_bytes, normalize_path};
use anyhow::Context;
use quote::quote;

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
        let item;
        match &self.bytes {
            None => {
                item = generate_abi_code(self.abi_path.to_string_lossy())
                    .context("generating abi code")?;
            }
            Some(bytes) => {
                item = generate_abi_code_from_bytes(bytes).context("generating abi code")?;
            }
        }

        // FIXME: We wrap into a fake module because `syn::parse2(file)` doesn't like it when there is
        // no wrapping statement. Below that we remove the first and last line of the generated code
        // which fixes the problem.
        //
        // There is probably a way to avoid that somehow?
        let file = quote! {
            mod __remove__ {
                #item
            }
        };

        let file = syn::File {
            attrs: vec![],
            items: vec![syn::parse2(file).context("parsing generated code")?],
            shebang: None,
        };

        let code = prettyplease::unparse(&file);
        let mut lines = code.lines();
        lines.next();
        lines.next_back();

        Ok(GeneratedBindings {
            code: lines.collect::<Vec<_>>().join("\n"),
        })
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
