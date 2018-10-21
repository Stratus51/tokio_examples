extern crate futures;
extern crate tokio;

// "Imports" -----------------------------------------------------------------------
// Trait required for map_err
use futures::Future;

// Trait required for fold on streams
use futures::Stream;

// Trait required for stream send method
use futures::sink::Sink;

// Standard time structures
use std::time::{Duration, Instant};

// Error management boiler plate ---------------------------------------------------
#[derive(Debug)] // For {:?} printability
enum PingerError {
    Socket(tokio::io::Error),
    Timer(tokio::timer::Error),
}

// PingerError converters from sub error types
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

// Pinger future (main process) ----------------------------------------------------
struct Pinger {
    stream: tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>,
    interval: tokio::timer::Interval,
}

impl futures::Future for Pinger {
    type Item = ();
    type Error = PingerError;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        // Check our socket to print the server messages (looping on the availible
        // lines).
        while let futures::Async::Ready(data) = self.stream.poll()? {
            // Stream convention: data will be Some on data and None on stream
            // ending
            match data {
                Some(line) => println!("Server said: {}", line),
                None => {
                    println!("Socket closed. Exiting.");

                    // Return Ready so that the scheduler kills us.
                    return Ok(futures::Async::Ready(()));
                }
            }
        }

        // Check the interval
        if let futures::Async::Ready(Some(_instant)) = self.interval.poll()? {
            println!("Sending ping!");

            // Adding a ping packet in the pending writes of our stream (not sent)
            self.stream.start_send("Ping".to_string())?;

            // Resubscribe to the interval notifications by triggering a "NotReady"
            // return value.
            //
            // In fact, if you skip this part, after the first interval trigger
            // your task won't be watching the interval events anymore. Calling a
            // polling that you know will fail will force the resubscription of
            // your task to the time events produced by self.interval.
            //
            // A more elegant way of doing that is removing that last line and using
            // a "while" instead of a "if" for this block (like the above stream
            // loop). But I thought this implementation was a good way to expose
            // the notification system.
            self.interval.poll()?;
        }
        // Try to flush the pending writes on our stream
        if let futures::Async::Ready(_) = self.stream.poll_complete()? {
            println!("Stream writes fully flushed");
        }

        // Return that we are not ready to die yet
        Ok(futures::Async::NotReady)
    }
}

// Main ----------------------------------------------------------------------------
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
                // Creating and returning a Pinger future (our main process)
                Pinger {
                    stream : tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new()),
                    interval : tokio::timer::Interval::new(ping_start, ping_period),
                }
                // Processing pinger errors (PingerError)
                .map_err(|err| {
                    println!("Pinger error: {:?}", err);
                })
            }),
    );
}
