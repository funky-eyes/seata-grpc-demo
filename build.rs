use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .compile_protos(&["proto/grpcMessage.proto"], &["proto"])
        .unwrap();
}