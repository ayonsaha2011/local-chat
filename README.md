# LAN Chat

A modern, secure, cross-platform local area network messenger built with Rust. Fast, lightweight, and privacy-focused communication without the need for internet or central servers.

## ✨ Features

### Core Features
- **🔒 End-to-End Encryption**: Industry-standard AES-256-GCM + RSA 2048 encryption
- **⚡ Real-Time Messaging**: Instant P2P communication with message delivery receipts
- **📁 Direct File Transfer**: Send files directly between computers with progress tracking
- **🔍 Auto-Discovery**: Automatic peer discovery via UDP multicast
- **🌐 Cross-Platform**: Works on Windows, macOS, Linux, iOS, and Android
- **🎨 Modern UI**: Beautiful, responsive interface with dark mode support
- **🚀 Lightweight & Fast**: Built with Rust for maximum performance
- **🔐 No Server Required**: Truly peer-to-peer architecture

### Security Features
- **End-to-end encryption** for all messages
- **RSA key exchange** for secure session establishment
- **Digital signatures** for message authentication
- **No data collection** - everything stays on your local network
- **Open source** - fully auditable code

## 🏗️ Architecture

### Project Structure

```
local-ip-chat/
├── crates/
│   ├── core/          # Core data types and utilities
│   ├── crypto/        # Encryption and key management
│   ├── discovery/     # Peer discovery service (UDP multicast)
│   ├── protocol/      # Messaging protocol (TCP)
│   └── transfer/      # File transfer service
├── desktop/           # Tauri desktop application
│   ├── src/           # React + TypeScript UI
│   └── src-tauri/     # Rust backend integration
└── mobile/            # (Future) React Native mobile app
```

### Technology Stack

**Backend (Rust):**
- `tokio` - Async runtime
- `ring` / `rsa` / `aes-gcm` - Cryptography
- `serde` / `serde_json` - Serialization
- `socket2` - Low-level networking

**Desktop (Tauri):**
- `tauri` - Desktop framework
- `React` + `TypeScript` - UI framework
- `Tailwind CSS` - Styling
- `Vite` - Build tool

**Mobile (Planned):**
- `React Native` - Mobile framework
- `Rust FFI` - Native integration

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Node.js** 18+ ([Install Node.js](https://nodejs.org/))
- **Platform-specific tools:**
  - Windows: Visual Studio Build Tools or Windows SDK
  - macOS: Xcode Command Line Tools
  - Linux: `webkit2gtk`, `libgtk-3-dev`, `libappindicator3-dev`, `librsvg2-dev`

### Build & Run

#### Desktop Application

1. **Clone the repository:**
```bash
git clone <repository-url>
cd local-ip-chat
```

2. **Install dependencies:**
```bash
cd desktop
npm install
```

3. **Run in development mode:**
```bash
npm run tauri dev
```

4. **Build for production:**
```bash
npm run tauri build
```

The built application will be in `desktop/src-tauri/target/release/bundle/`.

#### Core Libraries Only

To build just the Rust libraries:

```bash
cargo build --release
```

To run tests:

```bash
cargo test
```

## 📖 How It Works

### 1. Peer Discovery
- Uses UDP multicast on port `37842`
- Broadcasts presence announcements
- Automatic peer detection within the same network
- Heartbeat mechanism to maintain peer list

### 2. Secure Messaging
- TCP connections on port `37843`
- RSA-2048 for key exchange
- AES-256-GCM for message encryption
- Message delivery and read receipts

### 3. File Transfer
- Direct TCP connections on port `37844`
- Chunked transfer with progress tracking
- SHA-256 hash verification
- Resume support for interrupted transfers

## 🔧 Configuration

### Network Ports

| Service | Port | Protocol | Purpose |
|---------|------|----------|---------|
| Discovery | 37842 | UDP Multicast | Peer discovery |
| Messaging | 37843 | TCP | Text messages |
| File Transfer | 37844 | TCP | File sharing |

### Multicast Addresses

- **IPv4**: `239.255.42.99`
- **IPv6**: `ff02::1`

## 📱 Platforms

### Desktop

- ✅ **Windows** 10/11
- ✅ **macOS** 10.15+
- ✅ **Linux** (Debian, Ubuntu, Fedora, Arch)

### Mobile (Planned)

- 🔄 **iOS** 13+
- 🔄 **Android** 8+

## 🛠️ Development

### Project Commands

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Check code
cargo clippy --workspace

# Format code
cargo fmt --workspace

# Desktop development
cd desktop && npm run dev

# Desktop build
cd desktop && npm run tauri build
```

### Adding Features

1. **Core library**: Add to `crates/core/`
2. **Networking**: Modify `crates/protocol/` or `crates/discovery/`
3. **UI**: Update `desktop/src/` components
4. **Backend integration**: Modify `desktop/src-tauri/src/`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔒 Security

This project takes security seriously. All communications are encrypted end-to-end using industry-standard cryptography:

- **AES-256-GCM** for symmetric encryption
- **RSA-2048** for key exchange
- **SHA-256** for file integrity verification
- **Ed25519** for digital signatures

If you discover a security vulnerability, please email [security contact] instead of using the issue tracker.

## 🙏 Acknowledgments

- Inspired by [LAN Messenger](https://lanmessenger.github.io/)
- Built with [Tauri](https://tauri.app/)
- Powered by [Rust](https://www.rust-lang.org/)

## 📞 Support

For questions and support:
- 📫 Open an issue on GitHub
- 💬 Join our discussions
- 📧 Email [contact]

---

**Made with ❤️ using Rust and modern web technologies**
