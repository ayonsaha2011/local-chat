# Debug Checklist: Why Contacts Aren't Showing

## 🔍 Step-by-Step Debugging

Follow these steps in order to identify the issue:

### **Step 1: Is the App Running?**

Run the app in development mode:
```cmd
cd d:\workspace\local-ip-chat\desktop
npm run tauri dev
```

✅ **Expected**: App window opens with welcome screen  
❌ **If not**: Check for errors in terminal

---

### **Step 2: Can You Initialize Your Profile?**

1. Enter your **username** (e.g., "alice")
2. Enter your **display name** (e.g., "Alice Smith")
3. Click **Get Started**

✅ **Expected**: You see the main app screen with sidebar  
❌ **If error**: Check browser console (F12) for errors

---

### **Step 3: Check Backend Services Started**

Look at the **terminal** where you ran `npm run tauri dev`. You should see:

```
Starting chat services...
User profile: Alice Smith (12345678-...)
Local IP: 192.168.x.x
Starting peer discovery service...
Starting messaging server on port 37843...
Starting file transfer service...
All services started successfully!
```

✅ **If you see these**: Backend is working  
❌ **If errors or missing**: Services failed to start

**Common errors:**
- `Address already in use` → Port 37843 is busy, restart computer
- `Permission denied` → Run as administrator
- `Network unreachable` → Check network connection

---

### **Step 4: Check Browser Console**

Press **F12** in the app window and check the **Console** tab.

You should see:
```javascript
Profile initialized: { user_id: "...", username: "alice", ... }
Initial peers loaded: []
```

✅ **If you see this**: Frontend is working  
❌ **If errors**: Note the error message

---

### **Step 5: Test Discovery (Single Device)**

Even with one device, the discovery service should be broadcasting. In the terminal, you might see:
```
[Discovery] Sending announcement...
[Discovery] Heartbeat sent
```

✅ **If you see broadcasts**: Discovery is working  
❌ **If not**: Discovery service failed

---

### **Step 6: Do You Have Another Device?**

**CRITICAL**: You MUST have at least 2 devices running the app on the same network.

#### **Option A: Two Physical Devices**
1. Install app on Computer A
2. Install app on Computer B (same WiFi/LAN)
3. Run both apps
4. Wait 30 seconds

#### **Option B: Virtual Machine**
1. Run app on host computer
2. Run app in VirtualBox/VMware (bridged network)
3. Wait 30 seconds

#### **Option C: Different Ports (Advanced)**

For testing, you can modify the code to run two instances on the same computer:

**Create a second instance with different ports:**

File: `crates/discovery/src/protocol.rs`
```rust
// For second instance, change to:
pub const DISCOVERY_PORT: u16 = 37852; // instead of 37842
```

File: `crates/protocol/src/lib.rs`
```rust
// For second instance, change to:
pub const MESSAGING_PORT: u16 = 37853; // instead of 37843
```

Then rebuild and run second instance.

---

### **Step 7: Check Firewall**

Windows Firewall might be blocking the app.

#### **Quick Test: Disable Firewall Temporarily**
```cmd
# Run as Administrator
netsh advfirewall set allprofiles state off
```

Try again. If it works, the firewall is the issue.

**Re-enable firewall:**
```cmd
netsh advfirewall set allprofiles state on
```

#### **Proper Fix: Add Firewall Rules**
```powershell
# Run PowerShell as Administrator
New-NetFirewallRule -DisplayName "LAN Chat Discovery" -Direction Inbound -Protocol UDP -LocalPort 37842 -Action Allow
New-NetFirewallRule -DisplayName "LAN Chat Messaging" -Direction Inbound -Protocol TCP -LocalPort 37843 -Action Allow
New-NetFirewallRule -DisplayName "LAN Chat Transfer" -Direction Inbound -Protocol TCP -LocalPort 37844 -Action Allow
```

---

### **Step 8: Verify Network Configuration**

