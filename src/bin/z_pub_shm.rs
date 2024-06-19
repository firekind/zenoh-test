use zenoh::{
    prelude::*,
    shm::{
        DeallocEldest, Deallocate, JustAlloc, PosixShmProviderBackend, ShmProviderBuilder,
        POSIX_PROTOCOL_ID,
    },
};

const N: usize = 10;

#[tokio::main]
async fn main() -> Result<(), ZError> {
    // Initiate logging
    zenoh::try_init_log_from_env();

    let mut config = zenoh::Config::default();
    config.set_mode(Some(zenoh::config::WhatAmI::Peer)).unwrap();
    config.transport.shared_memory.set_enabled(true).unwrap();
    let path = "demo/example/zenoh-rs-pub".to_owned();
    let payload = "Hello";

    // A probing procedure for shared memory is performed upon session opening. To enable `z_pub_shm` to operate
    // over shared memory (and to not fallback on network mode), shared memory needs to be enabled also on the
    // subscriber side. By doing so, the probing procedure will succeed and shared memory will operate as expected.
    config.transport.shared_memory.set_enabled(true).unwrap();

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Creating POSIX SHM provider...");
    // create an SHM backend...
    // NOTE: For extended PosixShmProviderBackend API please check z_posix_shm_provider.rs
    let backend = PosixShmProviderBackend::builder()
        .with_size(N * 1024)
        .unwrap()
        .res()
        .unwrap();
    // ...and an SHM provider
    let provider = ShmProviderBuilder::builder()
        .protocol_id::<POSIX_PROTOCOL_ID>()
        .backend(backend)
        .res();

    let publisher = session.declare_publisher(&path).await.unwrap();

    // Create allocation layout for series of similar allocations
    println!("Allocating Shared Memory Buffer...");
    let layout = provider.alloc(1024).into_layout().unwrap();

    println!("Press CTRL-C to quit...");
    for idx in 0..u32::MAX {
        tokio::time::sleep(std::time::Duration::from_millis(33)).await;

        // Allocate particular SHM buffer using pre-created layout
        let mut sbuf = layout
            .alloc()
            .with_policy::<Deallocate<1, JustAlloc, JustAlloc, DeallocEldest>>()
            .wait()
            .unwrap();

        // We reserve a small space at the beginning of the buffer to include the iteration index
        // of the write. This is simply to have the same format as zn_pub.
        let prefix = format!("[{idx:4}] ");
        let prefix_len = prefix.as_bytes().len();
        let slice_len = prefix_len + payload.as_bytes().len();

        sbuf[0..prefix_len].copy_from_slice(prefix.as_bytes());
        sbuf[prefix_len..slice_len].copy_from_slice(payload.as_bytes());

        // Write the data
        println!(
            "Put SHM Data ('{}': '{}')",
            path,
            String::from_utf8_lossy(&sbuf[0..slice_len])
        );
        publisher.put(sbuf).await?;
    }

    Ok(())
}
