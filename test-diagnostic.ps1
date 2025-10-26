# Quick Diagnostic Test

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "LAN Chat - Quick Diagnostic Test" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Test 1: Check if required ports are available
Write-Host "[1/5] Checking if ports are available..." -ForegroundColor Yellow
$ports = @(37842, 37843, 37844)
$portsInUse = @()

foreach ($port in $ports) {
    $result = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
    if ($result) {
        $portsInUse += $port
        Write-Host "  X Port $port is in use" -ForegroundColor Red
    } else {
        Write-Host "  OK Port $port is available" -ForegroundColor Green
    }
}

# Test 2: Check network configuration
Write-Host "`n[2/5] Checking network configuration..." -ForegroundColor Yellow
$ipConfig = Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notlike "*Loopback*" } | Select-Object -First 1

if ($ipConfig) {
    Write-Host "  OK Local IP: $($ipConfig.IPAddress)" -ForegroundColor Green
    Write-Host "    Interface: $($ipConfig.InterfaceAlias)" -ForegroundColor Gray
} else {
    Write-Host "  X No network connection found" -ForegroundColor Red
}

# Test 3: Check firewall status
Write-Host "`n[3/5] Checking firewall status..." -ForegroundColor Yellow
$firewallProfiles = Get-NetFirewallProfile
$anyEnabled = $firewallProfiles | Where-Object { $_.Enabled -eq $true }

if ($anyEnabled) {
    Write-Host "  ! Windows Firewall is enabled" -ForegroundColor Yellow
    Write-Host "    You may need to add firewall rules" -ForegroundColor Gray
    
    # Check if rules exist
    $rules = Get-NetFirewallRule -DisplayName "LAN Chat*" -ErrorAction SilentlyContinue
    if ($rules) {
        Write-Host "  OK LAN Chat firewall rules found: $($rules.Count)" -ForegroundColor Green
    } else {
        Write-Host "  X No LAN Chat firewall rules found" -ForegroundColor Red
        Write-Host "    Run as Administrator to create rules" -ForegroundColor Gray
    }
} else {
    Write-Host "  OK Windows Firewall is disabled" -ForegroundColor Green
}

# Test 4: Check if Node.js and npm are installed
Write-Host "`n[4/5] Checking development environment..." -ForegroundColor Yellow

$nodeVersion = $null
$npmVersion = $null
$cargoVersion = $null

try {
    $nodeVersion = node --version 2>$null
    if ($nodeVersion) {
        Write-Host "  OK Node.js installed: $nodeVersion" -ForegroundColor Green
    } else {
        Write-Host "  X Node.js not found" -ForegroundColor Red
    }
} catch {
    Write-Host "  X Node.js not found" -ForegroundColor Red
}

try {
    $npmVersion = npm --version 2>$null
    if ($npmVersion) {
        Write-Host "  OK npm installed: $npmVersion" -ForegroundColor Green
    } else {
        Write-Host "  X npm not found" -ForegroundColor Red
    }
} catch {
    Write-Host "  X npm not found" -ForegroundColor Red
}

try {
    $cargoVersion = cargo --version 2>$null
    if ($cargoVersion) {
        Write-Host "  OK Rust/Cargo installed: $cargoVersion" -ForegroundColor Green
    } else {
        Write-Host "  X Rust/Cargo not found" -ForegroundColor Red
    }
} catch {
    Write-Host "  X Rust/Cargo not found" -ForegroundColor Red
}

# Test 5: Check if project files exist
Write-Host "`n[5/5] Checking project structure..." -ForegroundColor Yellow

$projectRoot = "d:\workspace\local-ip-chat"
$requiredPaths = @(
    "$projectRoot\Cargo.toml",
    "$projectRoot\desktop\package.json",
    "$projectRoot\desktop\src-tauri\Cargo.toml",
    "$projectRoot\crates\core",
    "$projectRoot\crates\discovery"
)

$allExist = $true
foreach ($path in $requiredPaths) {
    if (Test-Path $path) {
        Write-Host "  OK Found: $(Split-Path $path -Leaf)" -ForegroundColor Green
    } else {
        Write-Host "  X Missing: $path" -ForegroundColor Red
        $allExist = $false
    }
}

# Summary
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "SUMMARY" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

if ($portsInUse.Count -gt 0) {
    Write-Host "! WARNING: Some ports are in use" -ForegroundColor Yellow
    Write-Host "  Close any apps using ports: $($portsInUse -join ', ')" -ForegroundColor Gray
    Write-Host "  Or restart your computer`n" -ForegroundColor Gray
}

if ($anyEnabled -and (-not $rules)) {
    Write-Host "! ACTION REQUIRED: Add firewall rules" -ForegroundColor Yellow
    Write-Host "  Run this script as Administrator, or" -ForegroundColor Gray
    Write-Host "  Temporarily disable Windows Firewall to test`n" -ForegroundColor Gray
}

if (-not $ipConfig) {
    Write-Host "X ERROR: No network connection" -ForegroundColor Red
    Write-Host "  Connect to WiFi or Ethernet`n" -ForegroundColor Gray
}

Write-Host "Next Steps:" -ForegroundColor Cyan
Write-Host "1. Run: cd desktop" -ForegroundColor White
Write-Host "2. Run: npm install (if not done)" -ForegroundColor White
Write-Host "3. Run: npm run tauri dev" -ForegroundColor White
Write-Host "4. Create profile in the app" -ForegroundColor White
Write-Host "5. Run app on SECOND device (same network)" -ForegroundColor White
Write-Host "6. Wait 30 seconds for discovery`n" -ForegroundColor White

Write-Host "For detailed debugging, see:" -ForegroundColor Cyan
Write-Host "  - DEBUG_CHECKLIST.md" -ForegroundColor Gray
Write-Host "  - TROUBLESHOOTING.md`n" -ForegroundColor Gray

Write-Host "========================================`n" -ForegroundColor Cyan
