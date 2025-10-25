# Setup Instructions

## Quick Setup

To complete the setup and build the application, follow these steps:

### 1. Install Frontend Dependencies

```bash
cd desktop
npm install
```

This will install all React, TypeScript, and Tailwind CSS dependencies.

### 2. Add Application Icons

The Tauri build requires icon files. You have two options:

#### Option A: Use Tauri's Default Icons (Quick Start)

```bash
cd desktop/src-tauri
mkdir -p icons
# Download or copy placeholder icons
```

Create the following icon files in `desktop/src-tauri/icons/`:
- `32x32.png` - 32x32 pixels
- `128x128.png` - 128x128 pixels
- `128x128@2x.png` - 256x256 pixels
- `icon.icns` - macOS icon
- `icon.ico` - Windows icon

#### Option B: Generate Icons from a Single Image

If you have a source image (PNG, at least 512x512):

```bash
# Install Tauri CLI globally
npm install -g @tauri-apps/cli

# Generate icons
cd desktop/src-tauri
tauri icon path/to/your/icon.png
```

### 3. Build the Application

#### Development Mode (Hot Reload)
```bash
cd desktop
npm run tauri dev
```

#### Production Build
```bash
cd desktop
npm run tauri build
```

### 4. Testing Core Libraries

```bash
# From the project root
cargo test --workspace

# Run specific tests
cargo test -p lan-chat-core
cargo test -p lan-chat-crypto
```

## Platform-Specific Notes

### Windows

1. Install dependencies:
```powershell
# PowerShell as Administrator
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
winget install Rustlang.Rustup
winget install OpenJS.NodeJS
winget install Microsoft.VisualStudio.2022.BuildTools
```

2. Allow firewall access when prompted

### macOS

1. Install dependencies:
```bash
# Install Homebrew if needed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
brew install node

# Install Xcode Command Line Tools
xcode-select --install
```

2. First run may require security approval in System Preferences

### Linux (Ubuntu/Debian)

1. Install dependencies:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install build dependencies
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential curl wget file \
    libssl-dev libgtk-3-dev \
    libayatana-appindicator3-dev librsvg2-dev
```

2. May need to configure firewall:
```bash
sudo ufw allow 37842/udp
sudo ufw allow 37843/tcp
sudo ufw allow 37844/tcp
```

## Troubleshooting

### Issue: "icons/icon.ico not found"

**Solution**: Add icon files to `desktop/src-tauri/icons/` directory (see step 2 above)

### Issue: "npm: command not found"

**Solution**: Install Node.js following the platform-specific instructions

### Issue: "cargo: command not found"

**Solution**: Install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: Frontend build errors

**Solution**: Make sure you're in the `desktop` directory and have run `npm install`

### Issue: "No peers discovered"

**Solutions**:
1. Check firewall settings
2. Ensure you're on the same network
3. Disable VPN temporarily
4. Verify multicast is supported on your network

### Issue: TypeScript errors in IDE

**Solution**: These are expected until you run `npm install`. The dependencies include type definitions.

## Verification

After setup, verify everything works:

```bash
# 1. Check Rust compilation
cd /path/to/local-ip-chat
cargo check --workspace

# 2. Check frontend compilation
cd desktop
npm run build

# 3. Run in dev mode
npm run tauri dev
```

## Next Steps

1. **Customize the UI**: Edit files in `desktop/src/components/`
2. **Add features**: Modify Rust crates in `crates/`
3. **Configure ports**: Edit port constants in the crates
4. **Branding**: Replace icons and update app name in `tauri.conf.json`

## Getting Help

- Check **QUICKSTART.md** for user guide
- See **BUILD.md** for detailed build instructions
- Review **API.md** for API documentation
- Open an issue on GitHub for bugs
- Read **PROJECT_SUMMARY.md** for architecture overview

---

**Ready to start!** Run the development server and test the application on your local network. 🚀
