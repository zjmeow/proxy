export RUSTUP_TOOLCHAIN=solana
rustup toolchain link solana /Users/jm/.local/share/solana/install/releases/stable-96c3a8519a3bac8c7e7dd49b6d6aefcfeba09d90/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools/rust
rustup toolchain uninstall 1.84.1-sbpf-solana-v1.51
cargo build-sbf
