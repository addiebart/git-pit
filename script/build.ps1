if !(Get-Command "git" -ErrorAction SilentlyContinue) {
    Write-Host "Missing git install!"
    Exit
}
if !(Get-Command "cargo" -ErrorAction SilentlyContinue) {
    Write-Host "Command does not exist."
    Exit
}
if !(Get-Command "npm" -ErrorAction SilentlyContinue) {
    Write-Host "Command does not exist."
    Exit
}
npm i
cargo build --release
npm run build
Exit
