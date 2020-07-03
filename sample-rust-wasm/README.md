# 🦀🕸️ `sample-rust-wasm`

## 🚴 prerequired

install wasm-pack

> https://rustwasm.github.io/wasm-pack/installer/

### 🛠️ Build

build pkg

```
wasm-pack build
```

npm link to local global

pkg link

```
cd pkg
npm link
```

npm link sample-rust-wasm

```
cd www
npm link sample-rust-wasm
```

### Start

start server

```
cd www
npm run start
```

http://localhost:8080
