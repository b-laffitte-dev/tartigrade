//! Build script for Tardigrade Git module
//!
//! This script generates gRPC code from .proto files.

use std::env;
use std::path::Path;

fn main() {
    // Check if we should generate gRPC code
    if env::var("CARGO_FEATURE_GRPC").is_ok() {
        generate_grpc_code();
    }
}

fn generate_grpc_code() {
    // Path to proto file
    let proto_path = "protos/git_service.proto";
    
    // Output directory for generated code
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Configure tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_path(&out_dir)
        .compile(&[proto_path], &["protos/"])
        .expect("Failed to compile proto files");
    
    // Print info
    println!("cargo:rerun-if-changed={}", proto_path);
}
