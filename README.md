# Template project to cross compile Rust for a raspberry pi

Note this was tested on a Pi Model B rev 2 

## Requirements

`cargo install cross`

Need `Cross.toml`, 0.1.16 is the last image version with openssl installed (openssl-1.0.1f)

```
[target.arm-unknown-linux-gnueabi]
image = "rustembedded/cross:arm-unknown-linux-gnueabi-0.1.16"
```

## Build for PI
`cross build --target=arm-unknown-linux-gnueabi`

