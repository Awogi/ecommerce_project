param(
    [int]$BackendPort = 8080,
    [int]$FrontendPort = 8082
)

# Get script root (repo root)
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Host "Starting development environment..."

# Start backend in a new PowerShell window with PORT set
$backendPath = Join-Path $scriptDir "backend"
$backendCmd = "Set-Location -Path '$backendPath'; $env:PORT='$BackendPort'; cargo run"
Write-Host "Starting backend on port $BackendPort in a new window..."
Start-Process -FilePath powershell -ArgumentList "-NoExit","-Command","$backendCmd"

# Start trunk in a new PowerShell window
$frontendPath = Join-Path $scriptDir "frontend"
$frontendCmd = "Set-Location -Path '$frontendPath'; trunk serve --port $FrontendPort"
Write-Host "Starting frontend (Trunk) on port $FrontendPort in a new window..."
Start-Process -FilePath powershell -ArgumentList "-NoExit","-Command","$frontendCmd"

Write-Host "\nDev servers started."
Write-Host "Backend: http://127.0.0.1:$BackendPort (or check backend output)"
Write-Host "Frontend: http://127.0.0.1:$FrontendPort"
Write-Host "If frontend needs to call the backend while Trunk serves the app, run this in the browser console to set the runtime override (no rebuild):"
Write-Host "localStorage.setItem('api_base', 'http://127.0.0.1:$BackendPort')"
Write-Host "\nOpen the frontend URL in your browser to start debugging."
