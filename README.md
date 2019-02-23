# rust-webassembly-test
Playground for Rust &amp; Webassembly

# Description
Nope, nothing of interest here. This is just me playing with Webassembly via Rust.

# Some info
You need Rust from the nightly channel. Once you have it, download wasm target:

`rustup target add wasm32-unknown-unknown --toolchain nightly`

Then install the extremely useful tool for testing your application:

`cargo install cargo-web`

Then clone this repository, run `cargo web start` and connect your browser to `localhost:8000` to see what I have painted on the canvas :)
