extern crate futures;
extern crate tokio;

// Trait required for map_err
use futures::Future;

// Trait required for stream send method
use futures::sink::Sink;

fn main() {
    // Configuration
    let port = 12345;
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();

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
            .and_then(|socket| {
                // Creating a stream from our socket (see server for details)
                let stream = tokio::codec::Framed::new(socket, tokio::codec::LinesCodec::new());

                // Sending a hello message
                // -------------------------
                // This creates a future (that'll resolve when the data is sent
                // or on sending error) that I return at the end of this closure.
                // That way it is automatically launched by the scheduler.
                // Else I would have had to use tokio::spawn.
                stream.send("I am a client".to_string()).map_err(|err| {
                    // Processing errors (before and_then)
                    println!("Send error: {:?}", err);

                // On success.
                // --------------------------
                // Also note that you get a (mut) reference to your stream in the
                // closure. How handy ! :D
                }).and_then(|mut stream| {
                    println!("Successfully sent packet!");

                    // Close socket
                    match stream.close() {
                        Ok(_) => println!("Successfully closed the stream !"),
                        Err(e) => println!("Couldn't close the socket: {:?}", e),
                    };

                    // Return required next future (which does nothing successfully)
                    futures::future::ok(())
                })
            }),
    );
}
