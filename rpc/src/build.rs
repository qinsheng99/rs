fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("data")
        .compile(&["proto/helloworld.proto"], &["data"])
        .unwrap();
}
