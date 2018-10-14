tokio_examples
============================================

Just trying to make a few tokio usage examples to learn how to use the library.
Feel free to correct me if I'm wrong via pull requests I guess? (<= not used to open source collaboration)

Conventions
============================================

namespaces
--------------------------------------------
In all of my examples, I'll try to avoid to "use" submodules. In fact, as these
examples' goal is to help the reader (and myself) understand how to use tokio and I
believe having the full name of each element (struct of methods) might help
understand the structure/hierarchy of the future/tokio ecosystem. And I think that
knowledge might help getting comfortable using the library.

I'll therefore avoid using the crates preludes.

Examples
============================================
Every example can be launched via `cargo run --bin <example_name>`.

Based on the line codec
--------------------------------------------
Examples using the tokio::codec::LinesCodec.

### line_listening_server
Program that basically sits there and wait for clients. Then on any connected
client, it waits for messages that it prints in its stdout.

### line_sending_client
Tries to connect to a server. On success sends a message to the server and
closes the socket.
