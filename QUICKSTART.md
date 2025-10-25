# Quick Start Guide

## Installation & First Run

### Option 1: Download Pre-built Binary (Recommended)

*Once released, binaries will be available on the GitHub releases page.*

### Option 2: Build from Source

#### 1. Install Prerequisites

**Windows:**
```powershell
# Install Rust
winget install Rustlang.Rustup

# Install Node.js
winget install OpenJS.NodeJS

# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

**macOS:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (using Homebrew)
brew install node

# Install Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install dependencies
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential curl wget file \
    libssl-dev libgtk-3-dev \
    libayatana-appindicator3-dev librsvg2-dev
```

#### 2. Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd local-ip-chat

# Navigate to desktop directory
cd desktop

# Install npm dependencies
npm install

# Run in development mode
npm run tauri dev

# OR build for production
npm run tauri build
```

## First Time Setup

### 1. Launch the Application

When you first launch LAN Chat, you'll see a welcome screen.

### 2. Create Your Profile

Enter your:
- **Username**: A unique identifier (e.g., `johndoe`)
- **Display Name**: Your visible name (e.g., `John Doe`)

Click **Get Started** to continue.

### 3. Discover Peers

The app will automatically:
- Start broadcasting your presence on the local network
- Discover other users running LAN Chat
- Display discovered peers in the sidebar

### 4. Start Chatting

1. Click on a peer from the sidebar
2. Type your message in the input field
3. Press Enter or click the Send button
4. Your messages are automatically encrypted!

## Features Overview

### 💬 Messaging

- **Real-time**: Messages appear instantly
- **Encrypted**: All messages use end-to-end encryption
- **Receipts**: See when messages are delivered and read
- **Status**: Know when someone is typing

### 📁 File Sharing

1. Click the paperclip icon in the chat
2. Select a file to send
3. Recipient receives a transfer request
4. After acceptance, file transfers directly
5. Progress bar shows transfer status

### 👥 Contacts

- **Auto-discovery**: Peers appear automatically
- **Status indicators**: See who's online, away, or busy
- **Profile info**: View username and network address

### ⚙️ Settings

- **Dark Mode**: Toggle between light and dark themes
- **Status**: Change your availability status
- **Profile**: Update your display name and status message

## Network Configuration

### Firewall Settings

LAN Chat uses these ports:

| Port | Protocol | Purpose |
|------|----------|---------|
| 37842 | UDP | Peer discovery |
| 37843 | TCP | Messaging |
| 37844 | TCP | File transfers |

**Allow these ports in your firewall:**

**Windows Firewall:**
```powershell
New-NetFirewallRule -DisplayName "LAN Chat" -Direction Inbound -Program "path\to\lan-chat.exe" -Action Allow
```

**macOS:**
System Preferences → Security & Privacy → Firewall → Firewall Options → Add application

**Linux (ufw):**
```bash
sudo ufw allow 37842/udp
sudo ufw allow 37843/tcp
sudo ufw allow 37844/tcp
```

### Network Requirements

- **Same network**: All users must be on the same local network
- **Multicast support**: Network must support UDP multicast
- **No VPN conflicts**: Some VPNs may block local discovery

## Troubleshooting

### No Peers Discovered

**Problem**: Sidebar shows "No peers discovered yet"

**Solutions:**
1. Check firewall settings (see above)
2. Ensure you're on the same network
3. Try disabling VPN temporarily
4. Verify network supports multicast
5. Check if antivirus is blocking the app

### Messages Not Sending

**Problem**: Messages show as "Failed" status

**Solutions:**
1. Check network connection
2. Verify peer is still online
3. Restart the application
4. Check firewall allows TCP port 37843

### File Transfer Fails

**Problem**: File transfer shows error or stuck

**Solutions:**
1. Check available disk space
2. Verify firewall allows TCP port 37844
3. Try smaller files first
4. Check file permissions
5. Ensure stable network connection

### Connection Issues on Windows

**Problem**: "Network error" messages

**Solutions:**
1. Run as Administrator (first time only)
2. Allow app through Windows Defender
3. Disable "Public network" restrictions
4. Check Windows Firewall settings

### Dark Mode Not Working

**Problem**: Theme doesn't change

**Solution**: Click the moon/sun icon in the sidebar header

## Tips & Best Practices

### Security

✅ **Do:**
- Keep the app updated
- Use on trusted networks only
- Verify peer identities
- Review file transfer requests

❌ **Don't:**
- Use on public WiFi
- Accept files from unknown users
- Share sensitive info over untrusted networks

### Performance

- **Large files**: Use file transfer for files > 10MB
- **Many peers**: App is optimized for 10-50 simultaneous users
- **Network load**: File transfers use direct connections (minimal network impact)

### Privacy

- **No servers**: All communication is peer-to-peer
- **No logging**: Messages are not stored on disk (by default)
- **No tracking**: No analytics or telemetry
- **Local only**: Nothing leaves your local network

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + K` | Search contacts |
| `Ctrl/Cmd + N` | New message |
| `Ctrl/Cmd + ,` | Open settings |
| `Ctrl/Cmd + D` | Toggle dark mode |
| `Esc` | Close current chat |
| `Enter` | Send message |
| `Shift + Enter` | New line in message |

## Advanced Usage

### Multiple Instances

You can run multiple instances on different networks, but:
- Each instance needs a unique profile
- They won't interfere with each other
- Use different network interfaces if available

### Custom Network Setup

For advanced users, you can modify the source code to:
- Change default ports
- Adjust multicast addresses
- Configure encryption parameters
- Customize UI themes

See `API.md` for technical details.

## Getting Help

### Documentation

- 📖 **README.md**: Project overview
- 🔧 **BUILD.md**: Detailed build instructions
- 📚 **API.md**: Technical API reference
- 🚀 **QUICKSTART.md**: This guide

### Support Channels

- 🐛 **Bug Reports**: Open an issue on GitHub
- 💡 **Feature Requests**: GitHub Discussions
- 💬 **Questions**: GitHub Discussions or Issues
- 📧 **Security**: Email security contact (see README)

## What's Next?

After getting started:

1. **Invite colleagues**: Share the app with your team
2. **Customize**: Adjust settings to your preference
3. **Provide feedback**: Help improve the app
4. **Contribute**: Check out the source code
5. **Stay updated**: Watch the repository for updates

---

**Enjoy secure, private messaging on your local network! 🚀**
