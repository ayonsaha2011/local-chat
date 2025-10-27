# Recent Fixes and Troubleshooting Guide

## Critical Fixes Applied

### 1. Multicast Interface Binding Order (FIXED)
**Issue**: The multicast interface was being set AFTER joining the multicast group, which caused issues on macOS and some network configurations.

**Fix**: Reordered socket configuration to set the multicast interface BEFORE joining the group.

**File**: `crates/discovery/src/service.rs:120-136`

**Impact**: This is the most likely fix for Windows-Mac connectivity issues.

---

### 2. Loopback IP Fallback Removed (FIXED)
**Issue**: If local IP detection failed, the app would fall back to `127.0.0.1` (loopback), which cannot be reached from other devices.

**Fix**:
- Added multiple fallback methods for IP detection (Google DNS, Cloudflare, OpenDNS)
- Removed loopback fallback - app now fails with clear error message
- Added validation to ensure detected IP is not loopback

**File**: `desktop/src-tauri/src/state.rs:167-213`

**Impact**: Prevents silent failures where devices think they're discoverable but aren't.

---

### 3. Socket Conversion Performance (FIXED)
**Issue**: The receive loop was converting socket types on every packet, causing performance issues and potential crashes with `unwrap()`.

**Fix**: Convert socket once before the loop and reuse it with proper error handling.

**File**: `crates/discovery/src/service.rs:145-182`

**Impact**: Better performance and reliability.

---

### 4. Peer Timeout Documentation (IMPROVED)
**Issue**: Inconsistent timing between when peers show offline (30s) vs when they're removed (45s).

**Fix**: Added documentation explaining the relationship between heartbeat interval (15s), offline threshold (30s), and cleanup timeout (45s).

**Files**:
- `crates/discovery/src/service.rs:12-14`
- `crates/core/src/peer.rs:32-37`

**Impact**: Clearer code maintenance.

---

## Troubleshooting: Devices Not Seeing Each Other

### Step 1: Verify Network Connection

**Windows PC:**
```cmd
ipconfig
```
Look for your IPv4 Address under your active network adapter (e.g., `192.168.1.100`)

**Mac:**
```bash
ifconfig | grep "inet " | grep -v 127.0.0.1
```
Look for an IP like `192.168.1.101`

**Requirements:**
- Both devices MUST be on the same subnet (e.g., both start with `192.168.1.`)
- IPs must NOT be `127.0.0.1` (loopback)

---

### Step 2: Windows Firewall (CRITICAL)

Windows Defender Firewall blocks multicast by default. You MUST add firewall rules.

**Easy Method:**
1. Right-click `fix-firewall.cmd` in the project folder
2. Select "Run as Administrator"
3. Follow the prompts

**Manual Method:**
Run PowerShell as Administrator:
```powershell
netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP In)" dir=in action=allow protocol=UDP localport=37842
netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP Out)" dir=out action=allow protocol=UDP localport=37842
netsh advfirewall firewall add rule name="LAN Chat Messaging (TCP In)" dir=in action=allow protocol=TCP localport=37843
netsh advfirewall firewall add rule name="LAN Chat Transfer (TCP In)" dir=in action=allow protocol=TCP localport=37844
```

**Verify:**
```cmd
netsh advfirewall firewall show rule name=all | findstr "LAN Chat"
```

---

### Step 3: macOS Firewall

**Check Firewall Status:**
```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate
```

**Option A: Disable Firewall (Testing Only):**
```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
```

**Option B: Add Exception:**
Add the app to System Settings > Network > Firewall > Options > Allow incoming connections

---

### Step 4: Router Configuration

**Check for AP Isolation:**
1. Open your router admin panel (usually `192.168.1.1` or `192.168.0.1`)
2. Look for "AP Isolation", "Client Isolation", or "Guest Network"
3. DISABLE these features
4. Restart router if needed

**Check IGMP Snooping:**
- IGMP Snooping should be ENABLED for multicast to work
- Usually found under Advanced > Multicast settings

---

### Step 5: Test Multicast

**Windows:**
```cmd
ping 239.255.42.99
```

**Mac:**
```bash
ping -c 5 239.255.42.99
```

If you see replies, multicast is working on your network.

---

