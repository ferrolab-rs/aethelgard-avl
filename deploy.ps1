# Sovereign-AVL Automated Deployment Script (Ferrolab) - Fixed v2

$ErrorActionPreference = "Stop"

Write-Host ">>> Starting Sovereign-AVL Deployment Sequence (Correction)..." -ForegroundColor Cyan

# 1. Verification of Environment Variables
if (-not $env:GITHUB_TOKEN) {
    Write-Error "ERROR: `$env:GITHUB_TOKEN is not set."
}

if (-not $env:CARGO_TOKEN) {
    Write-Error "ERROR: `$env:CARGO_TOKEN is not set."
}

# 2. Local Repository Status
Write-Host ">> Verifying local Git status..."
git status

# 3. Create Remote Repository (Via API if it doesn't exist)
# FIXED: Using /user/repos instead of /orgs/ferrolab-rs/repos
Write-Host ">> Ensuring remote repository exists on GitHub (user: ferrolab-rs)..."
$headers = @{
    "Authorization" = "token $($env:GITHUB_TOKEN)"
    "Accept"        = "application/vnd.github.v3+json"
}

$body = @{
    name = "aethelgard-avl"
    description = "A Sovereign, NASA-grade AVL Tree for Rust (by Ferrolab)."
    private = $false
} | ConvertTo-Json

try {
    # CORRECT ENDPOINT FOR USER ACCOUNT
    $resp = Invoke-RestMethod -Uri "https://api.github.com/user/repos" -Method Post -Headers $headers -Body $body
    Write-Host ">> Success: Repository created on GitHub: $($resp.html_url)" -ForegroundColor Green
} catch {
    $err = $_.Exception.Message
    if ($err -match "422") { # Unprocessable Entity - commonly means repo already exists
        Write-Host ">> Note: Repository already exists on GitHub (422), proceeding." -ForegroundColor Yellow
    } else {
        Write-Error ">> CRITICAL: Repo creation failed: $err"
    }
}

# 4. Push to GitHub
Write-Host ">> Pushing to GitHub (main branch)..."
try {
    git push -u origin main --force
    if ($LASTEXITCODE -ne 0) { throw "Push failed with exit code $LASTEXITCODE" }
} catch {
    Write-Error ">> CRITICAL: Git push failed. Error: $_"
}

# 5. Publish to Crates.io
Write-Host ">> Publishing to Crates.io..."
try {
    cargo publish --token $env:CARGO_TOKEN --allow-dirty
    Write-Host ">> Success: Published to Crates.io." -ForegroundColor Green
} catch {
    Write-Error ">> CRITICAL: Cargo publish failed. Error: $_"
}

Write-Host "`n>>> DEPLOYMENT SUCCESSFUL: Sovereign-AVL is now LIVE." -ForegroundColor Green
Write-Host "GitHub: https://github.com/ferrolab-rs/aethelgard-avl"
Write-Host "Crates: https://crates.io/crates/aethelgard-avl"
