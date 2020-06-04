fn main() {
    tonic_build::compile_protos("proto/block_service.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    tonic_build::compile_protos("proto/reporting_service.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

}
