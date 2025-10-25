# ⚠️ IMPORTANT: Icon Files Required

Before building the desktop application, you need to add icon files.

## Quick Solution

### Option 1: Use Tauri Icon Generator (Recommended)

If you have a source image (PNG, minimum 512x512 pixels):

```bash
# Install Tauri CLI
npm install -g @tauri-apps/cli

# Generate all icon formats
cd desktop/src-tauri
tauri icon /path/to/your-icon.png
```

This will automatically create all required icon files in the correct formats.

### Option 2: Add Icons Manually

Create these files in `desktop/src-tauri/icons/`:

1. **32x32.png** - 32×32 pixels PNG
2. **128x128.png** - 128×128 pixels PNG  
3. **128x128@2x.png** - 256×256 pixels PNG
4. **icon.icns** - macOS icon (can be generated from PNG)
5. **icon.ico** - Windows icon (can be generated from PNG)

### Option 3: Use Placeholder Icons (For Testing)

You can use simple colored squares as placeholders:

```bash
cd desktop/src-tauri
mkdir -p icons
cd icons

# Create simple placeholder images using imagemagick or online tools
# Or download free icons from sites like:
# - https://www.flaticon.com/
# - https://icons8.com/
# - https://heroicons.com/
```

## Icon Requirements

- **Format**: PNG (source), ICO (Windows), ICNS (macOS)
- **Size**: Minimum 512×512 for source
- **Background**: Transparent recommended
- **Style**: Simple, recognizable at small sizes
- **Colors**: High contrast for visibility

## Suggested Icon Design

For a LAN Chat application, consider:
- 💬 Chat bubble with network symbol
- 🌐 Globe with connection lines
- 📡 Signal waves icon
- 🔒 Lock with network symbol

## After Adding Icons

Once icons are in place:

```bash
cd desktop
npm install          # Install dependencies
npm run tauri dev    # Run development build
npm run tauri build  # Build production installers
```

## Need Help?

See **SETUP.md** for detailed instructions or visit Tauri's icon documentation:
https://tauri.app/v1/guides/features/icons

---

**Note**: The application will NOT build without these icon files. This is a Tauri requirement for creating platform-specific installers.
