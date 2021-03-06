# rusty-kern
## building
clone the repository:

```
git clone https://github.com/D3PSI/rusty-kern.git
```

### dependencies
a nightly rust toolchain is required. installation using [rustup](https://rustup.rs/):

```
rustup install nightly
rustup override set nightly
```

## building a bootable image
after installing all dependencies you should be able to build a bootable image as follows:

```
cargo image
```

## running
to run the image `qemu-system-x86_64` must be installed and on the PATH.
by default, the runner will start with UEFI boot:

```
cargo run
```

this requires QEMU's OVMF firmware to be separately installed. BIOS boot does not require that dependency:

```
cargo run -- --bios-boot
```
