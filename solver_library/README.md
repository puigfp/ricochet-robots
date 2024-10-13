## Requirements

### wasm-pack

```sh
cargo install wasm-pack 
```

## Building library to `pkg/`

### For a bundler

```sh
wasm-pack build --target bundler
```

### For Node.js

```sh
wasm-pack build --target nodejs
```

## Running unit tests

### Natively

```sh
cargo test
```

### In WASM

```sh
wasm-pack test --node
```
