echo "building wasm"

wasm-pack build --target web

echo "builded..."
echo "starting server"

cargo run