extern crate futures;
extern crate tokio;

// We need this trait declaration to have the for_each method auto implementation
// on our streams
use futures::Stream;

// We need this trait declaration to have the map_err on our futures
use futures::Future;

// Trait required for stream send method
use futures::sink::Sink;

struct ClientGreetings {
    stream: Option<tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>>,
    is_sending: bool,
}

impl ClientGreetings {
    fn new(stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>) -> Self {
        Self {
            stream: Some(stream),
            is_sending: false,
        }
    }
}

impl futures::Future for ClientGreetings {
    type Item = tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>;
    type Error = tokio::io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        match self.stream {
            Some(ref mut stream) => {
                if !self.is_sending {
                    println!("Sending greetings...");
                    stream.start_send("Hello".to_string())?;
                }

                if let futures::Async::NotReady = stream.poll_complete()? {
                    self.is_sending = true;

                    // TODO Check if the socket was closed before the greeting was
                    // sent
                    return Ok(futures::Async::NotReady);
                }
            }
            None => panic!("No ClientGreetings doesn't have a stream anymore!"),
        }

        println!("Greetings sent.");
        match self.stream.take() {
            Some(stream) => Ok(futures::Async::Ready(stream)),
            None => panic!("No ClientGreetings doesn't have a stream anymore!"),
        }
    }
}

struct ClientMaintenance {
    stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>,
}

impl futures::Future for ClientMaintenance {
    type Item = ();
    type Error = tokio::io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        while let futures::Async::Ready(data) = self.stream.poll()? {
            // Stream convention: data will be Some on data and None on stream
            // ending
            match data {
                Some(line) => println!("Client said: {}", line),
                None => {
                    println!("Socket closed.");

                    // Return Ready so that the scheduler kills us.
                    return Ok(futures::Async::Ready(()));
                }
            }
        }

        // Return that we are not ready to die yet
        Ok(futures::Async::NotReady)
    }
}

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
                println!("New client connected!");
                ClientGreetings::new(tokio::codec::Framed::new(
                    socket,
                    tokio::codec::LinesCodec::new()
                )).and_then(|stream| {
                    ClientMaintenance {
                        stream: stream,
                    }
                })

            // Same as tokio::spawn, tokio::run can't handle errors for you, so you
            // have to process them yourself.
            }).map_err(|err| {
                println!("Accept error: {:?}", err);
            }),
    );
}
