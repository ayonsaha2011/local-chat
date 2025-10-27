# CRITICAL: Multicast Not Working

## Problem Found

Your logs show the app is working perfectly, but multicast ping test **FAILED**:
```
ping -n 3 239.255.42.99
Request timed out. (100% packet loss)
```

This means **multicast traffic is being blocked** at the network level, NOT by the app.

## Root Cause

One of these issues is blocking multicast:

1. **Windows Network Profile is "Public"** (blocks multicast by default)
2. **Router has AP Isolation enabled** (prevents device-to-device communication)
3. **Router IGMP Snooping is disabled** (needed for multicast)
4. **Firewall rules not applying to the correct network profile**

## FIX #1: Change Network Profile from Public to Private

**THIS IS THE MOST LIKELY FIX**

### Check Current Profile:
```powershell
Get-NetConnectionProfile
```

If `NetworkCategory` shows **"Public"**, change it:

```powershell
Get-NetConnectionProfile | Set-NetConnectionProfile -NetworkCategory Private
```

Then **restart the app** and test again.

### Why This Matters:
- Windows blocks multicast on "Public" networks by default for security
- Firewall rules may not apply to "Public" networks
- Changing to "Private" allows local network discovery

---

## FIX #2: Update Firewall Rules to Include Public Network

If you can't change to Private, update firewall rules:

```cmd
netsh advfirewall firewall delete rule name="LAN Chat Discovery (UDP In)"
netsh advfirewall firewall delete rule name="LAN Chat Discovery (UDP Out)"

netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP In)" dir=in action=allow protocol=UDP localport=37842 profile=any
netsh advfirewall firewall add rule name="LAN Chat Discovery (UDP Out)" dir=out action=allow protocol=UDP localport=37842 profile=any
```

The `profile=any` makes rules work on both Private AND Public networks.

---

## FIX #3: Check Router Settings

### Disable AP Isolation:
1. Open router admin (usually http://192.168.1.1)
2. Login with admin credentials
3. Look for:
   - "AP Isolation" → **DISABLE**
   - "Client Isolation" → **DISABLE**
   - "Wireless Isolation" → **DISABLE**
4. Save and reboot router

### Enable IGMP Snooping:
1. Find "Multicast" or "IGMP" settings
2. Enable "IGMP Snooping"
3. Save and reboot router

### Common Router Admin URLs:
- TP-Link: http://tplinkwifi.net or http://192.168.0.1
- Netgear: http://routerlogin.net or http://192.168.1.1
- Linksys: http://192.168.1.1
- ASUS: http://router.asus.com or http://192.168.1.1
- D-Link: http://192.168.0.1

---

## Test Multicast After Fix

After applying fixes, test multicast:

```cmd
ping -n 5 239.255.42.99
```

You should see **replies** (not timeouts).

If still timing out, run on BOTH devices simultaneously:

**Device 1 (Windows):**
```powershell
powershell -File test-multicast-receive.ps1
```

**Device 2 (Mac):**
```bash
nc -u -l 37842
```

Then from Windows:
```powershell
powershell -File test-multicast-send.ps1
```

If Mac receives packets, multicast is working!

---

## Step-by-Step Checklist

On **Windows PC**:
- [ ] Check network profile: `Get-NetConnectionProfile`
- [ ] If Public, change to Private: `Get-NetConnectionProfile | Set-NetConnectionProfile -NetworkCategory Private`
- [ ] Verify firewall rules include `profile=any`
- [ ] Test multicast: `ping 239.255.42.99`

On **Mac**:
- [ ] Disable firewall: `sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off`
- [ ] Test multicast: `ping -c 5 239.255.42.99`

On **Router**:
- [ ] Disable AP Isolation
- [ ] Enable IGMP Snooping
- [ ] Reboot router

Then:
- [ ] Restart both apps
- [ ] Check if devices now see each other

---

## If Still Not Working: Wireshark Test

1. Download Wireshark: https://www.wireshark.org/download.html
2. Start capture on your WiFi/Ethernet adapter
3. Filter: `ip.dst == 239.255.42.99 and udp.port == 37842`
4. Start the app
5. You should see packets every 15 seconds

**If you see NO packets**: Windows is blocking at a lower level (driver/network adapter)
**If you see packets from YOUR device but not the other**: Router is blocking

---

## Alternative: Mobile Hotspot Test

To confirm it's a router issue:

1. Enable Mobile Hotspot on one device
2. Connect both devices to the hotspot
3. Run the app
4. If devices can now see each other → **Router is the problem**

---

## Network Adapter Settings Check

Sometimes network adapters block multicast:

```powershell
# Show network adapters
Get-NetAdapter | Format-Table Name, Status, LinkSpeed

# Enable multicast on adapter (replace "Wi-Fi" with your adapter name)
netsh interface ipv4 set interface "Wi-Fi" mcastdiscovery=enabled
```

---

## Summary

**Your app code is working perfectly!** The issue is:
- ✅ App detects correct IP (192.168.1.100)
- ✅ App joins multicast group successfully
- ✅ App sends announcements
- ❌ **Multicast packets are blocked by Windows or router**

**Most likely fix**: Change network from Public to Private
**Second most likely**: Router AP Isolation is enabled

Try the fixes above and let me know the results!
