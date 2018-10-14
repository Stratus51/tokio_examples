extern crate futures;
extern crate tokio;

// Trait required for map_err
use futures::Future;

// Trait required for for_each on streams
use futures::Stream;

// Trait required for stream send method
use futures::sink::Sink;

// Standard time structures
use std::time::{Duration, Instant};

fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let ping_period = Duration::from_secs(1);
    let ping_start_offset = 5;
    let ping_start = Instant::now() + Duration::from_secs(ping_start_offset);

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
                // Creating a stream from our socket (see server for details)
                let stream = tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new());

                println!("Waiting for {}s", ping_start_offset);
                tokio::timer::Interval::new(ping_start, ping_period)
                    .map_err(|err| {
                        println!("Couldn't start the timer: {:?}", err);
                    })
                    .fold(stream, |stream, _interval| {
                        // Sending a hello message
                        stream.send("Ping".to_string())
                        // Processing errors (before and_then)
                        .map_err(|err| {
                            println!("Send error: {:?}", err);
                        // On success print
                        }).and_then(|stream| {
                            println!("Successfully pinged!");

                            // Return required next future (which does nothing successfully)
                            futures::future::ok(stream)
                        })
                    }).map(|_| {})
            }),
    );
}
