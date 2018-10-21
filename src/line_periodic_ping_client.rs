extern crate futures;
extern crate tokio;

// Error management boiler plate
#[derive(Debug)] // For {:?} printability
enum PingerError {
    Socket(tokio::io::Error),
    Timer(tokio::timer::Error),
}

impl std::convert::From<tokio::timer::Error> for PingerError {
    fn from(data: tokio::timer::Error) -> Self {
        PingerError::Timer(data)
    }
}

impl std::convert::From<tokio::io::Error> for PingerError {
    fn from(data: tokio::io::Error) -> Self {
        PingerError::Socket(data)
    }
}

// Trait required for map_err
use futures::Future;

// Trait required for fold on streams
use futures::Stream;

// Trait required for stream send method
use futures::sink::Sink;

// Standard time structures
use std::time::{Duration, Instant};

struct Pinger {
    stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>,
    interval: tokio::timer::Interval,
}

impl futures::Future for Pinger {
    type Item = ();
    type Error = PingerError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        while let futures::Async::Ready(data) = self.stream.poll()? {
            match data {
                Some(line) => println!("Server said: {}", line),
                None => {
                    println!("Socket closed. Exiting.");
                    return Ok(futures::Async::Ready(()));
                }
            }
        }

        while let futures::Async::Ready(Some(_instant)) = self.interval.poll()? {
            println!("Sending ping!");
            self.stream.start_send("Ping".to_string())?;
        }
        self.stream.poll_complete()?;

        Ok(futures::Async::NotReady)
    }
}

fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let ping_period = Duration::from_secs(1);
    let ping_start = Instant::now() + Duration::from_secs(5);

    // Launching scheduler
    tokio::run(
        // Creating a ConnectFuture (attempt to connect)
        tokio::net::TcpStream::connect(&addr)

            // Processing connect error.
            // -------------------------
            // Has to be before and_then or map_err will then process and_then
            // errors. It might seem obvious to many, but I lost 10 mins thinking
            // It was about a type issue so I prefer underlining it.
            .map_err(|err| {
                println!("Connect error: {:?}", err);
            })

            // Processing connect success
            // --------------------------
            // That is what and_then (from Future trait) is for
            .and_then(move |socket| {
                Pinger {
                    stream : tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new()),
                    interval : tokio::timer::Interval::new(ping_start, ping_period),
                }
                .map_err(|err| {
                    println!("Pinger error: {:?}", err);
                })
            }),
    );
}
