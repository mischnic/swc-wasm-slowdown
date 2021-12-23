Run `yarn && yarn build:napi && yarn build:wasm` to build the swc binaries.

Each line is a run, to give the JIT optimzation some time. But ideally, it would be fast on first run already.

These times are for a noop roundtrip of parsing into the AST and then reemitting code again, without any transformations (the JS is 918kb).

## Babel

```
$ yarn test:babel
939464 -> 1005ms
939464 -> 532ms
939464 -> 601ms
939464 -> 450ms
939464 -> 445ms
939464 -> 417ms
939464 -> 450ms
939464 -> 513ms
939464 -> 407ms
939464 -> 388ms
```

## swc NAPI

```
$ yarn test:napi
997527 -> 292ms
997527 -> 279ms
997527 -> 267ms
997527 -> 263ms
997527 -> 285ms
```

## swc Wasm

```
$ yarn test:napi
997527 -> 686ms
997527 -> 705ms
997527 -> 531ms
997527 -> 380ms
997527 -> 323ms
997527 -> 290ms
997527 -> 287ms
997527 -> 307ms
997527 -> 294ms
997527 -> 330ms
```

## swc Wasm (force optimizing JIT)

```
$ yarn test:wasm2
997527 -> 303ms
997527 -> 294ms
997527 -> 287ms
997527 -> 286ms
997527 -> 291ms
997527 -> 289ms
997527 -> 296ms
997527 -> 303ms
997527 -> 306ms
997527 -> 305ms
```
