extern crate tokio;
#[macro_use]
extern crate futures;
extern crate bytes;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use futures::{future, Future, Stream};
use futures::sync::mpsc;
use bytes::{BytesMut, Bytes, BufMut};

type Tx = mpsc::UnboundedSender<Bytes>;
type Rx = mpsc::UnboundedReceiver<Bytes>;

struct ByteCodec {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}
impl ByteCodec {
    fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }
    fn buffer(&mut self, line: &[u8]) {
        // Push the line onto the end of the write buffer.
        //
        // The `put` function is from the `BufMut` trait.
        self.wr.put(line);
    }

    fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to write some bytes to the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer.
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Result<Async<()>, io::Error> {
        loop {
            // Ensure the read buffer has capacity.
            //
            // This might result in an internal allocation.
            self.rd.reserve(1024);

            // Read data into the buffer.
            //
            // The `read_buf` fn is provided by `AsyncRead`.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for ByteCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        let sock_closed = self.fill_read_buf()?.is_ready();

        let len = self.rd.len();
        if len > 0 {
            return Ok(Async::Ready(Some(self.rd.split_to(len))));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

fn program(port: u16) -> impl Future<Item = (), Error = ()> {
    future::poll_fn(move || {
        // Start listening on the socket
        let addr = format!("127.0.0.1:{}", port).parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();
        let client_acceptor = listener.incoming().for_each(|socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let stream = ByteCodec::new(socket);
            tokio::spawn(
                stream.for_each(|msg| {
                    match String::from_utf8(msg.to_vec()) {
                        Ok(s) => println!("{}", s),
                        Err(e) => println!("String::from_utf8 error: {:?}", e),
                    };
                    future::ok(())
                })
                .map_err(|err| {
                    println!("accept error = {:?}", err);
                })
            );
            future::ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });
        tokio::spawn(client_acceptor);
        println!("server running on localhost:{}", port);
        Ok(Async::Ready(()))
    })
}

fn main() {
    let port = 12345;
    tokio::run(program(port));
}
