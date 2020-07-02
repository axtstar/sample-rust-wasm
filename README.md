# 🦀🕸️ `sample-rust-wasm`

## 🚴 build

### 🐑 Use `cargo generate` to Clone this Template

install wasm-pack

> https://rustwasm.github.io/wasm-pack/installer/

### 🛠️ Build with `wasm-pack build`

build pkg
```
wasm-pack build
```

pkg link 
```
cd ./pkg
npm link
```

npm link sample-rust-wasm
```
cd www
npm link sample-rust-wasm
```

start server
```
cd www
npm run start
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
