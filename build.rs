use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
        .compile_protos(&["proto/calculator.proto"], &["proto"])
        .unwrap();

    // tonic_build::compile_protos().unwrap();
}
