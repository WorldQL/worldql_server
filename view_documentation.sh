cargo doc
echo "http://127.0.0.1:8000/worldql_server/index.html"
cd target/doc && python3 -m http.server -b localhost

