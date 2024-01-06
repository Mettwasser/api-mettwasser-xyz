# Starts server with cargo and tailwind
# Cargo and tailwind cli are required to run this ps1 script successfully

Write-Host "Dev server started at http://127.0.0.1:3000" -ForegroundColor yellow
cargo watch -w . -x run &
tailwindcss -c ./tailwind.config.js -i ./assets/styles/index.css -o ./build/index.css --watch
