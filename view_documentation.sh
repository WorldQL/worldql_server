# Generate Documentation
cargo doc

# Start Webserver
echo "http://127.0.0.1:8000/worldql_server/index.html"
python3 -m http.server -b localhost -d ./target/doc
