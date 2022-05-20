use std::io::Result;
fn main() -> Result<()> {
    let regenerate = std::env::var("SUBTREAMS_ETHEREUM_REGENERATE_PROTO")
        .unwrap_or("false".to_string())
        != "false";
    if !regenerate {
        return Ok(());
    }

    let proto_path = std::env::var("SUBTREAMS_ETHEREUM_PROTO_PATH")
        .unwrap_or("../sf-ethereum/proto".to_string());
    println!("cargo:rerun-if-changed={}", proto_path);

    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("./src/pb");
    prost_build.compile_protos(&["sf/ethereum/type/v1/type.proto"], &[proto_path])
}
