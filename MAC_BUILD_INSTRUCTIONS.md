# Mac Build Instructions - SO_REUSEPORT Fix

## What Was Fixed

The previous fix attempt used `socket.set_reuse_port()` which doesn't exist in the `socket2` crate on all platforms.

The new fix uses **low-level libc API** to set `SO_REUSEPORT` directly:

```rust
#[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
{
    use std::os::fd::AsRawFd;
    let optval: libc::c_int = 1;
    unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_REUSEPORT,
            &optval as *const _ as *const libc::c_void,
            std::mem::size_of_val(&optval) as libc::socklen_t,
        )
    };
}
```

This is the **correct way** to set SO_REUSEPORT on macOS/BSD/Linux systems.

---

## Files Changed

1. **crates/discovery/src/service.rs:111-133**
   - Uses libc to set SO_REUSEPORT directly
   - Only compiles on Unix platforms (not Windows)
   - Includes error handling with warning log

2. **crates/discovery/Cargo.toml:21-22**
   - Added `libc = "0.2"` as Unix-only dependency
   - Only linked on Unix targets (Mac, Linux)

---

## Build Instructions for Mac

### 1. Navigate to Project Directory
```bash
cd /Users/ayonsaha/Workspace/MyWorks/local-chat
```

### 2. Clean Build (Recommended)
```bash
cargo clean
cargo build
```

### 3. Run the App
```bash
npm run tauri dev
```

---

## Expected Build Output

You should see:
```
   Compiling lan-chat-discovery v0.1.0
   Compiling lan-chat-desktop v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

**No errors!**

---

## Expected Runtime Logs

When the Mac app starts, you should see:

```
Starting discovery service on port 37842
✓ Set SO_REUSEPORT for macOS multicast compatibility  ← NEW!
✓ Set multicast send interface to: 192.168.1.103
✓ Joined multicast group 239.255.42.99 on interface 192.168.1.103 for RECEIVING
Discovery receiver loop started, listening on multicast 239.255.42.99:37842
Sending announcement: User from 192.168.1.103
Sending discovery request to find existing peers
```

The key new line is:
**"✓ Set SO_REUSEPORT for macOS multicast compatibility"**

This confirms the socket option was set successfully.

---

## Testing After Build

### Test Scenario: Mac First, Windows Second

1. **Start Mac app**
   - Should see: "Starting discovery service..."
   - Should see: "✓ Set SO_REUSEPORT"

2. **Start Windows app** (2 seconds later)
   - Windows sends announcement
   - Windows sends discovery request

3. **Mac should receive Windows' packets** (THIS IS THE FIX!)
   - Mac logs should show: "📡 Discovery request from 192.168.1.100"
   - Mac logs should show: "✅ Peer discovered: User at 192.168.1.100"
   - Mac UI should show Windows user in contacts

4. **Verify bidirectional**
   - Windows should see Mac ✅ (already worked)
   - Mac should see Windows ✅ (THIS IS NEW!)

---

## Troubleshooting

### If Build Fails

**Error: "no method named `set_reuse_port`"**
- This error should be GONE with the new fix
- If you still see it, you might have cached old code
- Try: `cargo clean && cargo build`

**Error: "libc not found"**
- Shouldn't happen on Mac (libc is always available)
- If it does, add to Cargo.toml: `libc = "0.2"`

### If Mac Still Can't See Windows

1. **Check logs for SO_REUSEPORT line**
   ```
   ✓ Set SO_REUSEPORT for macOS multicast compatibility
   ```
   If you see this, the socket option was set successfully.

2. **Check for warning instead**
   ```
   ⚠ Failed to set SO_REUSEPORT (error XX)
   ```
   If you see this, the setsockopt call failed. Check errno.

3. **Check firewall**
   ```bash
   sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate
   ```
   Should be OFF or app should be excepted.

4. **Verify packets arrive with tcpdump**
   ```bash
   sudo tcpdump -i en0 -n udp port 37842
   ```
   You should see packets from both 192.168.1.100 and 192.168.1.103

---

## Why This Approach?

### Option 1: socket2.set_reuse_port() ❌
- Doesn't exist in socket2 v0.5
- Would need newer version or different crate

### Option 2: Raw libc setsockopt() ✅ (USED)
- Direct access to OS socket API
- Works on all Unix systems
- Minimal dependencies (just libc)
- Platform-specific compilation (#[cfg(unix)])

### Option 3: nix crate
- Would add another dependency
- Overkill for just one socket option

---

## Technical Details

### SO_REUSEPORT on macOS

From the macOS man page for setsockopt:

```
SO_REUSEPORT
    Allows completely duplicate bindings by multiple processes
    if they all set SO_REUSEPORT before binding the port.

    This option permits multiple instances of a program to each
    receive UDP/IP multicast or broadcast datagrams destined for
    the bound port.
```

**Why macOS needs this:**
- Without SO_REUSEPORT, macOS kernel delivers multicast packets to ONE socket
- With SO_REUSEPORT, macOS kernel delivers multicast packets to ALL matching sockets
- Windows doesn't have SO_REUSEPORT, uses just SO_REUSEADDR
- Linux added SO_REUSEPORT in kernel 3.9 (2013)

### Conditional Compilation

```rust
#[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
```

This ensures the code only compiles on:
- ✅ macOS
- ✅ Linux
- ✅ BSD variants
- ❌ Windows (doesn't have SO_REUSEPORT)
- ❌ Solaris/Illumos (different socket API)

---

## Commit Message (If Pushing to Git)

```
fix(discovery): add SO_REUSEPORT for macOS multicast reception

macOS requires SO_REUSEPORT socket option to properly receive
multicast packets. Without it, the kernel silently drops packets.

Uses libc::setsockopt directly since socket2 crate doesn't
expose set_reuse_port() on all platforms.

Fixes issue where Mac could send but not receive multicast
discovery packets, causing Windows peers to be invisible.

Platform-specific: Unix only (macOS, Linux, BSD)
```

---

## Summary

**Issue**: socket2 crate doesn't have `set_reuse_port()` method

**Fix**: Use libc `setsockopt()` directly with `SO_REUSEPORT` constant

**Result**: Mac should now receive multicast packets from Windows

**Action Required**: Rebuild on Mac with new code

Run these commands on Mac:
```bash
cd /Users/ayonsaha/Workspace/MyWorks/local-chat
cargo clean
cargo build
npm run tauri dev
```

Then test if Mac can now see Windows!
