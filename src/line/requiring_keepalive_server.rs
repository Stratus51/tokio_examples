extern crate futures;
extern crate tokio;

// "Imports" -----------------------------------------------------------------------
// Trait required for poll on streams
use futures::Stream;

// We need this trait declaration to have the map_err on our futures
use futures::Future;

// Trait required for start_send and poll_complete method on our line stream
use futures::sink::Sink;

// Standard time structures
use std::time::{Duration, Instant};

// ClientError boiler plate --------------------------------------------------------
#[derive(Debug)] // For {:?} printability
enum ClientError {
    Socket(tokio::io::Error),
    Timer(tokio::timer::Error),
}

// PingerError converters from sub error types
impl std::convert::From<tokio::timer::Error> for ClientError {
    fn from(data: tokio::timer::Error) -> Self {
        ClientError::Timer(data)
    }
}

impl std::convert::From<tokio::io::Error> for ClientError {
    fn from(data: tokio::io::Error) -> Self {
        ClientError::Socket(data)
    }
}

// ClientGreetings (future sending a greeting) -------------------------------------
struct ClientGreetings {
    // This stream is an option so that we can move it when we are ready
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
    type Error = ClientError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        // Decapsulate stream
        match self.stream {
            Some(ref mut stream) => {
                // If we haven't started sending the greeting
                if !self.is_sending {
                    self.is_sending = true;
                    println!("Sending greetings...");
                    stream.start_send("Hello".to_string())?;
                }

                // If the stream write could not be flushed yet
                if let futures::Async::NotReady = stream.poll_complete()? {
                    // TODO Check if the socket was closed before the greeting was
                    // sent

                    // Return that we are still waiting for the flush
                    return Ok(futures::Async::NotReady);
                }
            },

            // Not supposed to happen
            None => panic!("No ClientGreetings doesn't have a stream anymore!"),
        }

        // We're done, send the ready signal and pass the stream
        println!("Greetings sent.");
        match self.stream.take() {
            Some(stream) => Ok(futures::Async::Ready(stream)),

            // Not supposed to happen
            None => panic!("No ClientGreetings doesn't have a stream anymore!"),
        }
    }
}

// ClientMaintenance (printing client messages) ------------------------------------
struct ClientMaintenance {
    stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>,
    keepalive: u64,
    keepalive_delay: tokio::timer::Delay,
}

impl ClientMaintenance {
    fn new(stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>, keepalive: u64) -> Self {
        Self {
            stream: stream,
            keepalive: keepalive,
            keepalive_delay: tokio::timer::Delay::new(Instant::now() + Duration::from_secs(keepalive)),
        }
    }
}

impl futures::Future for ClientMaintenance {
    type Item = ();
    type Error = ClientError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        // While there are buffered lines
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

            // Refresh keepalive deadline on message reception
            self.keepalive_delay.reset(Instant::now() + Duration::from_secs(self.keepalive));
        }

        // Keepalive check
        if let futures::Async::Ready(_) = self.keepalive_delay.poll()? {
            // Keepalive reached
            println!("Client connection expired. Closing socket.");

            // Return Ready so that the scheduler kills us.
            return Ok(futures::Async::Ready(()));
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
    let keepalive = 10;

    // Creating a TCP socket listener
    let listener = tokio::net::TcpListener::bind(&addr).unwrap();

    // Launching scheduler
    tokio::run(
        // Using the listener incoming socket callback as Future to run.
        listener
            .incoming() // This is a Stream
            .for_each(move |socket| {
                println!("New client connected!");
                let keepalive = keepalive;

                // Build line stream from socket
                let stream = tokio::codec::Framed::new(
                    socket,
                    tokio::codec::LinesCodec::new(),
                );

                // Spawn new task to take care of the socket
                tokio::spawn(
                    // Give the stream to a future that will greet it
                    ClientGreetings::new(stream)
                        .and_then(move |stream| {
                            // And then forward the stream to a future that will
                            // maintain the connection
                            ClientMaintenance::new(
                                stream,
                                keepalive,
                            )
                        // And always manage errors
                        }).map_err(|err| {
                            println!("Client error: {:?}", err);
                        })
                );

                futures::future::ok(())

            // Same as tokio::spawn, tokio::run can't handle errors for you, so you
            // have to process them yourself.
            }).map_err(|err| {
                println!("Accept error: {:?}", err);
            }),
    );
}
