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
Tries to connect to a server. On success:
- Sends a message to the server and
- Closes the socket.

### line_periodic_ping_client
Tries to connect to a server. On success:
- Waits 5 seconds
- Sends a message to the server every seconds.

Documentation
============================================
This is of course not an exhaustive list of the tokio documentation out there. It's
just a list of the documentation I can remember I read about tokio.
I probably forgot a lot, but I tend to read them from reddit on my way to work so I
never remember what were the sources I read from when I'm finally back at home and
all the chores are done.

If you have any additional documentation you want to put here, feel free to share a pull request.

Official documentation
--------------------------------------------
[Tokio tutorial](https://tokio.rs/docs/getting-started/hello-world/)

[Tokio official examples](https://github.com/tokio-rs/tokio/tree/master/examples)

[Tokio documentation](https://docs.rs/tokio/)

[Future documentation](https://docs.rs/futures/)

Blog articles
--------------------------------------------
[dev.to article](https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3) about future usage. Was a good lesson about what the futures are especially as an standalone component instead of a "tokio callback".

[Caffeinated Bitstream article](https://cafbit.com/post/tokio_internals/) explaining how tokio works on a lower level and what it does with the futures you are throwing at it. Especially interesting if you've already implemented your own scheduler on some project.

Other projects
--------------------------------------------
[mqtt-protocol lib example](https://github.com/zonyitoo/mqtt-rs/blob/master/examples/sub-client-async.rs)

[jgallagher chat client/server example](https://github.com/jgallagher/tokio-chat-example/blob/master/tokio-chat-client/src/main.rs)
