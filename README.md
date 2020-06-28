# Template project to cross compile Rust for a raspberry pi

Note this was tested on a Pi Model B rev 2 

## Requirements

`rustup target add arm-unknown-linux-gnueabi`

`sudo apt-get install gcc-multilib-arm-linux-gnueabi`

We need to add our build target to ~/.cargo/config by adding the following lines, so that rust knows which linker to use.

```
 [target.arm-unknown-linux-gnueabi]
 linker = "arm-linux-gnueabi-gcc"
```

## Build for PI
`cargo build --target=arm-unknown-linux-gnueabi`

<!-- PKG_CONFIG_ALLOW_CROSS=1 cargo build --target=arm-unknown-linux-gnueabi -->
