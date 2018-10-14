Confusing errors:
============================================================================
fn program(port: u16) -> impl Future<Item = (), Error = tokio::io::Error>
----------------------------------------------------------------------------
```
error[E0271]: type mismatch resolving `<impl futures::Future as futures::Future>::Error == ()`
  --> src/main.rs:52:5
   |
52 |     tokio::run(program(port));
   |     ^^^^^^^^^^ expected struct `std::io::Error`, found ()
   |
   = note: expected type `std::io::Error`
              found type `()`
   = note: required by `tokio::run`
```

Confusing: looks like tokio::run wants std::io::Error but found (), when it is the opposite.

There are more like this, the first one is the most confusing. Unfortunately, you get used to it ...

stream.send(String).then(|| {
----------------------------------------------------------------------------
```
error[E0282]: type annotations needed
  --> src/dumb/client.rs:29:21
   |
29 |                     futures::future::ok(())
   |                     ^^^^^^^^^^^^^^^^^^^ cannot infer type for `E`

error: aborting due to previous error
```

I guessed E was the Error type of futures::future::ok, but when starting tokio programming
that might not be obvious... Or I might lack Rust experience, that's entirely possible too :p


tokio::net::TcpStream::connect(&addr).and_then(move |socket| {
----------------------------------------------------------------------------
```
error[E0277]: the trait bound `futures::stream::AndThen<futures::stream::MapErr<tokio::timer::Interval, [closure@src/line_periodic_ping_client.rs:46:30: 48:22]>, [closure@src/line_periodic_ping_client.rs:49:31: 61:18 stream:_], futures::AndThen<futures::MapErr<futures::sink::Send<tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>>, [closure@src/line_periodic_ping_client.rs:51:70: 55:22]>, futures::FutureResult<(), ()>, [closure@src/line_periodic_ping_client.rs:55:33: 60:22]>>: futures::Future` is not satisfied
  --> src/line_periodic_ping_client.rs:41:14
   |
41 |             .and_then(move |socket| {
   |              ^^^^^^^^ the trait `futures::Future` is not implemented for `futures::stream::AndThen<futures::stream::MapErr<tokio::timer::Interval, [closure@src/line_periodic_ping_client.rs:46:30: 48:22]>, [closure@src/line_periodic_ping_client.rs:49:31: 61:18 stream:_], futures::AndThen<futures::MapErr<futures::sink::Send<tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>>, [closure@src/line_periodic_ping_client.rs:51:70: 55:22]>, futures::FutureResult<(), ()>, [closure@src/line_periodic_ping_client.rs:55:33: 60:22]>>`
   |
   = note: required because of the requirements on the impl of `futures::IntoFuture` for `futures::stream::AndThen<futures::stream::MapErr<tokio::timer::Interval, [closure@src/line_periodic_ping_client.rs:46:30: 48:22]>, [closure@src/line_periodic_ping_client.rs:49:31: 61:18 stream:_], futures::AndThen<futures::MapErr<futures::sink::Send<tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>>, [closure@src/line_periodic_ping_client.rs:51:70: 55:22]>, futures::FutureResult<(), ()>, [closure@src/line_periodic_ping_client.rs:55:33: 60:22]>>`
```

My closure, as stated in this log returns an AndThen Future which I would have thought implements
futures::Future ... But it seems either I'm wrong or it is hidding a type mismatch error.

I didn't resolve that one. While looking for a solution, I changed the structure of my code, so no
answer for that one. Sorry.

tokio::timer::Interval::new().fold(`<Stream>`, |stream, interval| {
----------------------------------------------------------------------------
```
error[E0271]: type mismatch resolving `<futures::AndThen<futures::MapErr<futures::sink::Send<tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>>, [closure@src/line_periodic_ping_client.rs:53:34: 56:26]>, futures::FutureResult<(), ()>, [closure@src/line_periodic_ping_client.rs:56:37: 61:26]> as futures::IntoFuture>::Item == tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>`
  --> src/line_periodic_ping_client.rs:49:22
   |
49 |                     .fold(stream, |stream, timer| {
   |                      ^^^^ expected (), found struct `tokio::codec::Framed`
   |
   = note: expected type `()`
              found type `tokio::codec::Framed<tokio::net::TcpStream, tokio::codec::LinesCodec>`
```

That one was annoying. If you read the prototype, you'll see that you need your closure to return
a future that contains the same type of object as the first `fold` parameter. In other words, what
I did was make sure my closure returned `futures::future::ok(stream)`.
See the line_periodic_ping_client code for more informations on that.
