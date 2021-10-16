cargo build --release && sudo dd if=target/x86_64-rusty-kern/release/rusty-kern of=$1 && sync
