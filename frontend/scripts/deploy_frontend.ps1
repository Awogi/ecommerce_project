# Build the frontend with Trunk and copy the dist/ output into backend/static/
# Usage: run from repository root or from frontend folder

param(
    [string]$TrunkPath = "trunk",
    [string]$FrontendDir = "..\frontend",
    [string]$BackendStaticDir = "..\backend\static"
)

Write-Host "Building frontend with Trunk (release)..."
Push-Location $FrontendDir
& $TrunkPath build --release
$exitCode = $LASTEXITCODE
Pop-Location

if ($exitCode -ne 0) {
    Write-Error "Trunk build failed (exit code $exitCode). Aborting copy."
    exit $exitCode
}

$distPath = Join-Path -Path $FrontendDir -ChildPath "dist"
if (-Not (Test-Path $distPath)) {
    Write-Error "Dist folder not found at $distPath"
    exit 1
}

Write-Host "Copying dist/* -> $BackendStaticDir"
# Ensure target directory exists
if (-Not (Test-Path $BackendStaticDir)) {
    New-Item -ItemType Directory -Path $BackendStaticDir | Out-Null
}

# Remove old files (be careful) then copy
Get-ChildItem -Path $BackendStaticDir -Force | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
Copy-Item -Path (Join-Path $distPath "*") -Destination $BackendStaticDir -Recurse -Force

Write-Host "Frontend deployed to backend static folder. You can now run the backend and it will serve the frontend at /." -ForegroundColor Green
