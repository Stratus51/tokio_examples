extern crate futures;
extern crate tokio;

use futures::{future, Future, Stream};

fn main() {
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    tokio::run(
        tokio::net::TcpStream::connect(&addr)
            .and_then(|socket| {
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
                println!("connect error = {:?}", err);
            }),
    );
}
