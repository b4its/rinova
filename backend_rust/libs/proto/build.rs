fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the blockchain proto into Rust (server + client stubs).
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/blockchain.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/blockchain.proto");
    Ok(())
}
