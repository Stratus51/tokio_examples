extern crate bincode;
extern crate bytes;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;

// We need this trait declaration to have the for_each method auto implementation
// on our streams
use futures::Stream;

// We need this trait declaration to have the map_err on our futures
use futures::Future;

mod codec;

// Usage:
// You can telnet that server (telnet 127.0.0.1 12345) and send it messages
fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();

    // Creating a TCP socket listener
    let listener = tokio::net::TcpListener::bind(&addr).unwrap();

    // Launching scheduler
    tokio::run(
        // Using the listener incoming socket callback as Future to run.
        listener
            .incoming() // This is a Stream
            .for_each(|socket| {
                // Save the name of the peer (address)
                let name = socket.peer_addr().unwrap();

                // New client processing
                println!("New client from {} opened a socket.", name);

                // Creating a Stream from the socket (because a TcpStream ain't a
                // stream)
                //
                // I could have used the BytesCodec, but hey, if we're going to be
                // using a codec anyway, let's get comfy and use one that returns
                // strings :D
                let stream = tokio::codec::Framed::new(socket, codec::Codec::new());

                // Spawning a future (~callback in our case) to process each
                // stream new items
                tokio::spawn(
                    stream
                        .for_each(move |msg| {
                            // Just print the message ...
                            println!("{} sent '{:?}'", name, msg);

                            // Return a future for some reason?
                            // Apparently there wont be any other item fed to our
                            // closure until this future is completed.
                            futures::future::ok(())

                        // Error catching because tokio spawn doesn't know how to
                        // handle any non "()" error. (Quite logical indeed, but
                        // the associated error is quite confusing because it looks
                        // like the two types are swapped. See notes confusing
                        // errors)
                        }).map_err(move |err| {
                            println!("{} had a socket error: {:?}", name, err);
                        // Signal socket departure
                        }).then(move |_| {
                            println!("{} socket is disconnected", name);
                            // We have to return a future
                            futures::future::ok(())
                        }),
                );

                // Same: for_each asks to return a future to further process
                // the current item and will freeze the stream item flow until
                // then.
                futures::future::ok(())

            // Same as tokio::spawn, tokio::run can't handle errors for you, so you
            // have to process them yourself.
            }).map_err(|err| {
                println!("Accept error: {:?}", err);
            }),
    );
}