### Step 6: Check App Logs

When you start the app, check the console/logs for:

**Good Signs:**
```
✅ Local IP: 192.168.1.100
Set multicast interface to: 192.168.1.100
Joined multicast group 239.255.42.99 on interface 192.168.1.100
✅ Peer discovered: [Name] at 192.168.1.101
```

**Bad Signs:**
```
❌ CRITICAL: Failed to detect local network IP address!
Failed to join multicast group
Failed to set multicast interface
```

---

### Step 7: Advanced Debugging with Wireshark

If devices still can't see each other:

1. Install Wireshark on Windows
2. Start capture on your active network interface
3. Filter: `udp.port == 37842`
4. Start the app
5. You should see multicast packets to `239.255.42.99:37842`

**What to check:**
- Are packets being sent? (You should see them every 15 seconds)
- Are packets being received from the other device?
- What interface are packets going through?

---

## Common Issues and Solutions

### Issue: "Failed to detect local network IP address"

**Cause**: Network connection is down or no active network interface

**Solution**:
1. Check network connection
2. Disable VPN temporarily
3. Restart network adapter
4. Check if you're connected to WiFi/Ethernet

---

### Issue: Devices show up but can't send messages

**Cause**: TCP port 37843 is blocked

**Solution**:
1. Add firewall rule for TCP port 37843 (see Step 2)
2. Check router doesn't block TCP connections between clients

---

### Issue: Files won't transfer

**Cause**: TCP port 37844 is blocked

**Solution**:
1. Add firewall rule for TCP port 37844 (see Step 2)

---

### Issue: One device sees the other, but not vice versa

**Cause**: Asymmetric firewall rules

**Solution**:
1. Ensure BOTH devices have firewall rules configured
2. Check both inbound AND outbound rules are present
3. On Windows, ensure rules apply to both Private and Public networks

---

### Issue: Devices can ping each other but app doesn't work

**Cause**: Multicast is not the same as unicast (regular ping)

**Solution**:
1. Test multicast specifically: `ping 239.255.42.99`
2. Check IGMP Snooping is enabled on router
3. Disable AP Isolation on router

---

## Testing Checklist

Before reporting issues, verify:

- [ ] Both devices on same subnet (check with `ipconfig`/`ifconfig`)
- [ ] Windows firewall rules added (run `fix-firewall.cmd` as admin)
- [ ] macOS firewall disabled or app exception added
- [ ] Router AP Isolation is OFF
- [ ] Router IGMP Snooping is ON
- [ ] Multicast test succeeds: `ping 239.255.42.99`
- [ ] App logs show correct local IP (not 127.0.0.1)
- [ ] App logs show "Joined multicast group" message

---

## Network Ports Reference

| Port  | Protocol | Purpose           | Direction |
|-------|----------|-------------------|-----------|
| 37842 | UDP      | Peer Discovery    | In + Out  |
| 37843 | TCP      | Messaging         | Inbound   |
| 37844 | TCP      | File Transfer     | Inbound   |

**Multicast Address**: `239.255.42.99` (IPv4)

---

## Still Having Issues?

If you've tried everything above and devices still can't see each other:

1. Check that you're not on a corporate/school network (they often block multicast)
2. Try connecting both devices to a mobile hotspot
3. Check app logs for specific error messages
4. Use Wireshark to see if multicast packets are being sent/received
5. Open an issue on GitHub with:
   - OS versions (Windows 11, macOS Ventura, etc.)
   - Network setup (same WiFi, different subnets, etc.)
   - App logs from both devices
   - Results of multicast ping test

---

## Technical Details

### Discovery Protocol
- Uses UDP multicast (RFC 1112)
- Multicast group: `239.255.42.99` (site-local scope)
- Heartbeat every 15 seconds
- Peers marked offline after 30 seconds (2 missed heartbeats)
- Peers removed after 45 seconds (3 missed heartbeats)

### Why Multicast?
- Efficient: One packet reaches all devices
- No central server needed
- Works across routers with IGMP
- Industry standard for LAN discovery

### Firewall Requirements
- **UDP 37842**: Discovery packets must be received
- **TCP 37843**: Must accept incoming connections for messaging
- **TCP 37844**: Must accept incoming connections for file transfer
