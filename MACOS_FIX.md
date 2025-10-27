# macOS Multicast Receive Fix

## Problem Diagnosed

Based on your logs, the issue was **asymmetric communication**:

### What Was Working:
- ✅ Mac could SEND multicast packets (Windows received them)
- ✅ Windows could SEND multicast packets
- ✅ Windows could RECEIVE multicast packets (saw Mac)

### What Was NOT Working:
- ❌ Mac could NOT RECEIVE multicast packets (couldn't see Windows)

### Evidence from Logs:

**Windows logs** (started first at 02:58:45):
```
02:58:45.359 - Sending announcement: User from 192.168.1.100
02:58:45.359 - Sending discovery request
02:59:08.526 - ✅ Peer discovered: User at 192.168.1.103  ← Mac discovered!
02:59:08.527 - 📡 Discovery request from 192.168.1.103    ← Received Mac's request
```

**Mac logs** (started second at 02:59:08):
```
02:59:08.688 - Sending announcement: User from 192.168.1.103
02:59:08.688 - Sending discovery request
02:59:08.689 - 📡 Discovery request from 192.168.1.103    ← Only sees itself!
(NO "Peer discovered" message - never saw Windows!)
```

### Analysis:
Mac sent packets → Windows received them ✅
Windows sent packets → **Mac did NOT receive them** ❌

This is a classic **macOS multicast socket configuration issue**.

---

## Root Cause: Missing SO_REUSEPORT

On macOS (and other BSD-derived systems), multicast sockets require **BOTH**:
- `SO_REUSEADDR` - Allow multiple sockets to bind to same address
- `SO_REUSEPORT` - **CRITICAL for macOS** - Allow multiple processes/sockets to receive multicast

Without `SO_REUSEPORT`, macOS may:
1. Bind the socket successfully
2. Join the multicast group successfully
3. **Silently fail to deliver incoming multicast packets to the socket**

This is a well-known macOS quirk that differs from Windows and Linux behavior.

---

## The Fix

### File: `crates/discovery/src/service.rs:111-116`

**Before (Broken on macOS):**
```rust
socket
    .set_reuse_address(true)
    .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), DISCOVERY_PORT);
socket.bind(&addr.into())?;
```

**After (Fixed):**
```rust
// SO_REUSEADDR allows multiple sockets to bind to the same address
socket
    .set_reuse_address(true)
    .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

// SO_REUSEPORT is CRITICAL for macOS multicast sockets
// Without this, macOS may not receive multicast packets properly
#[cfg(not(windows))]
socket
    .set_reuse_port(true)
    .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), DISCOVERY_PORT);
socket.bind(&addr.into())?;
```

### Why Platform-Specific?

- **macOS/BSD/Linux**: Require `SO_REUSEPORT` for multicast to work correctly
- **Windows**: Doesn't have `SO_REUSEPORT` (uses only `SO_REUSEADDR`)

The `#[cfg(not(windows))]` ensures this only compiles on non-Windows platforms.

---

## Additional Improvements

### Better Logging

Added clearer log messages to help diagnose issues:

```rust
info!("✓ Set multicast send interface to: {}", local_ipv4);
info!("✓ Joined multicast group {} on interface {} for RECEIVING", multicast_addr, interface_addr);
```

Now you can clearly see:
1. Which interface is being used for sending
2. Which interface joined the multicast group for receiving

---

## Testing Instructions

### 1. Rebuild on BOTH Devices

**On Windows:**
```bash
cargo build
npm run tauri dev
```

**On Mac:**
```bash
cargo build
npm run tauri dev
```

### 2. Test All Scenarios

#### Scenario A: Windows First
1. Start Windows app
2. Wait 2 seconds
3. Start Mac app
4. **Expected**: Both see each other immediately ✅

#### Scenario B: Mac First (This is the one that was broken!)
1. Start Mac app
2. Wait 2 seconds
3. Start Windows app
4. **Expected**: Both see each other immediately ✅ **← THIS SHOULD NOW WORK!**

#### Scenario C: Simultaneous Start
1. Start both apps at the same time
2. **Expected**: Both see each other immediately ✅

#### Scenario D: Restart
1. Both apps running
2. Close Mac app
3. Restart Mac app
4. **Expected**: Rejoins and discovers Windows immediately ✅

---

## Expected Log Output

### Mac Logs (After Fix):

```
Starting discovery service on port 37842
✓ Set multicast send interface to: 192.168.1.103
✓ Joined multicast group 239.255.42.99 on interface 192.168.1.103 for RECEIVING
Discovery receiver loop started, listening on multicast 239.255.42.99:37842
Sending announcement: User from 192.168.1.103
Sending discovery request to find existing peers
📡 Discovery request from 192.168.1.103, sending our info  ← Sees itself (normal)
📡 Discovery request from 192.168.1.100, sending our info  ← Sees Windows! (NEW!)
✅ Peer discovered via response: User at 192.168.1.100    ← Discovers Windows! (NEW!)
```

The **NEW** lines are what was missing before.

---

## Why This Wasn't Caught Earlier

The `test-multicast-receive.ps1` PowerShell script worked because PowerShell's UDP handling is different from Rust's `socket2` crate. PowerShell likely sets `SO_REUSEPORT` automatically or handles multicast differently.

This is why network-level testing (PowerShell script) showed packets flowing, but the Rust app couldn't receive them on Mac.

---

## Technical Background

### SO_REUSEADDR vs SO_REUSEPORT

| Option | Purpose | Windows | macOS/Linux |
|--------|---------|---------|-------------|
| `SO_REUSEADDR` | Allow binding to address already in use | Required | Required |
| `SO_REUSEPORT` | Allow multiple sockets to receive on same port | Not available | **Required for multicast** |

### Why macOS Needs SO_REUSEPORT

macOS kernel behavior (inherited from BSD):
1. When a multicast packet arrives on port 37842
2. Kernel checks: "Which socket(s) should receive this?"
3. **Without SO_REUSEPORT**: Delivers to only ONE socket (undefined which one)
4. **With SO_REUSEPORT**: Delivers to ALL sockets listening on that port

For multicast to work reliably, we need **ALL** matching sockets to receive packets.

---

## Platform Differences Summary

### Windows:
- Uses `SO_REUSEADDR` for multicast
- `SO_REUSEPORT` doesn't exist
- Multicast works with just `SO_REUSEADDR` ✅

### macOS/BSD:
- Needs BOTH `SO_REUSEADDR` and `SO_REUSEPORT`
- Kernel is stricter about multicast delivery
- Without `SO_REUSEPORT`, silently drops packets ❌

### Linux:
- Originally didn't have `SO_REUSEPORT` (added in kernel 3.9, 2013)
- Modern Linux (kernel 3.9+) behaves like macOS
- Older Linux behaves like Windows

---

## Verification Checklist

After rebuilding and testing:

- [ ] Mac logs show "✓ Set multicast send interface"
- [ ] Mac logs show "✓ Joined multicast group ... for RECEIVING"
- [ ] Mac logs show "📡 Discovery request from 192.168.1.100" (from Windows)
- [ ] Mac logs show "✅ Peer discovered: User at 192.168.1.100"
- [ ] Mac UI shows Windows user in contacts/peers list
- [ ] Can send messages from Mac to Windows
- [ ] Can send messages from Windows to Mac

---

## Still Not Working?

If Mac still can't receive packets after this fix:

### 1. Check macOS Firewall

```bash
# Check status
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate

# If enabled, temporarily disable for testing
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
```

### 2. Check Network Interface

```bash
# List network interfaces
ifconfig

# Check which interface has your IP
ifconfig | grep "inet 192.168"

# Make sure it's not a VPN or virtual interface
```

### 3. Check IGMP on Mac

```bash
# See if Mac is receiving IGMP reports
sudo tcpdump -i en0 igmp
```

(Replace `en0` with your actual interface - usually `en0` for WiFi)

### 4. Use tcpdump to See Multicast Packets

```bash
# Capture multicast UDP on port 37842
sudo tcpdump -i en0 -n udp port 37842 and dst host 239.255.42.99
```

You should see packets from both 192.168.1.100 (Windows) and 192.168.1.103 (Mac).

If you only see Mac's packets, router may be blocking multicast.

---

## Summary

**Issue**: Mac couldn't receive multicast packets due to missing `SO_REUSEPORT` socket option

**Fix**: Added `SO_REUSEPORT` for non-Windows platforms

**Impact**: Mac should now receive multicast packets and discover Windows peers

**Rebuild both apps and test!** The Mac app should now see Windows peers immediately, regardless of start order.
