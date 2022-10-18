# wasmcrime

learn about wasm, cranelift, jit, and more by committing wasm crimes

## north star goal

```sh
$ wasmcrime hello.wasm
Hello, world!
```

## naive pipeline

* parse wasm
* translate to cranelift IR
* optimizations
* emit AArch64 ELF
* print/stdout WASI
