#!/run/current-system/sw/bin/bash
# Pre-commit validation script
# Runs all checks that are also performed in GitHub CI

set -e

echo "🔧 Running pre-commit checks..."
echo ""

echo "1️⃣ Code formatting check..."
if ! cargo fmt --check; then
    echo "❌ Code formatting issues found. Running cargo fmt to fix..."
    cargo fmt
    echo "✅ Code formatted. Please review changes and commit again."
    exit 1
else
    echo "✅ Code formatting is correct"
fi
echo ""

echo "2️⃣ Linting (no warnings allowed)..."
cargo clippy -- -D warnings
echo "✅ Clippy checks passed"
echo ""

echo "3️⃣ Running all tests..."
cargo test
echo "✅ All tests passed"
echo ""

echo "4️⃣ Release build verification..."
cargo build --release
echo "✅ Release build successful"
echo ""

echo "5️⃣ Nix build verification..."
if command -v nix &> /dev/null; then
    nix build
    echo "✅ Nix build successful"
else
    echo "⚠️  Nix not available, skipping nix build"
fi
echo ""

echo "🎉 All pre-commit checks passed!"
echo ""
echo "💡 You can now safely commit your changes:"
echo "   git add ."
echo "   git commit -m \"your message\""
echo "   git push" 