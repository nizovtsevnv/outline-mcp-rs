# 🚀 Release Process

## Automated Release Creation

This repository uses GitHub Actions to automatically build and release cross-platform binaries.

### 📦 What gets built automatically:

1. **Linux x86_64** - Standard glibc build
2. **Linux x86_64 (musl)** - Static build for maximum portability  
3. **Windows x86_64** - Cross-compiled .exe file
4. **macOS x86_64** - Intel Mac build (native runner)
5. **macOS ARM64** - Apple Silicon build (native runner)

### 🎯 How to create a release:

#### Option 1: Git Tag (Recommended)
```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0
```

#### Option 2: GitHub Release
1. Go to GitHub → Releases → "Create a new release"
2. Choose or create a new tag (e.g., `v1.0.0`)
3. Add release title and description
4. Publish release

### 🔄 What happens automatically:

1. **GitHub Actions triggers** when you push a tag or create a release
2. **Multi-platform builds** using appropriate runners:
   - **Linux builds** (Ubuntu runner): `nix build`, `nix build .#musl`, `nix build .#windows`
   - **macOS Intel builds** (macOS-13 runner): `nix build .#macos-x86_64`
   - **macOS ARM builds** (macOS-14 runner): `nix build .#macos-arm64`
3. **Artifacts are created**:
   - `outline-mcp-linux-x86_64.tar.gz` - Standard Linux binary + docs
   - `outline-mcp-linux-x86_64-musl.tar.gz` - Static Linux binary + docs  
   - `outline-mcp-windows-x86_64.zip` - Windows executable + docs
   - `outline-mcp-macos-x86_64.tar.gz` - macOS Intel binary + docs
   - `outline-mcp-macos-arm64.tar.gz` - macOS ARM binary + docs
   - Each archive contains: binary, SHA256 checksum, README.txt with usage
4. **GitHub Release is updated** with:
   - All archive files
   - Auto-generated release notes

### 📋 File naming convention:

```
outline-mcp-{platform}-{arch}.{ext}
  ├── outline-mcp[.exe]          # Main binary
  ├── outline-mcp[.exe].sha256   # Checksum  
  └── README.txt                 # Usage instructions
```

Examples:
- `outline-mcp-linux-x86_64.tar.gz` → contains `outline-mcp`
- `outline-mcp-linux-x86_64-musl.tar.gz` → contains `outline-mcp`
- `outline-mcp-windows-x86_64.zip` → contains `outline-mcp.exe`
- `outline-mcp-macos-x86_64.tar.gz` → contains `outline-mcp`
- `outline-mcp-macos-arm64.tar.gz` → contains `outline-mcp`

### 🛠️ Manual testing before release:

```bash
# Test Linux/Windows builds locally (works on any Nix system)
nix build                # Linux glibc
nix build .#musl         # Linux static
nix build .#windows      # Windows cross-compile

# Test macOS builds (requires macOS with Nix)
nix build .#macos-x86_64 # macOS Intel (works on Intel Mac or CI)
nix build .#macos-arm64  # macOS ARM (works on Apple Silicon Mac or CI)

# Run tests
nix develop -c cargo test
nix develop -c cargo clippy
nix develop -c cargo audit
```

### 🍎 macOS Build Details:

#### Local macOS Development:
```bash
# Install Nix on macOS
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Enable flakes
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# Build for current macOS architecture
nix build                # Auto-detects Intel/ARM

# Cross-compile for other macOS architecture  
nix build .#macos-x86_64 # Intel target
nix build .#macos-arm64  # ARM target
```

#### macOS Dependencies:
- **Security Framework** - For TLS/HTTPS support
- **CoreFoundation** - System foundation services  
- **SystemConfiguration** - Network configuration APIs
- **Apple SDK** - Provided by Nix's Darwin packages

#### GitHub Actions Setup:
- **macOS-13 runners** - Intel-based for x86_64 builds
- **macOS-14 runners** - ARM-based for ARM64 builds
- **Native compilation** - Better compatibility than cross-compilation

### 🔧 Requirements for the repository:

1. **GitHub Secrets** (optional):
   - `CACHIX_AUTH_TOKEN` - For Nix binary cache (speeds up builds)
   - `WINDOWS_CERTIFICATE` - Base64-encoded P12/PFX certificate for Windows signing
   - `WINDOWS_CERTIFICATE_PASSWORD` - Password for Windows certificate

2. **Repository Settings**:
   - Actions must be enabled
   - Workflow permissions: "Read and write permissions"
   - GitHub-hosted runners enabled (including macOS)

3. **Nix Configuration**:
   - `flake.nix` with Darwin cross-compilation targets
   - Apple SDK frameworks properly configured
   - Platform-specific tokio features for threading

4. **Code Signing** (optional):
   - **Windows**: Requires valid Authenticode certificate
   - **macOS**: Code signing handled on macOS runners automatically

### 🎨 Customizing releases:

- **Modify `.github/workflows/release.yml`** to:
  - Add more platforms
  - Change binary names
  - Customize release notes
  - Add additional files to archives

- **Version format**: Use semantic versioning `v{major}.{minor}.{patch}`

### 🐛 Troubleshooting:

#### Linux/Windows builds:
- **Build fails**: Check Nix flake configuration and dependencies
- **Cross-compilation issues**: Verify target configurations in `flake.nix`

#### macOS builds:
- **Local build fails**: Ensure Nix with flakes is properly installed
- **CI macOS build fails**: Check runner availability and SDK compatibility
- **Cross-compilation limits**: Some macOS builds may require native runners

#### Windows signing:
- **Certificate issues**: Verify P12/PFX certificate is valid and base64-encoded
- **osslsigncode errors**: Check certificate password and format
- **Signing disabled**: Remove certificate secrets to build unsigned binaries

#### General:
- **No release created**: Ensure tag format is `v*.*.*` (e.g., `v1.0.0`)
- **Missing binaries**: Check GitHub Actions logs for platform-specific errors
- **Permission denied**: Verify repository workflow permissions

### 🔐 Code Signing Setup

#### Windows Code Signing
To enable Windows binary signing, add these repository secrets:

1. **Get a code signing certificate**:
   - Purchase from CA (DigiCert, Sectigo, etc.)
   - Or use self-signed for testing

2. **Convert to base64**:
   ```bash
   # Convert P12/PFX to base64
   base64 -i your-certificate.p12 -o certificate.base64
   ```

3. **Add GitHub Secrets**:
   - `WINDOWS_CERTIFICATE`: Content of certificate.base64
   - `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

4. **Uncomment signing code** in `.github/workflows/release.yml`

#### macOS Code Signing
macOS runners automatically handle code signing for valid Apple Developer accounts.
No additional configuration needed for basic signing. 