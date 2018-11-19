extern crate futures;
extern crate tokio;
extern crate bytes;
extern crate bincode;
#[macro_use]
extern crate serde_derive;

// "Imports" -----------------------------------------------------------------------
// Trait required for poll on streams
use futures::Stream;

// We need this trait declaration to have the map_err on our futures
use futures::Future;

// Our custom codec
mod chat_server_mod;
use chat_server_mod::client_manager;
mod codec;

// TODO Implement the server chat algorithm (message spreading)

// Usage:
// You can telnet that server (telnet 127.0.0.1 12345) and send it messages
fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let connect_timeout = 5;
    let keepalive = 10;

    // Creating a TCP socket listener
    let listener = tokio::net::TcpListener::bind(&addr).unwrap();

    // Launching scheduler
    tokio::run(
        // TODO This code is just some temporary code to test the client_manager
        // Using the listener incoming socket callback as Future to run.
        listener
            .incoming() // This is a Stream
            .for_each(move |socket| {
                println!("New client connected!");

                // Build line stream from socket
                let stream = tokio::codec::Framed::new(
                    socket,
                    codec::Codec::new(),
                );

                // Spawn new task to take care of the socket
                let client_manager = client_manager::new(stream, connect_timeout, keepalive)
                    .map_err(|err| {
                        println!("Client error: {:?}", err);
                    });
                tokio::spawn(client_manager);

                futures::future::ok(())

            // Same as tokio::spawn, tokio::run can't handle errors for you, so you
            // have to process them yourself.
            }).map_err(|err| {
                println!("Accept error: {:?}", err);
            }),
    );
}
