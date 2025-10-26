#!/bin/bash
# LAN Chat - Mac Diagnostic Script

echo "========================================"
echo "LAN Chat - Mac Diagnostic Test"
echo "========================================"
echo ""

# Test 1: Check network configuration
echo "[1/5] Checking network configuration..."
IP_ADDR=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -1)

if [ -n "$IP_ADDR" ]; then
    echo "  ✓ Local IP: $IP_ADDR"
else
    echo "  ✗ No network connection found"
fi

# Test 2: Check firewall status
echo ""
echo "[2/5] Checking firewall status..."
FW_STATUS=$(/usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate 2>/dev/null)

if echo "$FW_STATUS" | grep -q "enabled"; then
    echo "  ⚠ Firewall is ENABLED"
    echo "    This might block peer discovery"
    echo "    To test, disable temporarily:"
    echo "    sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off"
elif echo "$FW_STATUS" | grep -q "disabled"; then
    echo "  ✓ Firewall is disabled"
else
    echo "  ? Could not determine firewall status"
fi

# Test 3: Check if ports are listening
echo ""
echo "[3/5] Checking if app ports are listening..."

for PORT in 37842 37843 37844; do
    if lsof -i :$PORT >/dev/null 2>&1; then
        echo "  ✓ Port $PORT is listening"
    else
        echo "  ✗ Port $PORT is NOT listening"
        echo "    Make sure the app is running"
    fi
done

# Test 4: Check multicast connectivity
echo ""
echo "[4/5] Testing multicast connectivity..."
if ping -c 2 -t 1 239.255.42.99 >/dev/null 2>&1; then
    echo "  ✓ Multicast is working"
else
    echo "  ⚠ Multicast test inconclusive"
    echo "    This is normal if no other device is responding"
fi

# Test 5: Check development environment
echo ""
echo "[5/5] Checking development environment..."

if command -v node >/dev/null 2>&1; then
    NODE_VER=$(node --version)
    echo "  ✓ Node.js installed: $NODE_VER"
else
    echo "  ✗ Node.js not found"
fi

if command -v npm >/dev/null 2>&1; then
    NPM_VER=$(npm --version)
    echo "  ✓ npm installed: $NPM_VER"
else
    echo "  ✗ npm not found"
fi

if command -v cargo >/dev/null 2>&1; then
    CARGO_VER=$(cargo --version)
    echo "  ✓ Rust/Cargo installed: $CARGO_VER"
else
    echo "  ✗ Rust/Cargo not found"
fi

# Summary
echo ""
echo "========================================"
echo "SUMMARY & RECOMMENDATIONS"
echo "========================================"
echo ""

if echo "$FW_STATUS" | grep -q "enabled"; then
    echo "⚠ ACTION: Disable firewall temporarily to test"
    echo "   Run: sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off"
    echo ""
fi

if ! lsof -i :37842 >/dev/null 2>&1; then
    echo "⚠ WARNING: App might not be running"
    echo "   Make sure you've run: npm run tauri dev"
    echo ""
fi

echo "Quick Test Steps:"
echo "1. Disable firewall: sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off"
echo "2. Verify your IP: $IP_ADDR"
echo "3. From Windows, ping this Mac: ping $IP_ADDR"
echo "4. Wait 30 seconds for discovery"
echo ""

echo "If Windows can't ping this Mac:"
echo "  → Router has AP Isolation enabled"
echo "  → Check router settings and disable it"
echo ""

echo "For detailed debugging, see:"
echo "  - CROSS_PLATFORM_DEBUG.md"
echo "  - TROUBLESHOOTING.md"
echo ""
echo "========================================"
