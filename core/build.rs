use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    // Only generate Quick Protobuf code if explicitly requested
    if env::var("GENERATE_QUICK_PB").is_ok() {
        generate_quick_protobuf();
    }
}

fn generate_quick_protobuf() {
    use pb_rs::types::FileDescriptor;
    use pb_rs::ConfigBuilder;

    println!("cargo:warning=Generating Quick Protobuf code...");

    let proto_dir = env::var("PROTO_DIR").unwrap_or_else(|_| {
        panic!("PROTO_DIR environment variable must be set when GENERATE_QUICK_PB=1");
    });

    let proto_dir = PathBuf::from(proto_dir);
    let output_dir = PathBuf::from("src/pb/quick");

    // Create a temporary directory for pb-rs output
    let temp_dir = env::temp_dir().join("pb-rs-output");
    fs::create_dir_all(&temp_dir).unwrap();

    // Find all .proto files, but filter out unnecessary Google protobuf types
    let proto_files: Vec<PathBuf> = WalkDir::new(&proto_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "proto").unwrap_or(false))
        .filter(|e| {
            let path = e.path();
            let path_str = path.to_string_lossy();

            // Include all sf/ethereum files
            if path_str.contains("sf/ethereum") {
                return true;
            }

            // Only include the specific Google protobuf types used by the Ethereum protos.
            // Important: all files in google/protobuf share the same package name, so pb-rs
            // would overwrite the output file for each one — only include what we actually need.
            if path_str.contains("google/protobuf") {
                return path_str.contains("timestamp.proto");
            }

            false
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if proto_files.is_empty() {
        panic!("No .proto files found in {}", proto_dir.display());
    }

    println!("cargo:warning=Found {} proto files", proto_files.len());
    for proto_file in &proto_files {
        println!("cargo:warning=  - {}", proto_file.display());
    }

    // Preprocess proto files to remove unsupported options
    let preprocessed_dir = env::temp_dir().join("pb-rs-preprocessed");
    fs::create_dir_all(&preprocessed_dir).unwrap();
    let preprocessed_files = preprocess_proto_files(&proto_files, &proto_dir, &preprocessed_dir);

    // Use pb-rs to generate code with proper include paths
    println!("cargo:warning=Generating code with pb-rs...");
    let config = ConfigBuilder::new(
        &preprocessed_files,
        None,
        Some(&temp_dir),
        &[preprocessed_dir.clone()], // Include path for resolving imports
    )
    .unwrap();

    if let Err(e) = FileDescriptor::run(&config.build()) {
        panic!("pb-rs generation failed: {}", e);
    }

    // Organize the generated files
    organize_modules(&temp_dir, &output_dir);

    // Clean up temp directories
    fs::remove_dir_all(&temp_dir).ok();
    fs::remove_dir_all(&preprocessed_dir).ok();

    println!("cargo:warning=Quick Protobuf generation complete!");
    println!("cargo:warning=Files written to src/pb/quick/");
}

// pb-rs doesn't escape Rust reserved keywords used as proto package segment names.
// This fixes generated code that uses `type` as a module name.
fn fix_reserved_keywords(content: &str) -> String {
    content
        .replace("pub mod type;", "pub mod r#type;")
        .replace("::type::", "::r#type::")
}

fn preprocess_proto_files(
    proto_files: &[PathBuf],
    proto_dir: &Path,
    output_dir: &Path,
) -> Vec<PathBuf> {
    let mut preprocessed_files = Vec::new();

    for proto_file in proto_files {
        let content = fs::read_to_string(proto_file).unwrap();

        // Remove unsupported option lines
        let cleaned_content: Vec<&str> = content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                // Skip option lines that pb-rs doesn't support
                !(trimmed.starts_with("option go_package")
                    || trimmed.starts_with("option java_package")
                    || trimmed.starts_with("option java_outer_classname")
                    || trimmed.starts_with("option csharp_namespace")
                    || trimmed.starts_with("option objc_class_prefix")
                    || trimmed.starts_with("option php_namespace"))
            })
            .collect();

        // Preserve the relative path structure
        let relative_path = proto_file.strip_prefix(proto_dir).unwrap();
        let output_file = output_dir.join(relative_path);

        // Create parent directories
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Write the cleaned content
        fs::write(&output_file, cleaned_content.join("\n")).unwrap();
        preprocessed_files.push(output_file);
    }

    preprocessed_files
}

fn organize_modules(input_dir: &Path, output_dir: &Path) {
    // Clean the output directory to remove any stale files from previous runs
    if output_dir.exists() {
        fs::remove_dir_all(output_dir).unwrap();
    }
    fs::create_dir_all(output_dir).unwrap();

    // Recursively copy all generated files from pb-rs output, preserving directory structure
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(input_dir).unwrap();

        if relative_path.as_os_str().is_empty() {
            continue;
        }

        let dest_path = output_dir.join(relative_path);

        if source_path.is_dir() {
            fs::create_dir_all(&dest_path).unwrap();
        } else if source_path.extension().map(|s| s == "rs").unwrap_or(false) {
            let content = fs::read_to_string(source_path).unwrap();
            let content = fix_reserved_keywords(&content);
            fs::write(&dest_path, &content).unwrap();
            println!("cargo:warning=Created: {}", dest_path.display());
        }
    }
}
