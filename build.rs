fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]");
    config.compile_well_known_types();

    tower_grpc_build::Config::from_prost(config)
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/test.proto"], &["proto/protobuf/src", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
