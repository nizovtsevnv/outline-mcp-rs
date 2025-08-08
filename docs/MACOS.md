# 🍎 macOS Development Guide

This guide covers building and developing `outline-mcp-rs` on macOS systems.

## 🚀 Quick Start

### Prerequisites

1. **Nix Package Manager** (recommended):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
   ```

2. **Enable Nix Flakes**:
   ```bash
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

3. **Restart terminal** or source the Nix environment

### Build Commands

```bash
# Clone the repository
git clone https://github.com/nizovtsevnv/outline-mcp-rs
cd outline-mcp-rs

# Build for current macOS architecture
nix build                # Auto-detects Intel or Apple Silicon

# Build for specific architectures
nix build .#macos-x86_64 # Intel Mac target
nix build .#macos-arm64  # Apple Silicon target

# Use development environment
nix develop .#macos      # macOS-specific development shell
```

## 🏗️ Architecture Support

### Intel Macs (x86_64)
- **Target**: `x86_64-apple-darwin`
- **Build Command**: `nix build .#macos-x86_64`
- **Compatible**: Intel-based Macs (2020 and earlier)

### Apple Silicon Macs (ARM64)
- **Target**: `aarch64-apple-darwin`
- **Build Command**: `nix build .#macos-arm64`
- **Compatible**: Apple Silicon Macs (M1, M2, M3, etc.)

### Universal Compatibility
The auto-detection build (`nix build`) will create binaries for your current system architecture.

## 🛠️ Development Environment

### Enter macOS Development Shell
```bash
nix develop .#macos
```

This provides:
- ✅ **Rust toolchain** with macOS targets
- ✅ **Apple SDK frameworks** (Security, CoreFoundation, SystemConfiguration)
- ✅ **Development tools** (cargo, clippy, rustfmt)
- ✅ **Testing utilities** (cargo-audit, cargo-deny)

### Manual Cargo Commands
```bash
# Inside the development shell
cargo build --release                              # Current architecture
cargo build --target x86_64-apple-darwin --release # Intel target
cargo build --target aarch64-apple-darwin --release # ARM target

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check
```

## 🔧 Framework Dependencies

The macOS build requires several Apple frameworks:

### Security Framework
- **Purpose**: TLS/HTTPS connections to Outline API
- **Usage**: SSL certificate validation, secure networking
- **Required**: Yes

### CoreFoundation
- **Purpose**: System foundation services
- **Usage**: String handling, collections, system integration
- **Required**: Yes

### SystemConfiguration
- **Purpose**: Network configuration detection
- **Usage**: Network reachability, proxy settings
- **Required**: Yes

These frameworks are automatically included when using the Nix environment.

## 🧪 Testing on macOS

### Unit Tests
```bash
nix develop .#macos -c cargo test
```

### Integration Tests
```bash
# Set your Outline API credentials
export OUTLINE_API_KEY="your-api-key"
export OUTLINE_API_URL="https://your-outline.example.com"

# Run integration tests
nix develop .#macos -c cargo test --test integration
```

### End-to-End Tests
```bash
# Build and test the binary
nix build
./result/bin/outline-mcp --help

# Test HTTP mode
./result/bin/outline-mcp --http &
curl http://localhost:3000/mcp -d '{"jsonrpc":"2.0","method":"initialize","id":1}'
```

## 📦 Distribution

### Creating Universal Binaries
For maximum compatibility, build both architectures:

```bash
# Build Intel version
nix build .#macos-x86_64
cp result/bin/outline-mcp outline-mcp-intel

# Build ARM version  
nix build .#macos-arm64
cp result/bin/outline-mcp outline-mcp-arm64

# Create universal binary with lipo (if needed)
lipo -create -output outline-mcp-universal outline-mcp-intel outline-mcp-arm64
```

### Code Signing (Optional)
For distribution outside the App Store:

```bash
# Sign the binary
codesign --force --deep --sign "Developer ID Application: Your Name" result/bin/outline-mcp

# Verify signature
codesign --verify --verbose result/bin/outline-mcp
```

## 🔄 Cross-Compilation Limitations

### From macOS to Other Platforms
- ✅ **Linux**: `nix build .#musl` (static build)
- ✅ **Windows**: `nix build .#windows` (MinGW cross-compile)
- ✅ **Other macOS**: Intel ↔ ARM cross-compilation

### From Other Platforms to macOS
- ❌ **Linux → macOS**: Not supported (requires macOS SDK licensing)
- ❌ **Windows → macOS**: Not supported

Use GitHub Actions with macOS runners for CI/CD.

## 🐛 Troubleshooting

### Common Issues

#### Nix Installation Problems
```bash
# Check Nix installation
nix --version

# Reinstall if needed
/nix/nix-installer uninstall
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

#### Framework Linking Errors
```bash
# Check if frameworks are available
ls /System/Library/Frameworks/Security.framework
ls /System/Library/Frameworks/CoreFoundation.framework

# Try rebuilding with clean cache
nix build --rebuild
```

#### Architecture Mismatch
```bash
# Check current architecture
uname -m
# x86_64 = Intel Mac
# arm64 = Apple Silicon Mac

# Build for correct target
nix build .#macos-x86_64  # Intel
nix build .#macos-arm64   # Apple Silicon
```

#### Permission Issues
```bash
# Fix Nix store permissions
sudo chown -R $(whoami) /nix

# Or reinstall Nix with correct permissions
```

### Getting Help

1. **Check GitHub Actions logs** for successful macOS builds
2. **Verify Nix flake configuration** with `nix flake check`
3. **Test minimal builds** with `nix build --no-substitute`
4. **Join discussions** in GitHub Issues for platform-specific problems

## 📚 Additional Resources

- [Nix on macOS Official Guide](https://nixos.org/manual/nix/stable/installation/installing-binary.html#macos)
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Apple Developer Documentation](https://developer.apple.com/documentation/)
- [GitHub Actions macOS Runners](https://docs.github.com/en/actions/using-github-hosted-runners/about-github-hosted-runners#supported-runners-and-hardware-resources) 