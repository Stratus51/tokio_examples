extern crate futures;
extern crate tokio;

// Trait required for map_err
use futures::Future;

// Trait required for fold on streams
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

                // Create our interval (periodic timer)
                // ------------------------------------
                // This interval will start at the time ping_start and then trigger
                // every ping_period.
                // It is also the return item of our closure which means it will
                // be spawned by the scheduler automatically
                tokio::timer::Interval::new(ping_start, ping_period)
                    // Catch interval error
                    .map_err(|err| {
                        println!("Couldn't start the timer: {:?}", err);
                    })
                    // TODO this is a terrible hack totally unusable in practice.
                    // ... Working on the simplest fix I can find. Right now the
                    // only option I see is the one implemented in the official
                    // tokio chat example. But it's a bit cumbersome so I'm digging
                    // just in case there a simple correct way for this example.
                    //
                    // Okay, I needed to insert the stream somehow in the interval
                    // closure, which was complicated as I could not just move it in
                    // because of closure type conflict (something about FnOnce and
                    // FnMut).
                    //
                    // Therefore, I ended up copying a trick I found on mqtt-protocol.
                    //
                    // I used the method fold that usually means on an iterator:
                    // Start with a value and modify it with each item from the
                    // iterator. Therefore on each new item, the closure will be passed
                    // the current value and will have to return the new one.
                    //
                    // When the iterator is exhausted, the fold method will return the
                    // last value.
                    //
                    // Here this is exactly what we do: we give the initial value to
                    // fold (our stream), then it will be passed as a mutable to our
                    // closure to be modified and at the end we have to return the
                    // result. We return our stream unchanged so that it can be passed
                    // again to us as the current value on the next turn.
                    //
                    // We're not really folding the values into one... Or if we
                    // consider a stream filled with the wanted packets a result then
                    // I guess we are folding something.
                    // (Well I'm not even using the interval value but whatever)
                    .fold(stream, |stream, _interval| {
                        // Sending a ping message
                        stream.send("Ping".to_string())
                        // Processing send errors
                        .map_err(|err| {
                            println!("Send error: {:?}", err);
                        // On success print
                        }).and_then(|stream| {
                            println!("Successfully pinged!");

                            // Return required next future containing the fold result
                            futures::future::ok(stream)
                        })
                    // The "final" future result has to be Result<(), ()> for
                    // tokio::run to be execute successfully. Therefore we just drop
                    // the folded value (our stream) to match the wanted type.
                    //
                    // We could have closed the stream. But you know, it'll be closed
                    // anyway when droping... I assume ... :p
                    }).map(|_stream| {})
            }),
    );
}
