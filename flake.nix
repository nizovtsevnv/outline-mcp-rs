{
  description = "Development environment for Outline MCP Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Package metadata with automatic sync from Cargo.toml
        cargoTomlPath = ./Cargo.toml;
        packageMeta = 
          if builtins.pathExists cargoTomlPath
          then (builtins.fromTOML (builtins.readFile cargoTomlPath)).package // {
            # Binary name is different from package name  
            binaryName = "outline-mcp";
          }
          else {
            # Fallback values if Cargo.toml not available
            name = "outline-mcp-rs";
            version = "1.0.5";
            description = "MCP server for Outline knowledge base";
            license = "MIT";
            repository = "https://github.com/nizovtsevnv/outline-mcp-rs";
            binaryName = "outline-mcp";
        };

        # Rust toolchain with cross-compilation targets
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
          targets = [ 
            "x86_64-pc-windows-gnu" 
            "x86_64-unknown-linux-musl" 
            "aarch64-unknown-linux-musl"
          ];
        };

        # Base build inputs shared across all environments
        baseBuildInputs = [
          rustToolchain
          pkgs.pkg-config
          pkgs.cacert
        ];

        # Common shell hook function
        mkShellHook = targetName: extraInfo: ''
          echo "🦀 ${packageMeta.description}"
          echo "📦 Rust $(rustc --version)"
          echo "🎯 Target: ${targetName}"
          ${extraInfo}
          echo ""
        '';

        # Reusable shell builder function
        mkDevShell = { 
          name, 
          extraBuildInputs ? [], 
          extraEnvVars ? {}, 
          extraShellHook ? "" 
        }: pkgs.mkShell (extraEnvVars // {
          buildInputs = baseBuildInputs ++ extraBuildInputs;
          shellHook = mkShellHook name extraShellHook;
        });

      in
      {
        devShells = {
          # Default development environment
          default = mkDevShell {
            name = "Native Development";
            extraBuildInputs = [
              pkgs.openssl
              pkgs.cargo-watch
              pkgs.cargo-audit
              pkgs.cargo-deny
            ];
            extraShellHook = ''
              echo "🚀 Commands: cargo run, cargo test, cargo clippy"
              echo "🔧 Cross-compile: nix develop .#musl / .#windows"
            '';
          };

          # musl static build environment
          musl = mkDevShell {
            name = "musl Static Build";
            extraBuildInputs = [
              pkgs.pkgsStatic.openssl
            ];
            extraEnvVars = {
              OPENSSL_STATIC = "1";
              OPENSSL_LIB_DIR = "${pkgs.pkgsStatic.openssl.out}/lib";
              OPENSSL_INCLUDE_DIR = "${pkgs.pkgsStatic.openssl.dev}/include";
              PKG_CONFIG_ALL_STATIC = "1";
            };
            extraShellHook = ''
              echo "🚀 Build: cargo build --target x86_64-unknown-linux-musl --release"
              echo "🔗 Linker: rust-lld (static)"
            '';
          };

          # Windows cross-compilation environment
          windows = mkDevShell {
            name = "Windows Cross-compilation";
            extraBuildInputs = [
              pkgs.pkgsCross.mingwW64.stdenv.cc
              pkgs.pkgsCross.mingwW64.windows.pthreads
            ];
            extraEnvVars = {
              CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}gcc";
              CC_x86_64_pc_windows_gnu = "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}gcc";
              PKG_CONFIG_ALLOW_CROSS = "1";
              # Add pthread library path for Windows cross-compilation
              CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L ${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
            };
            extraShellHook = ''
              echo "🚀 Build: cargo build --target x86_64-pc-windows-gnu --release"
              echo "🔗 Linker: ${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}gcc"
              echo "📚 Pthread lib: ${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
            '';
          };

          # macOS development environment (only on Darwin systems)
          macos = mkDevShell {
            name = "macOS Development";
            extraBuildInputs = with pkgs; lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.CoreFoundation
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];
            extraShellHook = ''
              echo "🚀 Native: cargo build --release"
              echo "🚀 Intel: cargo build --target x86_64-apple-darwin --release"
              echo "🚀 ARM64: cargo build --target aarch64-apple-darwin --release"
              echo "🍎 Frameworks: Security, CoreFoundation, SystemConfiguration"
            '';
          };
        };

        # Package definition using Cargo.toml metadata
        packages = {
  default = pkgs.rustPlatform.buildRustPackage {
    pname = packageMeta.name;
    version = packageMeta.version;
    src = ./.;
    
            # Use cargoHash instead of lockFile for better compatibility
        cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.openssl ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
      # Use newer Darwin frameworks (12.3 is the newest available)
      pkgs.darwin.apple_sdk_12_3.frameworks.Security
      pkgs.darwin.apple_sdk_12_3.frameworks.CoreFoundation  
      pkgs.darwin.apple_sdk_12_3.frameworks.SystemConfiguration
    ];
    meta = with pkgs.lib; {
      description = packageMeta.description;
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };

          # musl static build
            musl = pkgs.pkgsStatic.rustPlatform.buildRustPackage {
    pname = "${packageMeta.name}-musl";
    version = packageMeta.version;
    src = ./.;
    
    cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.pkgsStatic.openssl ];
    
    CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
    OPENSSL_STATIC = "1";
    OPENSSL_LIB_DIR = "${pkgs.pkgsStatic.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.pkgsStatic.openssl.dev}/include";
    PKG_CONFIG_ALL_STATIC = "1";
    
    meta = with pkgs.lib; {
      description = "${packageMeta.description} (musl static)";
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };

          # glibc optimized build (smaller dynamic binary with static OpenSSL)
            glibc-optimized = pkgs.rustPlatform.buildRustPackage {
    pname = "${packageMeta.name}-glibc-optimized";
    version = packageMeta.version;
    src = ./.;
    
    cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ 
      pkgs.openssl.dev
      pkgs.openssl.out
    ];
    
    CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
    OPENSSL_STATIC = "1";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
    PKG_CONFIG_ALL_STATIC = "1";
    
    meta = with pkgs.lib; {
      description = "${packageMeta.description} (glibc optimized - static OpenSSL, dynamic glibc)";
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };

          # Windows cross-compilation
            windows = pkgs.pkgsCross.mingwW64.rustPlatform.buildRustPackage {
    pname = "${packageMeta.name}-windows";
    version = packageMeta.version;
    src = ./.;
    
    cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ 
      pkgs.pkgsCross.mingwW64.windows.pthreads
    ];
    
    CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
    PKG_CONFIG_ALLOW_CROSS = "1";
    CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L ${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
    
    meta = with pkgs.lib; {
      description = "${packageMeta.description} (Windows)";
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };

          # macOS x86_64 cross-compilation  
            macos-x86_64 = pkgs.pkgsCross.x86_64-darwin.rustPlatform.buildRustPackage {
    pname = "${packageMeta.name}-macos-x86_64";
    version = packageMeta.version;
    src = ./.;
    
    cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = with pkgs.pkgsCross.x86_64-darwin; [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreFoundation
      darwin.apple_sdk.frameworks.SystemConfiguration
    ];
    
    CARGO_BUILD_TARGET = "x86_64-apple-darwin";
    PKG_CONFIG_ALLOW_CROSS = "1";
    
    meta = with pkgs.lib; {
      description = "${packageMeta.description} (macOS x86_64)";
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };

          # macOS ARM64 (Apple Silicon) cross-compilation
            macos-arm64 = pkgs.pkgsCross.aarch64-darwin.rustPlatform.buildRustPackage {
    pname = "${packageMeta.name}-macos-arm64";
    version = packageMeta.version;
    src = ./.;
    
    cargoHash = "sha256-CfU++oEyFCZO3ukAjpbsBOzrN1adC20xOEsKKzBAP5M=";
    
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = with pkgs.pkgsCross.aarch64-darwin; [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreFoundation  
      darwin.apple_sdk.frameworks.SystemConfiguration
    ];
    
    CARGO_BUILD_TARGET = "aarch64-apple-darwin";
    PKG_CONFIG_ALLOW_CROSS = "1";
    
    meta = with pkgs.lib; {
      description = "${packageMeta.description} (macOS ARM64)";
      license = licenses.mit;
      homepage = packageMeta.repository;
    };
  };
        };
      }
    );
} 