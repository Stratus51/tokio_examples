extern crate futures;
extern crate tokio;

use futures::{future, Future, Stream};

fn main() {
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(&addr).unwrap();

    tokio::run(
        listener
            .incoming()
            .for_each(|socket| {
                println!("New client from {:?}", socket.peer_addr().unwrap());

                let stream = tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new());
                tokio::spawn(
                    stream
                        .for_each(|msg| {
                            println!("Client said: '{}'", msg);
                            future::ok(())
                        }).map_err(|err| {
                            println!("Socket error: {:?}", err);
                        }),
                );
                future::ok(())
            }).map_err(|err| {
                println!("Accept error: {:?}", err);
            }),
    );
}
