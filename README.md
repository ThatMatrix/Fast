# Fast

## Description 

Fast is a simple web server made in Rust.

The project originally started as with the codecrafter Webserver tutorial: ["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).
However, it is fairly easy to finish and as such, after a week of non-consecutive work, it became apparent that I should make the project into something bigger and separated to add more functionnalities.

## Functionnalities

- Support of [HTTP/1.1](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol)

## Compile & run the server

1. Ensure you have `cargo (1.70)` installed locally
2. Run `./your_server.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
