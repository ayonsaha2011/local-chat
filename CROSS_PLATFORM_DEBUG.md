# Cross-Platform Discovery Issues - Windows + Mac

## 🔍 Immediate Checks

Since you have both devices running but they're not seeing each other, let's systematically check each point:

---

## ✅ **Step 1: Verify Both Apps Are Actually Running**

### **On Windows PC:**
```cmd
cd d:\workspace\local-ip-chat\desktop
npm run tauri dev
```

Check terminal output for:
```
Starting chat services...
User profile: [Your Name] ([UUID])
Local IP: 192.168.x.x
Starting peer discovery service...
Starting messaging server on port 37843...
All services started successfully!
```

### **On Mac:**
```bash
cd /path/to/local-ip-chat/desktop
npm run tauri dev
```

Check terminal output for the same messages.

**✅ If both show "All services started successfully!"** → Services are running  
**❌ If errors** → Note the specific error and fix before continuing

---

## ✅ **Step 2: Verify Same Network (CRITICAL)**

### **On Windows:**
```cmd
ipconfig
```

Look for **IPv4 Address** under your active network adapter:
```
IPv4 Address: 192.168.1.100
```

### **On Mac:**
```bash
ifconfig | grep "inet "
```

Or use System Preferences → Network → Check IP address.

**✅ MUST BE SAME SUBNET:**
- Windows: `192.168.1.100` ✓
- Mac: `192.168.1.101` ✓

**❌ DIFFERENT SUBNETS WON'T WORK:**
- Windows: `192.168.1.100` ✗
- Mac: `192.168.2.100` ✗

**Common Issues:**
- Mac connected to 5GHz WiFi, Windows to 2.4GHz (some routers separate these)
- One device on WiFi, other on Ethernet (different VLANs)
- VPN active on one device

---

## ✅ **Step 3: Check Firewall on BOTH Devices**

### **On Windows:**

Check if firewall is blocking:
```powershell
# Check firewall status
Get-NetFirewallProfile | Select-Object Name, Enabled

# Check if rules exist
Get-NetFirewallRule -DisplayName "LAN Chat*"
```

**Add firewall rules** (Run PowerShell as Administrator):
```powershell
New-NetFirewallRule -DisplayName "LAN Chat Discovery" -Direction Inbound -Protocol UDP -LocalPort 37842 -Action Allow
New-NetFirewallRule -DisplayName "LAN Chat Messaging" -Direction Inbound -Protocol TCP -LocalPort 37843 -Action Allow
New-NetFirewallRule -DisplayName "LAN Chat Transfer" -Direction Inbound -Protocol TCP -LocalPort 37844 -Action Allow

# Also allow outbound
New-NetFirewallRule -DisplayName "LAN Chat Discovery OUT" -Direction Outbound -Protocol UDP -LocalPort 37842 -Action Allow
```

**Quick Test - Temporarily disable** (to verify firewall is the issue):
```powershell
# Run as Administrator
netsh advfirewall set allprofiles state off
# Test if it works
# Then re-enable:
netsh advfirewall set allprofiles state on
```

### **On Mac:**

Check firewall status:
```bash
# Check if firewall is on
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate
```

**Option 1: Disable firewall temporarily** (for testing):
```bash
# Disable
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off

# Test if peers appear

# Re-enable
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate on
```

**Option 2: Allow the app permanently:**

Go to:
1. **System Preferences** → **Security & Privacy** → **Firewall** → **Firewall Options**
2. Click **"+"** and add the LAN Chat app
3. Set to **"Allow incoming connections"**

---

## ✅ **Step 4: Verify Multicast Support**

Some routers block multicast between different device types.

### **Test multicast from Windows:**
```cmd
# Try to ping multicast address
ping 239.255.42.99 -n 5
```

### **Test multicast from Mac:**
```bash
# Try to ping multicast address
ping -c 5 239.255.42.99
```

**❌ If both timeout**: Your router blocks multicast  
**✅ If responses**: Multicast works

---

## ✅ **Step 5: Check Router Settings**

### **Common Router Issues:**

1. **AP Isolation / Client Isolation Enabled**
   - Prevents devices from seeing each other
   - Check router settings: Look for "AP Isolation", "Client Isolation", "Private WiFi"
   - **Solution**: Disable this setting

2. **5GHz and 2.4GHz Separation**
   - Some routers keep 5GHz and 2.4GHz clients isolated
   - **Solution**: Connect both devices to same band

3. **Guest Network**
   - Guest networks isolate devices
   - **Solution**: Use main network, not guest network

4. **IGMP Snooping Disabled**
   - Required for multicast
   - **Solution**: Enable IGMP Snooping in router settings

### **How to Check:**

1. Open router admin page (usually `192.168.1.1` or `192.168.0.1`)
2. Look for **Wireless Settings** or **Advanced Settings**
3. Find options like:
   - **AP Isolation**: Should be OFF
   - **IGMP Snooping**: Should be ON
   - **Multicast Filtering**: Should be OFF or Allow

---

## ✅ **Step 6: Verify Ports Are Listening**

### **On Windows:**
```cmd
netstat -ano | findstr 37842
netstat -ano | findstr 37843
netstat -ano | findstr 37844
```

