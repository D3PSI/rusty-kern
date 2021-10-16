cargo kimage --release && sudo dd if=target/x86_64-rusty-kern/debug/boot-uefi-rusty-kern.img of=$1 && sync
