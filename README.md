tokio_examples
============================================
Just trying to make a few tokio usage examples to learn how to use the library.

Examples
============================================
Every example can be launched via `cargo run --bin <example_name>`.

Based on the line codec
--------------------------------------------
Examples using the tokio::codec::LinesCodec.

### line_listening_server
Program that:
- Waits for clients. 
- Prints messages from each client into the console.

### line_sending_client
Tries to connect to a server. On success:
- Sends a message to the server
- Closes the socket

### line_periodic_ping_client
Tries to connect to a server. On success:
- Waits 5 seconds
- Sends a message to the server every seconds.

Documentation
============================================

Official documentation
--------------------------------------------
[Tokio tutorial](https://tokio.rs/docs/getting-started/hello-world/)

[Tokio official examples](https://github.com/tokio-rs/tokio/tree/master/examples)

[Tokio documentation](https://docs.rs/tokio/)

[Future documentation](https://docs.rs/futures/)

Blog articles
--------------------------------------------
["Rust futures: an uneducated, short and hopefully not boring tutorial"](https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3)

["Tokio internals: Understanding Rust's asynchronous I/O framework from the bottom up"](https://cafbit.com/post/tokio_internals/)

Other projects
--------------------------------------------
[mqtt-protocol lib example](https://github.com/zonyitoo/mqtt-rs/blob/master/examples/sub-client-async.rs)

[jgallagher chat client/server example](https://github.com/jgallagher/tokio-chat-example/blob/master/tokio-chat-client/src/main.rs)
