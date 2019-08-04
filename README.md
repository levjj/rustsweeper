# Rustsweeper

Small example Rust project of a mine finding game with an HTML UI.

- [src/model.rs](src/model.rs) contains the game state and tests
- [src/lib.rs](src/lib.rs) contains a UI component for rendering the Game and reacting to events

The code is compiled to [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly)
and uses uses [yew](https://github.com/yewstack/yew)
and [stdweb](https://github.com/koute/stdweb) to render and interact with HTML.

The project is build and deployed with [Cargo Web](https://github.com/koute/cargo-web).

# Installing and running

- Install `rustup` (see https://rustup.rs/).
- Install Cargo Web with `cargo install cargo-web`.
- Run the tests with `cargo test`.
- Start a live server with `cargo web start`.
- Navigate to http://localhost:8000/.

