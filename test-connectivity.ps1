# Quick Connectivity Test - Run on Windows

param(
    [Parameter(Mandatory=$true)]
    [string]$MacIP
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Testing Connectivity to Mac" -ForegroundColor Cyan
Write-Host "Mac IP: $MacIP" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Test 1: Basic ping
Write-Host "[1/4] Testing basic network connectivity..." -ForegroundColor Yellow
$pingResult = Test-Connection -ComputerName $MacIP -Count 4 -Quiet

if ($pingResult) {
    Write-Host "  ✓ Can ping Mac successfully!" -ForegroundColor Green
    Write-Host "    Network connection is working" -ForegroundColor Gray
} else {
    Write-Host "  ✗ Cannot ping Mac" -ForegroundColor Red
    Write-Host "    This means:" -ForegroundColor Gray
    Write-Host "    - Router has AP Isolation enabled, OR" -ForegroundColor Gray
    Write-Host "    - Devices are on different networks, OR" -ForegroundColor Gray
    Write-Host "    - Mac firewall is blocking ICMP" -ForegroundColor Gray
    Write-Host "`n  FIX: Check router for 'AP Isolation' setting and disable it" -ForegroundColor Yellow
}

# Test 2: Check if multicast works
Write-Host "`n[2/4] Testing multicast..." -ForegroundColor Yellow
$multicastPing = Test-Connection -ComputerName "239.255.42.99" -Count 2 -Quiet -ErrorAction SilentlyContinue

if ($multicastPing) {
    Write-Host "  ✓ Multicast is responding" -ForegroundColor Green
} else {
    Write-Host "  ⚠ Multicast not responding (this is normal if no peers are broadcasting)" -ForegroundColor Yellow
}

# Test 3: Check local firewall
Write-Host "`n[3/4] Checking Windows Firewall..." -ForegroundColor Yellow
$firewallRules = Get-NetFirewallRule -DisplayName "LAN Chat*" -ErrorAction SilentlyContinue

if ($firewallRules) {
    Write-Host "  ✓ Firewall rules exist: $($firewallRules.Count) rules found" -ForegroundColor Green
} else {
    Write-Host "  ⚠ No firewall rules found" -ForegroundColor Yellow
    Write-Host "    Add rules by running as Administrator:" -ForegroundColor Gray
    Write-Host "    New-NetFirewallRule -DisplayName 'LAN Chat Discovery' -Direction Inbound -Protocol UDP -LocalPort 37842 -Action Allow" -ForegroundColor Gray
}

# Test 4: Check if app is running
Write-Host "`n[4/4] Checking if LAN Chat is running..." -ForegroundColor Yellow
$ports = @(37842, 37843, 37844)
$allRunning = $true

foreach ($port in $ports) {
    $connection = Get-NetUDPEndpoint -LocalPort $port -ErrorAction SilentlyContinue
    if (-not $connection) {
        $connection = Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue
    }
    
    if ($connection) {
        Write-Host "  ✓ Port $port is active" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Port $port is not active" -ForegroundColor Red
        $allRunning = $false
    }
}

if (-not $allRunning) {
    Write-Host "`n  ⚠ App might not be running. Start it with:" -ForegroundColor Yellow
    Write-Host "    cd desktop" -ForegroundColor Gray
    Write-Host "    npm run tauri dev" -ForegroundColor Gray
}

# Summary and recommendations
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "DIAGNOSIS & NEXT STEPS" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

if ($pingResult) {
    Write-Host "✓ Network connectivity: GOOD" -ForegroundColor Green
    Write-Host "`nSince you CAN ping the Mac, the issue is likely:" -ForegroundColor Yellow
    Write-Host "1. Firewall blocking UDP multicast" -ForegroundColor White
    Write-Host "2. App not running on both devices" -ForegroundColor White
    Write-Host "`nTRY THIS:" -ForegroundColor Cyan
    Write-Host "1. On Mac, disable firewall:" -ForegroundColor White
    Write-Host "   sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off" -ForegroundColor Gray
    Write-Host "2. On Windows, run as Administrator:" -ForegroundColor White
    Write-Host "   netsh advfirewall set allprofiles state off" -ForegroundColor Gray
    Write-Host "3. Restart both apps" -ForegroundColor White
    Write-Host "4. Wait 30 seconds" -ForegroundColor White
} else {
    Write-Host "✗ Network connectivity: FAILED" -ForegroundColor Red
    Write-Host "`nSince you CANNOT ping the Mac, the issue is:" -ForegroundColor Yellow
    Write-Host "1. Router has AP Isolation / Client Isolation enabled" -ForegroundColor White
    Write-Host "2. Devices are on different networks (check IP addresses)" -ForegroundColor White
    Write-Host "`nFIX THIS FIRST:" -ForegroundColor Cyan
    Write-Host "1. Log into your router (usually 192.168.1.1)" -ForegroundColor White
    Write-Host "2. Look for 'AP Isolation' or 'Client Isolation'" -ForegroundColor White
    Write-Host "3. DISABLE this setting" -ForegroundColor White
    Write-Host "4. Restart router if needed" -ForegroundColor White
    Write-Host "5. Try ping test again" -ForegroundColor White
}

Write-Host "`n========================================`n" -ForegroundColor Cyan
