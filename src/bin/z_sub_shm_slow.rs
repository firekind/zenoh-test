use zenoh::{prelude::*, shm::zshm};

#[tokio::main]
async fn main() {
    // Initiate logging
    zenoh::try_init_log_from_env();

    let mut config = zenoh::Config::default();
    config.set_mode(Some(zenoh::config::WhatAmI::Peer)).unwrap();
    config.transport.shared_memory.set_enabled(true).unwrap();
    let key_expr = "demo/example/zenoh-rs-pub".to_owned();

    // A probing procedure for shared memory is performed upon session opening. To enable `z_pub_shm` to operate
    // over shared memory (and to not fallback on network mode), shared memory needs to be enabled also on the
    // subscriber side. By doing so, the probing procedure will succeed and shared memory will operate as expected.
    config.transport.shared_memory.set_enabled(true).unwrap();

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Declaring Subscriber on '{}'...", &key_expr);
    let subscriber = session.declare_subscriber(&key_expr).await.unwrap();

    println!("Press CTRL-C to quit...");
    while let Ok(sample) = subscriber.recv_async().await {
        print!(
            ">> [Subscriber] Received {} ('{}': ",
            sample.kind(),
            sample.key_expr().as_str(),
        );
        match sample.payload().deserialize::<&zshm>() {
            Ok(payload) => print!("'{}'", String::from_utf8_lossy(payload)),
            Err(e) => print!("'Not a ShmBufInner: {:?}'", e),
        }
        println!(")");
        tokio::time::sleep(std::time::Duration::from_millis(200)).await
    }
}
