echo "building wasm"

cargo install wasm-pack
wasm-pack build --target web --out-dir ./static/script

echo "builded..."
echo "starting server"

cargo run