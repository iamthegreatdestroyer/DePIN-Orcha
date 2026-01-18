# Authentication & Rate Limiting Test Script

Write-Host "`n🧪 DePIN-Orcha Authentication & Rate Limiting Tests`n" -ForegroundColor Cyan

# Test 1: Create API Key
Write-Host "Test 1: Creating API Key..." -ForegroundColor Yellow
$createResponse = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys" `
    -Method POST `
    -ContentType "application/json" `
    -Body (@{
        name                  = "test-key-1"
        description           = "First test API key"
        expires_in_days       = 30
        rate_limit_per_minute = 60
    } | ConvertTo-Json)

Write-Host "✅ API Key created:" -ForegroundColor Green
Write-Host "   ID: $($createResponse.id)"
Write-Host "   Key: $($createResponse.key)"
Write-Host "   Name: $($createResponse.name)"
Write-Host "   Rate Limit: $($createResponse.rate_limit_per_minute)/min"
Write-Host "   Expires: $($createResponse.expires_at)"

$apiKey = $createResponse.key

# Test 2: List API Keys
Write-Host "`nTest 2: Listing all API keys..." -ForegroundColor Yellow
$listResponse = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys"
Write-Host "✅ Found $($listResponse.Count) API key(s)" -ForegroundColor Green

# Test 3: Test auth without key (should fail)
Write-Host "`nTest 3: Testing protected endpoint WITHOUT API key..." -ForegroundColor Yellow
try {
    Invoke-RestMethod -Uri "http://localhost:8080/api/v1/metrics/current"
    Write-Host "❌ FAILED: Request should have been rejected" -ForegroundColor Red
}
catch {
    Write-Host "✅ Correctly rejected: $($_.Exception.Response.StatusCode)" -ForegroundColor Green
}

# Test 4: Test auth with valid key
Write-Host "`nTest 4: Testing protected endpoint WITH valid API key..." -ForegroundColor Yellow
$headers = @{
    "X-API-Key" = $apiKey
}
$metricsResponse = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/metrics/current" -Headers $headers
Write-Host "✅ Successfully authenticated!" -ForegroundColor Green

# Test 5: Test rate limiting
Write-Host "`nTest 5: Testing rate limiting (sending 65 requests)..." -ForegroundColor Yellow
$successCount = 0
$rateLimitedCount = 0

for ($i = 1; $i -le 65; $i++) {
    try {
        Invoke-RestMethod -Uri "http://localhost:8080/api/v1/metrics/current" -Headers $headers -ErrorAction Stop | Out-Null
        $successCount++
        if ($i % 10 -eq 0) {
            Write-Host "   Sent $i requests..." -NoNewline
        }
    }
    catch {
        if ($_.Exception.Response.StatusCode -eq 429) {
            $rateLimitedCount++
        }
    }
}

Write-Host ""
Write-Host "✅ Results:" -ForegroundColor Green
Write-Host "   Successful requests: $successCount"
Write-Host "   Rate-limited requests: $rateLimitedCount"

if ($rateLimitedCount -gt 0) {
    Write-Host "   ✅ Rate limiting is working!" -ForegroundColor Green
}
else {
    Write-Host "   ⚠️  No rate limiting detected" -ForegroundColor Yellow
}

# Test 6: Update API key
Write-Host "`nTest 6: Updating API key name..." -ForegroundColor Yellow
$updateResponse = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys/$($createResponse.id)" `
    -Method PUT `
    -ContentType "application/json" `
    -Body (@{
        name = "updated-test-key"
    } | ConvertTo-Json)
Write-Host "✅ API key updated: $($updateResponse.name)" -ForegroundColor Green

# Test 7: Get API key info
Write-Host "`nTest 7: Getting API key info..." -ForegroundColor Yellow
$infoResponse = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys/$($createResponse.id)"
Write-Host "✅ API Key Info:" -ForegroundColor Green
Write-Host "   Name: $($infoResponse.name)"
Write-Host "   Active: $($infoResponse.is_active)"
Write-Host "   Last Used: $($infoResponse.last_used_at)"

# Test 8: Delete API key
Write-Host "`nTest 8: Deleting API key..." -ForegroundColor Yellow
Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys/$($createResponse.id)" -Method DELETE
Write-Host "✅ API key deleted" -ForegroundColor Green

# Test 9: Verify deletion
Write-Host "`nTest 9: Verifying deletion..." -ForegroundColor Yellow
try {
    Invoke-RestMethod -Uri "http://localhost:8080/api/v1/metrics/current" -Headers $headers
    Write-Host "❌ FAILED: Deleted key should not work" -ForegroundColor Red
}
catch {
    Write-Host "✅ Correctly rejected deleted key: $($_.Exception.Response.StatusCode)" -ForegroundColor Green
}

Write-Host "`n🎉 All tests completed!" -ForegroundColor Cyan
