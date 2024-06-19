# Zenoh pub sub issue

changes from original [z_pub_shm.rs](https://github.com/eclipse-zenoh/zenoh/blob/39ea5e50eecede3c61da0afdcdc1d489e5383efd/examples/examples/z_pub_shm.rs):

- allocation policy changed from `BlockOn<GarbageCollect>` to `Deallocate<1, JustAlloc, JustAlloc, DeallocEldest>`
- default config with mode set to peer and shared memory enabled.

changes from original [z_sub_shm.rs](https://github.com/eclipse-zenoh/zenoh/blob/39ea5e50eecede3c61da0afdcdc1d489e5383efd/examples/examples/z_pub_shm.rs):

- default config with mode set to peer and shared memory enabled.
- `tokio::time::sleep` added (only for [z_sub_shm_slow.rs](src/bin/z_sub_shm_slow.rs))

## Results:

### Case 1
Running [z_pub_shm.rs](src/bin/z_pub_shm.rs) and stopping it:

```
$ cargo run --bin z_pub_shm     
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/z_pub_shm`
Opening session...
Creating POSIX SHM provider...
Allocating Shared Memory Buffer...
Press CTRL-C to quit...
Put SHM Data ('demo/example/zenoh-rs-pub': '[   0] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   1] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   2] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   3] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   4] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   5] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   6] Hello')
.
.
.
Put SHM Data ('demo/example/zenoh-rs-pub': '[  23] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  24] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  25] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  26] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  27] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  28] Hello')
^C
$ 
```

Running [z_sub_shm.rs](src/bin/z_sub_shm.rs):

```
$ cargo run --bin z_sub_shm
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/z_sub_shm`
Opening session...
Declaring Subscriber on 'demo/example/zenoh-rs-pub'...
Press CTRL-C to quit...
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   0] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   1] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   2] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   3] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   4] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   5] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   6] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   7] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   8] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[   9] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  10] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  11] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  12] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  13] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  14] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  15] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  16] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  17] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  18] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  19] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  20] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  21] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  22] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  23] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  24] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  25] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  26] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  27] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[  28] Hello')

```

the subscriber stops, waiting for data from the publisher. each data point is received once.

### Case 2:

Running [z_pub_shm.rs](src/bin/z_pub_shm.rs) and stopping it:

```
nyohhira mr/zenoh-test ‹master*› » cargo run --bin z_pub_shm
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/z_pub_shm`
Opening session...
Creating POSIX SHM provider...
Allocating Shared Memory Buffer...
Press CTRL-C to quit...
Put SHM Data ('demo/example/zenoh-rs-pub': '[   0] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   1] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   2] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   3] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   4] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   5] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   6] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   7] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   8] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[   9] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  10] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  11] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  12] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[  13] Hello')
...
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 165] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 166] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 167] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 168] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 169] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 170] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 171] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 172] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 173] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 174] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 175] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 176] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 177] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 178] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 179] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 180] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 181] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 182] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 183] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 184] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 185] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 186] Hello')
Put SHM Data ('demo/example/zenoh-rs-pub': '[ 187] Hello')
^C
nyohhira mr/zenoh-test ‹master*› »   
```

Running [z_sub_shm_slow.rs](src/bin/z_sub_shm_slow.rs):

```
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 117] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 118] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 119] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 130] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 131] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 142] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 143] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 154] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 155] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 166] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 167] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 178] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 179] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 180] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 181] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 182] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 183] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 184] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 185] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 186] Hello')
>> [Subscriber] Received PUT ('demo/example/zenoh-rs-pub': '[ 187] Hello')
```

data with index 178 to 187 is received multiple times.
