& wasm-pack build --target web --release

cp ./pkg/placeplace.js ./docs/placeplace.js
cp ./pkg/placeplace_bg.wasm ./docs/placeplace_bg.wasm
