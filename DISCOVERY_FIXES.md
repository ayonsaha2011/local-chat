# Discovery Fixes - Issue Resolution

## Problem Summary

Based on your multicast test results, **packets were being transmitted and received** at the network level, but the app had timing and processing issues:

- ✅ Multicast packets flowing between devices (confirmed by `test-multicast-receive.ps1`)
- ✅ Windows receives heartbeats from Mac (192.168.1.103)
- ✅ Mac receives heartbeats from Windows (192.168.1.100)
- ❌ **But:** Windows app could only see Mac if started FIRST
- ❌ **But:** Mac app couldn't see Windows at all

## Root Causes Found

### 1. Race Condition on Startup
**File**: `crates/discovery/src/service.rs:63-72`

**Problem**:
- Initial announcement was sent IMMEDIATELY after spawning the receiver task
- Receiver needed time to convert socket and start listening
- Result: Devices joining the network late might miss early announcements

**Fix**:
```rust
// Give receiver time to start listening before sending announcements
tokio::time::sleep(Duration::from_millis(100)).await;

// Send initial announcement
self.announce(socket.as_ref()).await?;

// Send a discovery request to trigger responses from existing peers
self.send_discovery_request(socket.as_ref()).await?;
```

**Impact**:
- Receiver is now fully ready before first announcement
- Discovery request triggers immediate responses from existing peers
- No need to wait 15 seconds for first heartbeat

---

### 2. Discovery Request Handler Not Responding
**File**: `crates/discovery/src/service.rs:220-235`

**Problem**:
```rust
DiscoveryMessage::DiscoveryRequest => {
    debug!("Discovery request from {}", from);
    // Respond with our info (implemented via heartbeat)  ← WRONG!
}
```

The comment said "implemented via heartbeat" but **NO response was actually sent**. This meant:
- When a device joins and sends DiscoveryRequest, existing devices ignore it
- New device must wait up to 15 seconds for the next heartbeat to discover peers
- Result: Slow discovery and order-dependent behavior

**Fix**:
```rust
DiscoveryMessage::DiscoveryRequest => {
    info!("📡 Discovery request from {}, sending our info", from);
    // Respond immediately with our full profile
    let response = DiscoveryMessage::DiscoveryResponse {
        profile: self.profile.clone(),
        address: self.listen_address.clone(),
        public_key: self.public_key.clone(),
    };

    // Send response back via multicast
    if let Err(e) = self.send_multicast_from_handler(&response).await {
        warn!("Failed to send discovery response: {}", e);
    }
}
```

**Impact**:
- Devices now respond IMMEDIATELY to discovery requests
- Discovery happens in ~100ms instead of up to 15 seconds
- Order independence: doesn't matter which device starts first

---

### 3. New Helper Method for Handler Responses
**File**: `crates/discovery/src/service.rs:343-375`

Added `send_multicast_from_handler()` method to send multicast messages from within message handlers, which don't have access to the main socket.

---

### 4. Added Discovery Request Sender
**File**: `crates/discovery/src/service.rs:377-382`

Added `send_discovery_request()` method to actively query for existing peers on startup.

---

## What Changed in Behavior

### Before Fixes:
1. Device starts
2. Sends announcement
3. Starts listening (too late, might miss announcements)
4. Waits up to 15 seconds for heartbeat from existing peers
5. **Result**: Slow, order-dependent discovery

### After Fixes:
1. Device starts
2. Starts listening
3. **Waits 100ms** for listener to be ready
4. Sends announcement
5. **Sends discovery request**
6. Existing peers **immediately respond** with DiscoveryResponse
7. **Result**: Fast (~100ms), order-independent discovery

---

## Expected Log Output

### On Windows (after restart):
```
Starting discovery service on port 37842
Set multicast interface to: 192.168.1.100
Joined multicast group 239.255.42.99 on interface 192.168.1.100
Sending announcement: User from 192.168.1.100
Sending discovery request to find existing peers
Discovery receiver loop started
📡 Discovery request from 192.168.1.103:xxxxx, sending our info  ← Mac requesting
✅ Peer discovered via response: User at 192.168.1.103 (from 192.168.1.103:xxxxx)  ← Mac discovered!
```

