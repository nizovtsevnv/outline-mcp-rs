# 🔐 Windows Code Signing Guide

This guide covers how to enable code signing for Windows binaries in GitHub Actions.

## 🎯 Why Code Signing?

- ✅ **User Trust** - Windows SmartScreen won't flag your app
- ✅ **Security** - Verifies binary integrity and authenticity  
- ✅ **Professional** - Shows serious software development
- ✅ **Distribution** - Required for some enterprise environments

## 📋 Prerequisites

### 1. Code Signing Certificate

You need a valid Authenticode certificate:

#### **Commercial Certificates** (Recommended)
- **DigiCert** - $400-600/year, high trust
- **Sectigo** - $200-400/year, good reputation
- **Comodo** - $100-300/year, basic signing
- **GlobalSign** - $200-500/year, enterprise options

#### **Self-Signed** (Testing only)
```powershell
# Create self-signed certificate (Windows)
New-SelfSignedCertificate -DnsName "Your Company" -Type CodeSigning -CertStoreLocation cert:\CurrentUser\My

# Export to P12
$cert = Get-ChildItem -Path cert:\CurrentUser\My -CodeSigningCert
Export-PfxCertificate -Cert $cert -FilePath "self-signed.p12" -Password (ConvertTo-SecureString -String "password123" -Force -AsPlainText)
```

### 2. Certificate Format
Ensure your certificate is in **P12/PFX** format with private key included.

## 🔧 Setup Instructions

### Step 1: Prepare Certificate

```bash
# Convert certificate to base64 (Linux/macOS)
base64 -i your-certificate.p12 > certificate.base64

# On Windows PowerShell
[Convert]::ToBase64String([IO.File]::ReadAllBytes("your-certificate.p12")) | Out-File certificate.base64
```

### Step 2: Add GitHub Secrets

1. Go to your repository → **Settings** → **Secrets and variables** → **Actions**

2. Add these repository secrets:
   - **Name**: `WINDOWS_CERTIFICATE`
   - **Value**: Content of `certificate.base64` file
   
   - **Name**: `WINDOWS_CERTIFICATE_PASSWORD` 
   - **Value**: Your certificate password

### Step 3: Enable Signing in Workflow

Edit `.github/workflows/release.yml` and uncomment the signing code:

```yaml
- name: Sign Windows binary (if certificate available)
  if: matrix.target == 'x86_64-pc-windows-gnu' && runner.os == 'Linux'
  shell: bash
  run: |
    # Install osslsigncode for cross-platform signing
    sudo apt-get update && sudo apt-get install -y osslsigncode
    
    # Decode certificate
    echo "${{ secrets.WINDOWS_CERTIFICATE }}" | base64 -d > cert.p12
    
    # Sign the binary
    osslsigncode sign \
      -pkcs12 cert.p12 \
      -pass "${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}" \
      -n "Outline MCP Server" \
      -i "https://github.com/nizovtsevnv/outline-mcp-rs" \
      -t http://timestamp.digicert.com \
      -in result/bin/outline-mcp.exe \
      -out result/bin/outline-mcp-signed.exe
    
    # Replace unsigned with signed binary
    mv result/bin/outline-mcp-signed.exe result/bin/outline-mcp.exe
    
    # Verify signature
    osslsigncode verify result/bin/outline-mcp.exe
    
    # Clean up certificate
    rm cert.p12
```

### Step 4: Test Signing

1. **Push a tag** to trigger release workflow:
   ```bash
   git tag v1.0.0-test
   git push origin v1.0.0-test
   ```

2. **Check GitHub Actions logs** for signing success

3. **Download and verify** the signed binary on Windows:
   ```powershell
   # Check signature (Windows)
   Get-AuthenticodeSignature outline-mcp.exe
   
   # Should show "Valid" status
   ```

## 🔍 Verification

### On Windows
```powershell
# PowerShell verification
Get-AuthenticodeSignature .\outline-mcp.exe | Format-List

# Expected output:
# Status: Valid
# SignerCertificate: [Your certificate info]
```

### File Properties
Right-click the `.exe` → **Properties** → **Digital Signatures** tab should show your certificate.

## 🐛 Troubleshooting

### Common Issues

#### Certificate Format Errors
```bash
# Error: "Invalid certificate format"
# Solution: Ensure P12/PFX format with private key
openssl pkcs12 -info -in your-certificate.p12  # Verify format
```

#### Base64 Encoding Issues
```bash
# Error: "Invalid base64"
# Solution: Check for line breaks in base64 string
cat certificate.base64 | tr -d '\n' > certificate-clean.base64
```

#### Timestamp Server Failures
```yaml
# Add fallback timestamp servers
-t http://timestamp.digicert.com \
# Fallbacks:
# -t http://timestamp.sectigo.com
# -t http://timestamp.globalsign.com
```

#### osslsigncode Installation
```bash
# Ubuntu/Debian
sudo apt-get install osslsigncode

# Alpine Linux (in containers)
apk add osslsigncode

# macOS
brew install osslsigncode
```

### Debugging Workflow

1. **Check certificate validity**:
   ```bash
   openssl pkcs12 -info -in cert.p12 -nokeys
   ```

2. **Test signing locally**:
   ```bash
   osslsigncode sign -pkcs12 cert.p12 -pass "password" -in unsigned.exe -out signed.exe
   ```

3. **Verify secrets are accessible**:
   ```yaml
   - name: Debug secrets
     run: |
       echo "Certificate length: ${#WINDOWS_CERTIFICATE}"
       echo "Password set: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD != '' }}"
     env:
       WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}
   ```

## 🎨 Advanced Configuration

### Custom Signing Parameters
```bash
osslsigncode sign \
  -pkcs12 cert.p12 \
  -pass "$CERT_PASSWORD" \
  -n "Your App Name" \                    # Display name
  -i "https://your-website.com" \         # More info URL
  -d "App description" \                  # Description
  -t http://timestamp.digicert.com \      # Timestamp server
  -h sha256 \                             # Hash algorithm
  -in unsigned.exe \
  -out signed.exe
```

### Multiple Certificate Support
```yaml
# Sign with different certificates based on environment
- name: Choose certificate
  run: |
    if [ "${{ github.ref }}" == "refs/heads/main" ]; then
      echo "CERT_SECRET=PRODUCTION_WINDOWS_CERTIFICATE" >> $GITHUB_ENV
    else
      echo "CERT_SECRET=DEVELOPMENT_WINDOWS_CERTIFICATE" >> $GITHUB_ENV
    fi
```

## 💰 Cost Considerations

### Certificate Costs (Annual)
- **Code Signing**: $100-600/year depending on CA
- **EV Code Signing**: $300-800/year (instant trust)
- **Organization Validation**: Additional verification time

### Free Alternatives
- **Self-signed**: Free, but triggers security warnings
- **Test certificates**: For development/CI only

## 📚 Resources

- [Microsoft Code Signing Guide](https://docs.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)
- [osslsigncode Documentation](https://github.com/mtrojnar/osslsigncode)
- [Certificate Authority Comparison](https://www.ssl.com/compare/)
- [Windows SmartScreen Information](https://docs.microsoft.com/en-us/windows/security/threat-protection/microsoft-defender-smartscreen/)

## 🔒 Security Best Practices

1. **Protect Private Keys**: Never commit certificates to git
2. **Use Hardware Tokens**: For high-value certificates (EV)
3. **Regular Rotation**: Update certificates before expiration
4. **Monitor Usage**: Track where signed binaries are distributed
5. **Timestamping**: Always use timestamp servers for long-term validity 