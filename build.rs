extern crate protoc_grpcio;

// use http::{Request, Response};

fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["schema.proto", "output.proto"],
        &[proto_root],
        &proto_root,
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