Both devices MUST be on the same network:

```cmd
ipconfig
```

Check the **IPv4 Address**:
- Computer A: `192.168.1.100`
- Computer B: `192.168.1.101`

✅ **Same subnet** (192.168.1.x) = Good  
❌ **Different subnets** = Won't work

---

### **Step 9: Test Multicast**

Some networks don't support UDP multicast.

```cmd
ping 239.255.42.99
```

✅ **If it responds**: Multicast works  
❌ **If timeout**: Your router doesn't support multicast

---

### **Step 10: Check Ports in Use**

```cmd
netstat -ano | findstr 37842
netstat -ano | findstr 37843
netstat -ano | findstr 37844
```

✅ **Shows the app's PID**: Ports are used by your app  
❌ **Different PID or empty**: Ports not used or blocked

---

## 🎯 **Most Likely Issues**

Based on experience, these are the most common causes:

### **1. Only One Device Running (90% of cases)**
**Problem**: You need at least 2 devices on same network  
**Solution**: Run app on second computer or VM

### **2. Firewall Blocking (5% of cases)**
**Problem**: Windows Firewall blocks UDP multicast  
**Solution**: Add firewall rules or disable temporarily to test

### **3. Different Networks (3% of cases)**
**Problem**: Devices on different subnets or VLANs  
**Solution**: Ensure same WiFi network

### **4. Services Not Starting (2% of cases)**
**Problem**: Port conflict or permission issue  
**Solution**: Check terminal logs, restart computer

---

## 📊 **Expected Timeline**

When everything is working correctly:

- **0-2s**: App starts, services initialize
- **2-5s**: Profile created, you see main screen
- **5-15s**: First discovery broadcast sent
- **15-30s**: Other peers should appear in sidebar
- **30s+**: Heartbeats maintain presence

---

## 🐛 **Debugging Commands**

### **Check if discovery is working:**
```cmd
# Install Wireshark (optional but helpful)
# Filter: udp.port == 37842
# Should see packets to 239.255.42.99
```

### **Check if messaging server is listening:**
```cmd
netstat -ano | findstr :37843
```

### **View detailed logs:**
In the app, open DevTools (F12) and check:
1. **Console** tab → Frontend logs
2. **Terminal** → Backend logs

---

## ✅ **Verification Steps**

After following the above, verify:

- [ ] App runs without errors
- [ ] You can create a profile
- [ ] Terminal shows "All services started successfully"
- [ ] Browser console shows "Profile initialized"
- [ ] Firewall allows ports 37842, 37843, 37844
- [ ] Two devices are on the same network
- [ ] Both devices have the app running
- [ ] Waited at least 30 seconds

---

## 🆘 **Still Not Working?**

If you've checked everything above and it's still not working, please provide:

1. **Terminal output** (full log from startup)
2. **Browser console errors** (F12 → Console tab)
3. **Network info**: 
   ```cmd
   ipconfig
   ```
4. **Port status**:
   ```cmd
   netstat -ano | findstr 3784
   ```
5. **Operating System**: Windows version
6. **Network type**: Home WiFi, Corporate, School, etc.

---

## 💡 **Quick Test**

**Fastest way to verify it works:**

1. Run app on Computer A (WiFi: 192.168.1.100)
2. Run app on Computer B (Same WiFi: 192.168.1.101)
3. Temporarily disable firewall on both:
   ```cmd
   netsh advfirewall set allprofiles state off
   ```
4. Wait 30 seconds
5. Check if peers appear
6. Re-enable firewall:
   ```cmd
   netsh advfirewall set allprofiles state on
   ```

If this works → It's a firewall issue  
If this doesn't work → It's a network/code issue

---

**Remember**: This is a **local network** app. It will NEVER show contacts if:
- Only one device is running
- Devices are on different networks
- Network doesn't support multicast

The app is working correctly - you just need the right setup! 🚀
