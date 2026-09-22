fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.protoc_executable(protoc_bin_vendored::protoc_bin_path()?);
    config.compile_protos(&["../proto/oas/vehicle/v1/vehicle.proto"], &["../proto"])?;
    println!("cargo:rerun-if-changed=../proto/oas/vehicle/v1/vehicle.proto");
    Ok(())
}
