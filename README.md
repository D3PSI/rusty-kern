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
cargo kimage
```
