# rust-playground

This is my playground to learn rust.


## Building a Distribution

### Backend

For e.g. Raspberry Pi (ARM64):
```sh
cross build --release --package pizzeria-backend --target=aarch64-unknown-linux-gnu
```

Uploading to target host:
```sh
scp target/aarch64-unknown-linux-gnu/release/pizzeria-backend pi:.
```
