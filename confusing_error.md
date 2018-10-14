Confusing errors:
============================================================================
fn program(port: u16) -> impl Future<Item = (), Error = tokio::io::Error>
----------------------------------------------------------------------------
error[E0271]: type mismatch resolving `<impl futures::Future as futures::Future>::Error == ()`
  --> src/main.rs:52:5
   |
52 |     tokio::run(program(port));
   |     ^^^^^^^^^^ expected struct `std::io::Error`, found ()
   |
   = note: expected type `std::io::Error`
              found type `()`
   = note: required by `tokio::run`

Confusing: looks like tokio::run wants std::io::Error but found (), when it is the opposite.

There are more like this, the first one is the most confusing. Unfortunately, you get used to it ...

stream.send(String).then(|| {
----------------------------------------------------------------------------
error[E0282]: type annotations needed
  --> src/dumb/client.rs:29:21
   |
29 |                     futures::future::ok(())
   |                     ^^^^^^^^^^^^^^^^^^^ cannot infer type for `E`

error: aborting due to previous error

I guessed E was the Error type of futures::future::ok, but when starting tokio programming
that might not be obvious... Or I might lack Rust experience, that's entirely possible too :p
