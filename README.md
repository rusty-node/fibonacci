# @rusty-maths/fibonacci

Fibonacci for Node.js, implemented in Rust with [NAPI-RS](https://napi.rs) and shipped as prebuilt native binaries for common Linux platforms (glibc and musl), so it works in Debian/Ubuntu, Alpine, and ARM Docker images without compiling on install.

## Install

```bash
npm install @rusty-maths/fibonacci
```

## Usage

```js
const { fibonacci } = require('@rusty-maths/fibonacci')

console.log(fibonacci(10)) // 55
console.log(fibonacci(20)) // 6765
```

`n` must be `<= 92` (larger values overflow `i64`).

## Supported Linux targets

| Target | Typical Docker base |
| --- | --- |
| `linux-x64-gnu` | `node:*-slim`, Debian, Ubuntu |
| `linux-x64-musl` | `node:*-alpine` |
| `linux-arm64-gnu` | ARM64 glibc images |
| `linux-arm64-musl` | ARM64 Alpine |
| `linux-arm-gnueabihf` | 32-bit ARM glibc |
| `linux-arm-musleabihf` | 32-bit ARM musl |
| `linux-riscv64-gnu` / `musl` | RISC-V |
| `linux-s390x-gnu` | IBM Z |
| `linux-ppc64-gnu` | POWER |

The loader in `index.js` picks gnu vs musl automatically.

## Development

Requires Node **>= 20**.

```bash
npm install
npm run build
npm test
```

## License

MIT