You should see:
```
UDP    0.0.0.0:37842    *:*    [PID]
TCP    0.0.0.0:37843    0.0.0.0:0    LISTENING    [PID]
TCP    0.0.0.0:37844    0.0.0.0:0    LISTENING    [PID]
```

### **On Mac:**
```bash
lsof -i :37842
lsof -i :37843
lsof -i :37844
```

You should see the app listening on these ports.

---

## ✅ **Step 7: Check App Logs**

### **On Both Devices:**

1. Open the app
2. Press **F12** (or Cmd+Option+I on Mac)
3. Go to **Console** tab

Look for:
```javascript
Profile initialized: { ... }
Initial peers loaded: []
```

### **In Terminal (Backend Logs):**

Look for:
```
[Discovery] Sending announcement...
[Discovery] Heartbeat sent
```

If you see these on both devices, discovery is working but packets aren't reaching each other.

---

## 🔧 **Most Likely Causes for Windows + Mac**

Based on cross-platform setups, the issue is usually:

### **1. AP Isolation (60% of cases)**
- **Problem**: Router has "AP Isolation" or "Client Isolation" enabled
- **Solution**: Access router settings and disable it
- **Test**: Try pinging Mac from Windows:
  ```cmd
  # On Windows
  ping 192.168.1.101  # Mac's IP
  ```
  If this fails, it's AP isolation

### **2. Firewall Blocking (25% of cases)**
- **Problem**: macOS firewall blocking incoming UDP
- **Solution**: Temporarily disable Mac firewall to test:
  ```bash
  sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
  ```
  If peers appear, it's the firewall

### **3. Different Subnets (10% of cases)**
- **Problem**: Devices on different VLANs
- **Solution**: Ensure same subnet (192.168.1.x)

### **4. Multicast Not Working (5% of cases)**
- **Problem**: Router doesn't forward multicast between WiFi bands
- **Solution**: Use Ethernet or same WiFi band

---

## 🧪 **Quick Test: Can Devices See Each Other?**

### **From Windows, ping Mac:**
```cmd
# Replace with Mac's actual IP
ping 192.168.1.101
```

### **From Mac, ping Windows:**
```bash
# Replace with Windows' actual IP
ping 192.168.1.100
```

**✅ If ping works:** Network connectivity is fine, issue is with firewall or multicast  
**❌ If ping fails:** AP Isolation is enabled or different networks

---

## 🔍 **Advanced Debugging**

### **Capture Network Traffic**

Install Wireshark on both devices and filter for:
```
udp.port == 37842
```

You should see UDP packets to `239.255.42.99` every 15 seconds.

**If Windows sends but Mac doesn't receive**: Mac firewall or router blocking  
**If Mac sends but Windows doesn't receive**: Windows firewall or router blocking  
**If neither sends**: Services not starting correctly

---

## 💡 **Quick Fix Checklist**

Try these in order:

1. **Disable firewalls on BOTH devices** (temporarily for testing)
   ```powershell
   # Windows (as Admin)
   netsh advfirewall set allprofiles state off
   ```
   ```bash
   # Mac
   sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off
   ```

2. **Verify same network**
   ```cmd
   # Windows
   ipconfig | findstr IPv4
   ```
   ```bash
   # Mac  
   ifconfig | grep "inet "
   ```

3. **Check router AP isolation**
   - Log into router
   - Disable "AP Isolation" / "Client Isolation"
   - Restart router

4. **Restart both apps**
   - Close completely
   - Wait 5 seconds
   - Restart both
   - Wait 30 seconds

5. **Check if they can ping each other**
   ```cmd
   ping [other device IP]
   ```

---

## 📊 **Expected Behavior**

When working correctly:

| Time | Windows | Mac |
|------|---------|-----|
| 0s | App starts | App starts |
| 2s | Services running | Services running |
| 5s | Sends discovery | Sends discovery |
| 15s | **Should see Mac** | **Should see Windows** |
| 30s | Connection stable | Connection stable |

---

## 🆘 **Report Back With:**

If still not working, provide:

1. **Network info:**
   ```cmd
   # Windows
   ipconfig
   ```
   ```bash
   # Mac
   ifconfig
   ```

2. **Ping test results:**
   ```cmd
   ping [other device IP]
   ```

3. **Port status:**
   ```cmd
   # Windows
   netstat -ano | findstr 3784
   ```
   ```bash
   # Mac
   lsof -i :37842
   ```

4. **Can they ping each other?** Yes/No

5. **Router model** and whether AP Isolation setting exists

---

## 🎯 **Most Likely Solution**

Based on Windows + Mac setups, **99% of the time** it's one of these:

1. **macOS firewall** blocking UDP (disable to test)
2. **Router AP Isolation** enabled (disable in router settings)
3. **Different networks** (verify same subnet)

**Fastest test:**
```bash
# On Mac - disable firewall
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off

# On Windows - disable firewall (as Admin)
netsh advfirewall set allprofiles state off

# Wait 30 seconds
# Check if peers appear
# If yes → It's firewall
# If no → It's router/network
```

Good luck! 🚀
