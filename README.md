# Rust lambda example

This repository is an example of AWS Lambda function empowered by Rust language.

## Build

Clone this repository.

```bash
git clone https://github.com/yoshihitoh/rust-lambda-example
```

### for Local

```bash
cargo build
```

### for AWS Lambda

```bash
docker container run --rm \
    -v $PWD:/code \
    -v $HOME/.cargo/registry:/root/.cargo/registry \
    -v $HOME/.cargo/git:/root/.cargo/git \
    softprops/lambda-rust
```
