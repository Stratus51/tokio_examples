extern crate futures;
extern crate tokio;

use futures::{Future};
use futures::sink::Sink;

fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();

    // Launching scheduler
    tokio::run(
        // Creating a ConnectFuture (attempt to connect)
        tokio::net::TcpStream::connect(&addr)
            .map_err(|err| {
                println!("Connect error: {:?}", err);
            })
            // Processing connect success: that is what and_then (from Future trait) is for
            .and_then(|socket| {
                // Creating a stream from our socket (see server for details)
                let stream = tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new());

                stream.send("I am a client".to_string()).map_err(|err| {
                    println!("Send error: {:?}", err);
                }).and_then(|mut stream| {
                    println!("Successfully sent packet!");
                    match stream.close() {
                        Ok(_) => println!("Successfully closed the stream !"),
                        Err(e) => println!("Couldn't close the socket: {:?}", e),
                    };
                    futures::future::ok(())
                })

            // Processing connect error
            }),
    );
}
