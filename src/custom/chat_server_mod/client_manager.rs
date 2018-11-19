// Tokio
use tokio;

// Trait required for poll on streams
use futures::Stream;

// We need this trait declaration to have the map_err on our futures
use futures::Future;

// Trait required for start_send and poll_complete method on our line stream
use futures::sink::Sink;

// Standard time structures
use std::time::{Duration, Instant};

// Our custom codec
use codec;

// ClientError boiler plate --------------------------------------------------------
#[derive(Debug)] // For {:?} printability
pub struct NamedClientError {
    name: Option<String>,
    error: ClientError,
}

#[derive(Debug)] // For {:?} printability
pub enum ClientError {
    Socket(tokio::io::Error),
    Timer(tokio::timer::Error),
    Unauthorized,
    BadConnectPacket(codec::Packet),
    ConnectTimeout,
    Keepalive,
    DisconnectBeforeConnectPacket,
}

// PingerError converters from sub error types
impl std::convert::From<tokio::timer::Error> for NamedClientError {
    fn from(data: tokio::timer::Error) -> Self {
        NamedClientError {
            name: None,
            error: ClientError::Timer(data)
        }
    }
}

impl std::convert::From<tokio::io::Error> for NamedClientError {
    fn from(data: tokio::io::Error) -> Self {
        NamedClientError {
            name: None,
            error: ClientError::Socket(data),
        }
    }
}

// ClientManager -------------------------------------------------------------------
pub fn new(stream: tokio::codec::Framed<tokio::net::TcpStream, codec::Codec>, connect_expire: u64, keepalive: u64) -> impl Future<Item = (), Error = NamedClientError> {
    ClientGreeter::new(stream, connect_expire, keepalive)
        .and_then(|maintainer| maintainer)
}

// ClientGreeter -------------------------------------------------------------------
struct ClientGreeter {
    // This stream is an option so that we can move it when we are ready
    stream: Option<tokio::codec::Framed<tokio::net::TcpStream, codec::Codec>>,
    expire_delay: tokio::timer::Delay,

    // For client maintainer
    keepalive: u64,
}

impl ClientGreeter {
    fn new(stream: tokio::codec::Framed<tokio::net::TcpStream, codec::Codec>, expire: u64, keepalive: u64) -> Self {
        Self {
            stream: Some(stream),
            expire_delay: tokio::timer::Delay::new(
                Instant::now() + Duration::from_secs(expire),
            ),
            keepalive,
        }
    }
}

impl futures::Future for ClientGreeter {
    type Item = ClientMaintainer;
    type Error = NamedClientError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        // Checking for new packet
        let packet = match self.stream.as_mut().unwrap().poll()? {
            // On stream event
            futures::Async::Ready(data) => match data {
                // On packet reception
                Some(packet) => packet,
                // On socket close
                None => return Err(NamedClientError {
                    name: None,
                    error: ClientError::DisconnectBeforeConnectPacket,
                }),
            },
            // If no packet was received yet
            futures::Async::NotReady => {
                // If connect delay expired
                if let futures::Async::Ready(_) = self.expire_delay.poll()? {
                    // Keepalive reached
                    println!("Client connection expired. Closing socket.");

                    // Return Ready so that the scheduler kills us.
                    return Err(NamedClientError {
                        name: None,
                        error: ClientError::ConnectTimeout,
                    });
                // If connect delay expired
                } else {
                    return Ok(futures::Async::NotReady);
                }
            },
        };

        // On valid packet
        let mut client_maintainer = match packet {
            codec::Packet::Connect { name } => {
                match self.stream.take() {
                    Some(stream) => ClientMaintainer::new(
                        stream,
                        name,
                        self.keepalive,
                    ),

                    // Not supposed to happen
                    None => panic!("We don't have a stream anymore!"),
                }
            }
            unexpected => return Err(NamedClientError {
                name: None,
                error: ClientError::BadConnectPacket(unexpected),
            }),
        };

        // Banning trolls from our server
        if client_maintainer.name == "troll" {
            client_maintainer.stream.start_send(codec::Packet::ConnectAck { accepted: false })?;
            Err(NamedClientError {
                name: Some(client_maintainer.name),
                error: ClientError::Unauthorized,
            })
        // Connection successful
        } else {
            println!("{} is now connected!", client_maintainer.name);
            match client_maintainer.stream.start_send(codec::Packet::ConnectAck { accepted: true }) {
                Ok(_) => (),
                Err(error) => return Err(NamedClientError {
                    name: Some(client_maintainer.name),
                    error: ClientError::Socket(error),
                }),
            };
            Ok(futures::Async::Ready(client_maintainer))
        }
    }
}

// ClientMaintainer ----------------------------------------------------------------
struct ClientMaintainer {
    stream: tokio::codec::Framed<tokio::net::TcpStream, codec::Codec>,
    name: String,
    keepalive: u64,
    keepalive_delay: tokio::timer::Delay,
}

impl ClientMaintainer {
    fn new(
        stream: tokio::codec::Framed<tokio::net::TcpStream, codec::Codec>,
        name: String,
        keepalive: u64,
    ) -> Self {
        Self {
            stream,
            name,
            keepalive,
            keepalive_delay: tokio::timer::Delay::new(
                Instant::now() + Duration::from_secs(keepalive),
            ),
        }
    }
}

impl futures::Future for ClientMaintainer {
    type Item = ();
    type Error = NamedClientError;
    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        // Check our socket to print the server messages (looping on the available lines).
        //
        // TODO Implement error context management to save the name of the client in the error.
        //  Doing it by hand implies some terrible boiler plate, and I still need to read about the
        //  current practices around error handling in Rust to do something clean
        while let futures::Async::Ready(data) = self.stream.poll()? {
            match data {
                // On packet
                //
                // TODO Send the message to the central server future for broadcast.
                Some(packet) => println!("{}: {:?}", self.name, packet),
                // On socket close
                None => {
                    println!("{}: Socket closed.", self.name);
                    return Ok(futures::Async::Ready(()));
                }
            }
        }

        // TODO Add keepalive management
        if let futures::Async::Ready(_) = self.keepalive_delay.poll()? {
            return Err(NamedClientError {
                name: Some(self.name.clone()),
                error: ClientError::Keepalive,
            });
        }

        // Wait for next event
        Ok(futures::Async::NotReady)
    }
}
