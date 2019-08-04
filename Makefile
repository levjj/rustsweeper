target/wasm32-unknown-emscripten/debug/rustsweeper.js target/wasm32-unknown-emscripten/debug/rustsweeper.wasm: src/main.rs src/lib.rs src/model.rs
	cargo build --target=wasm32-unknown-emscripten

target/wasm32-unknown-emscripten/release/rustsweeper.js target/wasm32-unknown-emscripten/release/rustsweeper.wasm: src/main.rs src/lib.rs src/model.rs
	cargo build --target=wasm32-unknown-emscripten --release

debug: target/wasm32-unknown-emscripten/debug/rustsweeper.js target/wasm32-unknown-emscripten/debug/rustsweeper.wasm
	cp target/wasm32-unknown-emscripten/debug/rustsweeper.js site/rustsweeper.js
	cp target/wasm32-unknown-emscripten/debug/rustsweeper.wasm site/rustsweeper.wasm

release: target/wasm32-unknown-emscripten/release/rustsweeper.js target/wasm32-unknown-emscripten/release/rustsweeper.wasm
	cp target/wasm32-unknown-emscripten/release/rustsweeper.js site/rustsweeper.js
	cp target/wasm32-unknown-emscripten/release/rustsweeper.wasm site/rustsweeper.wasm

served: debug
	cd site && python -m SimpleHTTPServer

server: release
	cd site && python -m SimpleHTTPServer

clean:
	rm -rf target
