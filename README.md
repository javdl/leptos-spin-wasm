# my-leptos-app

Prequisites:

- Rust [with the `wasm32-wasip1` target](https://www.rust-lang.org/tools/install) - `rustup target add wasm32-wasip1`
- [Spin](https://developer.fermyon.com/spin/v3/install)
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos#getting-started) - `cargo install --locked --version 0.2.22 cargo-leptos`

Build and run:

- `nix develop`
- `spin up --build` to build and run the server. It will print the application URL.

Rendering markdown pages:

- To render a markdown page, send a POST request to the `/markdown` endpoint with the markdown content in the request body.
