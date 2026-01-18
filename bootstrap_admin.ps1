#!/usr/bin/env pwsh
param(
    [string]$DbPath = ".\depin_orcha.db",
    [string]$KeyName = "Admin Bootstrap Key",
    [int]$RateLimit = 1000
)

Write-Host "DePIN-Orcha Admin Key Bootstrap" -ForegroundColor Cyan
Write-Host "=" * 50

# Generate API key
$uuid = [System.Guid]::NewGuid().ToString("N")
$apiKey = "dpn_$uuid"

Write-Host "Generated API Key: " -NoNewline
Write-Host $apiKey -ForegroundColor Green

# Hash using SHA256
$sha256 = [System.Security.Cryptography.SHA256]::Create()
$keyBytes = [System.Text.Encoding]::UTF8.GetBytes($apiKey)
$hashBytes = $sha256.ComputeHash($keyBytes)
$keyHash = [Convert]::ToBase64String($hashBytes)

# Current timestamp
$now = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffffffK")

# Permissions
$permissions = '["read","write","admin","delete"]'

# Create SQL script
$sql = @"
INSERT INTO api_keys
    (key_hash, name, description, created_at, is_active, rate_limit_per_minute, permissions)
VALUES
    ('$keyHash', '$KeyName', 'Bootstrap admin key', '$now', 1, $RateLimit, '$permissions');
"@

Set-Content -Path "bootstrap.sql" -Value $sql -Encoding UTF8

# Try direct PowerShell SQLite access
try {
    # Using dotnet sqlite package if available
    $query = "INSERT INTO api_keys (key_hash, name, description, created_at, is_active, rate_limit_per_minute, permissions) VALUES ('$keyHash', '$KeyName', 'Bootstrap admin key', '$now', 1, $RateLimit, '$permissions')"

    # Execute using cargo
    cargo run --bin depin-orcha -- bootstrap --api-key "$apiKey" 2>&1 | Out-Null
}
catch {
    Write-Host "Creating bootstrap SQL file..." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "API Key Details:" -ForegroundColor Cyan
Write-Host "   Key: $apiKey"
Write-Host "   Rate Limit: $RateLimit req/min"
Write-Host ""
Write-Host "Usage: Add header to requests" -ForegroundColor Gray
Write-Host "   X-API-Key: $apiKey"

Write-Host ""
Write-Host "=" * 50
