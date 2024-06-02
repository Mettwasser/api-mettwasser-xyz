# Starts autobuild with cargo and tailwind
# Cargo and tailwind cli are required to run this powershell script successfully

Write-Host "Dev server started at http://127.0.0.1:3000" -ForegroundColor yellow
Start-Job -Name "amx-cw" -ScriptBlock {
    cargo watch -w . -x run
}
Start-Job -Name "amx-tw" -ScriptBlock {
    tailwindcss -c ./tailwind.config.js -i ./assets/styles/index.css -o ./build/index.css --watch=always
}
