# Build Instructions

## Prerequisites

### All Platforms

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install Node.js** (v18 or later)
Download from https://nodejs.org/

### Platform-Specific Dependencies

#### Windows

Install Visual Studio Build Tools:
- Download from https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

Or install Windows SDK:
```powershell
winget install Microsoft.WindowsSDK
```

#### macOS

Install Xcode Command Line Tools:
```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Linux (Fedora)

```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    gtk3-devel \
    librsvg2-devel
```

#### Linux (Arch)

```bash
sudo pacman -S webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    gtk3 \
    librsvg
```

## Building

### Desktop Application

1. Navigate to the desktop directory:
```bash
cd desktop
```

2. Install npm dependencies:
```bash
npm install
```

3. For development:
```bash
npm run tauri dev
```

4. For production build:
```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

### Build Output Locations

- **Windows**: `desktop/src-tauri/target/release/bundle/msi/LAN Chat_0.1.0_x64_en-US.msi`
- **macOS**: `desktop/src-tauri/target/release/bundle/dmg/LAN Chat_0.1.0_x64.dmg`
- **Linux**: 
  - DEB: `desktop/src-tauri/target/release/bundle/deb/lan-chat_0.1.0_amd64.deb`
  - AppImage: `desktop/src-tauri/target/release/bundle/appimage/lan-chat_0.1.0_amd64.AppImage`

## Testing

### Rust Tests

Run all tests:
```bash
cargo test --workspace
```

Run specific crate tests:
```bash
cargo test -p lan-chat-core
cargo test -p lan-chat-crypto
```

### Code Quality

Check for errors:
```bash
cargo clippy --workspace
```

Format code:
```bash
cargo fmt --workspace
```

## Troubleshooting

### Windows

**Issue**: "error: linker 'link.exe' not found"
**Solution**: Install Visual Studio Build Tools

**Issue**: PowerShell execution policy
**Solution**: Run in PowerShell as Admin:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### macOS

**Issue**: "xcrun: error: invalid active developer path"
**Solution**: Install Xcode Command Line Tools:
```bash
xcode-select --install
```

### Linux

**Issue**: "error while loading shared libraries: libwebkit2gtk"
**Solution**: Install webkit2gtk development libraries (see platform-specific dependencies above)

**Issue**: Permission denied for multicast
**Solution**: May need to run with elevated privileges or configure firewall

## Development Mode

For faster iteration during development:

1. Run the Rust backend separately:
```bash
cd desktop/src-tauri
cargo run
```

2. Run the frontend separately:
```bash
cd desktop
npm run dev
```

## Cross-Compilation

To build for a different platform (advanced):

```bash
# Install target
rustup target add x86_64-pc-windows-msvc

# Build
cargo build --release --target x86_64-pc-windows-msvc
```

Note: Cross-compilation requires platform-specific toolchains and may not work for all targets.
