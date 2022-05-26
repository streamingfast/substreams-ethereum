use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    // All `path` arguments is relative to crate's Cargo.toml directory, in this example, it's 'abigen'
    Abigen::new("erc721", "examples/abi/erc721.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;

    Ok(())
}