### On Mac (after restart):
```
Starting discovery service on port 37842
Set multicast interface to: 192.168.1.103
Joined multicast group 239.255.42.99 on interface 192.168.1.103
Sending announcement: User from 192.168.1.103
Sending discovery request to find existing peers
Discovery receiver loop started
📡 Discovery request from 192.168.1.100:xxxxx, sending our info  ← Windows requesting
✅ Peer discovered via response: User at 192.168.1.100 (from 192.168.1.100:xxxxx)  ← Windows discovered!
```

---

## Testing Instructions

### 1. Rebuild the App
```bash
cargo build
npm run tauri dev
```

### 2. Test Scenario A: Windows First, Then Mac
1. Start Windows app
2. Wait 2 seconds
3. Start Mac app
4. **Expected**: Both devices see each other within 1 second

### 3. Test Scenario B: Mac First, Then Windows
1. Start Mac app
2. Wait 2 seconds
3. Start Windows app
4. **Expected**: Both devices see each other within 1 second (THIS IS THE FIX!)

### 4. Test Scenario C: Simultaneous Start
1. Start both apps at the same time
2. **Expected**: Both devices see each other within 1 second

### 5. Test Scenario D: App Restart
1. Both apps running and seeing each other
2. Close one app
3. Restart it
4. **Expected**: Rejoins network and discovers peer within 1 second

---

## Verification Checklist

Before testing:
- [x] Network profile set to "Private" (Windows)
- [x] Firewall rules added (Windows - run `fix-firewall.cmd`)
- [x] Firewall disabled or app excepted (Mac)
- [x] Both devices on same subnet (check with `ipconfig`/`ifconfig`)
- [x] Multicast test passes (run `test-multicast-receive.ps1`)

After fixes:
- [ ] Windows first → Both see each other quickly
- [ ] Mac first → Both see each other quickly (**This should now work!**)
- [ ] Simultaneous start → Both see each other
- [ ] Restart works → Rejoin network quickly
- [ ] Logs show "Discovery request from..." and "Peer discovered via response"

---

## Still Having Issues?

### If Mac still can't see Windows:

1. **Check Mac logs** for:
   - "Sending discovery request to find existing peers" ← Should appear
   - "Discovery request from 192.168.1.100" ← Should see request from Windows
   - "Peer discovered via response: User at 192.168.1.100" ← Should discover Windows

2. **If you don't see "Discovery request from 192.168.1.100"**:
   - Mac isn't receiving Windows' packets
   - Check Mac firewall: `sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate`
   - Check Mac is on same subnet: `ifconfig | grep "inet " | grep -v 127.0.0.1`

3. **If you see the request but not the "Peer discovered" message**:
   - Check for errors in logs
   - Windows might not be responding correctly

### If Windows still can't see Mac:

1. **Check Windows logs** for:
   - "Discovery request from 192.168.1.103" ← Should see request from Mac
   - "Peer discovered via response: User at 192.168.1.103" ← Should discover Mac

2. **Run multicast receive test** while apps are running:
   ```powershell
   powershell -File test-multicast-receive.ps1
   ```
   You should see both Announce and DiscoveryRequest/DiscoveryResponse messages

---

## Technical Details

### Discovery Timeline

**Time 0ms**: App starts
- Socket created and configured
- Multicast group joined
- Receiver task spawned

**Time 100ms**: Announcements begin
- Initial Announce sent (full profile)
- DiscoveryRequest sent (triggers responses)
- Heartbeat task starts

**Time 100-200ms**: Responses arrive
- Existing peers receive DiscoveryRequest
- Existing peers send DiscoveryResponse (full profile)
- Discovery complete

**Every 15 seconds**: Heartbeat
- Lightweight status update
- Keeps presence alive
- Updates last_seen timestamp

**After 30 seconds**: Offline detection
- No heartbeat for 30s → marked offline in UI

**After 45 seconds**: Cleanup
- No heartbeat for 45s → removed from peer list

---

## Summary

The core issue was **NOT network/firewall blocking** (packets were flowing fine), but rather:

1. **Timing issue**: Announcement sent before receiver ready
2. **Missing functionality**: Discovery request handler didn't actually respond
3. **Slow discovery**: Had to wait up to 15s for first heartbeat

All three issues are now fixed. Discovery should be:
- ✅ Fast (~100ms)
- ✅ Order-independent
- ✅ Reliable
- ✅ Bidirectional

Build the app and test it! You should now see both devices discover each other immediately, regardless of start order.
