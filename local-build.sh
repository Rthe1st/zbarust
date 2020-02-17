cargo build --verbose
cargo test --verbose
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
