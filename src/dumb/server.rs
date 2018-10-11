extern crate futures;
extern crate tokio;

use tokio::prelude::*;

use futures::{future, Future, Stream};

fn main() {
    let port = 12345;
    tokio::run(future::poll_fn(move || {
        let addr = format!("127.0.0.1:{}", port).parse().unwrap();
        let listener = tokio::net::TcpListener::bind(&addr).unwrap();
        tokio::spawn(
            listener
                .incoming()
                .for_each(|socket| {
                    println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

                    let stream = tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new());
                    tokio::spawn(
                        stream
                            .for_each(|msg| {
                                println!("Client said '{}'", msg);
                                future::ok(())
                            }).map_err(|err| {
                                println!("socket error = {:?}", err);
                            }),
                    );
                    future::ok(())
                }).map_err(|err| {
                    println!("accept error = {:?}", err);
                }),
        );

        println!("server running on localhost:{}", port);
        Ok(Async::Ready(()))
    }));
}
