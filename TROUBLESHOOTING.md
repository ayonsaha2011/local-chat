# Troubleshooting Guide

## 🔍 App Not Showing Contacts

If you're not seeing any contacts in the sidebar, here are the common causes and solutions:

### **Root Cause**
The contacts list will be empty because:
1. **No other peers are running** - You need at least 2 instances of the app on the same network
2. **Services not started** - Backend services need time to initialize
3. **Firewall blocking** - Windows Firewall may block UDP/TCP ports
4. **Network isolation** - Devices might be on different subnets

---

## ✅ **Quick Fixes**

### **1. Run Multiple Instances**

The app discovers peers on the **same local network**. To test:

#### **Option A: Two Computers**
- Install and run the app on two different computers on the same WiFi/LAN
- Both should see each other within 15-30 seconds

#### **Option B: Same Computer (Testing)**
You can run multiple instances on the same computer:

```bash
# Terminal 1 - First instance
cd d:\workspace\local-ip-chat\desktop
npm run tauri dev

# Terminal 2 - Second instance (different profile needed)
# Note: This requires code changes to support different ports
```

**Important**: Running multiple instances on the same machine requires modifying the code to use different ports for each instance.

---

### **2. Check Firewall Settings**

Windows Firewall might be blocking the required ports.

#### **Allow Ports in Windows Firewall**

Run PowerShell as **Administrator**:

```powershell
# Allow UDP discovery (port 37842)
New-NetFirewallRule -DisplayName "LAN Chat Discovery" -Direction Inbound -Protocol UDP -LocalPort 37842 -Action Allow

# Allow TCP messaging (port 37843)
New-NetFirewallRule -DisplayName "LAN Chat Messaging" -Direction Inbound -Protocol TCP -LocalPort 37843 -Action Allow

# Allow TCP file transfer (port 37844)
New-NetFirewallRule -DisplayName "LAN Chat Transfer" -Direction Inbound -Protocol TCP -LocalPort 37844 -Action Allow
```

Or manually:
1. Open **Windows Defender Firewall**
2. Click **"Advanced settings"**
3. Click **"Inbound Rules"** → **"New Rule"**
4. Select **"Port"** → Next
5. Select **"UDP"** and enter **37842** → Next
6. Select **"Allow the connection"** → Next
7. Check all profiles → Next
8. Name it **"LAN Chat Discovery"** → Finish
9. Repeat for TCP ports 37843 and 37844

---

### **3. Check Network Configuration**

#### **Verify Same Network**
Both devices must be on the **same subnet**:

```cmd
ipconfig
```

Look for the **IPv4 Address** - both devices should have addresses like:
- ✅ `192.168.1.100` and `192.168.1.101` (same subnet)
- ❌ `192.168.1.100` and `192.168.2.100` (different subnets)

#### **Test Multicast Support**
Some networks don't support UDP multicast:

```cmd
# Ping multicast address
ping 239.255.42.99
```

If this fails, your router might not support multicast.

---

### **4. Check Application Logs**

The app logs to the console. Check for errors:

#### **In Development Mode**
```bash
npm run tauri dev
```

Look for these messages in the console:
```
Starting chat services...
User profile: YourName (uuid)
Local IP: 192.168.1.x
Starting peer discovery service...
Starting messaging server on port 37843...
Starting file transfer service...
All services started successfully!
```

If you see errors, they'll indicate the problem.

---

### **5. Verify Services Started**

After initializing with your username/display name, check the browser console (F12):

```
Profile initialized: { user_id: "...", username: "...", ... }
Initial peers loaded: []  // Empty at first is normal
```

Peers will appear as `peer-discovered` events are received.

---

## 🧪 **Testing Discovery**

### **Test on Same Computer**

For testing purposes, you can modify the code to run multiple instances on different ports:

**File**: `crates/discovery/src/protocol.rs`

Change the discovery port for second instance:
```rust
pub const DISCOVERY_PORT: u16 = 37842;  // First instance
// pub const DISCOVERY_PORT: u16 = 37852;  // Second instance (uncomment)
```

Then rebuild and run.

---

## 🔧 **Common Issues**

### **Issue: "No peers discovered yet"**

**Causes:**
- Only one instance running
- Firewall blocking ports
- Different networks/VLANs
- VPN interfering

**Solutions:**
1. Run second instance on another device
2. Disable firewall temporarily to test
3. Ensure same network
4. Disable VPN

---

### **Issue: Services fail to start**

**Error**: "Discovery service error" or "Messaging server error"

**Causes:**
- Port already in use
- No network connectivity
- Insufficient permissions

**Solutions:**
```cmd
# Check if ports are in use
netstat -ano | findstr 37842
netstat -ano | findstr 37843
netstat -ano | findstr 37844

# If ports are busy, close the application using them
# Or restart your computer
```

---

### **Issue: Can't connect to peers**

**Symptom**: Peers appear but can't send messages

**Causes:**
- TCP port 37843 blocked
- Encryption key mismatch
- Network timeout

**Solutions:**
1. Check firewall allows TCP 37843
2. Restart both applications
3. Check network latency

---

## 📊 **Expected Behavior**

### **Timeline**
1. **0s** - App starts, services initialize
2. **1-2s** - Profile created, services running
3. **0-15s** - First discovery broadcast
4. **15-30s** - Other peers should appear
5. **30s+** - Heartbeats maintain presence

### **What Should Happen**
1. Open app → See welcome screen
2. Enter username/display name → Services start
3. Wait 15-30 seconds → Peers appear in sidebar
4. Click peer → Can send messages
5. Messages are encrypted automatically

---

## 🛠️ **Advanced Debugging**

### **Enable Verbose Logging**

The app already has logging enabled. To see more details:

1. Run in development mode: `npm run tauri dev`
2. Open DevTools (F12)
3. Check both console tabs (Frontend and Rust backend)

### **Check Rust Logs**

In the terminal running `npm run tauri dev`, you'll see:
```
[2024-xx-xx] INFO Starting chat services...
[2024-xx-xx] INFO User profile: Alice (uuid)
[2024-xx-xx] INFO Local IP: 192.168.1.100
[2024-xx-xx] INFO Starting peer discovery service...
[2024-xx-xx] INFO Peer discovered: Bob
```

### **Network Packet Capture**

Use Wireshark to verify multicast packets:
1. Install Wireshark
2. Filter: `udp.port == 37842`
3. Should see periodic UDP packets to `239.255.42.99`

---

## 💡 **Quick Test Checklist**

- [ ] App is running on at least 2 devices
- [ ] Both devices are on the same network (same subnet)
- [ ] Firewall allows UDP 37842, TCP 37843, TCP 37844
- [ ] No VPN is active
- [ ] Wait at least 30 seconds after starting
- [ ] Check console for errors
- [ ] Services show "started successfully"

---

## 🆘 **Still Not Working?**

If you've tried everything:

1. **Check the logs** for specific error messages
2. **Restart the application** on both devices
3. **Restart your router** (to refresh multicast routing)
4. **Try a simple network** (direct WiFi, not corporate/school network)
5. **Test with just 2 devices** first before scaling up

---

## 📝 **Report an Issue**

If you find a bug, please report with:
- Operating system and version
- Network configuration (home WiFi, corporate, etc.)
- Error messages from logs
- Steps to reproduce

---

**Remember**: The app works on **local networks only**. You need at least 2 devices running the app on the same network to see contacts appear! 🌐
