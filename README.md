Run `yarn && yarn build:napi && yarn build:wasm` to build the swc binaries.

Each line is a run, to give the JIT optimzation some time. But ideally, it would be fast on first run already.

These times are for a noop roundtrip of parsing into the AST and then reemitting code again, without any transformations (the JS is 918kb).

## Babel

```
$ yarn test:babel
939464 -> 1007ms
939464 -> 555ms
939464 -> 561ms
939464 -> 418ms
939464 -> 397ms
939464 -> 390ms
939464 -> 473ms
939464 -> 502ms
939464 -> 413ms
939464 -> 407ms
```

## swc NAPI

```
$ yarn test:napi
1067023 -> 123ms
1067023 -> 135ms
1067023 -> 124ms
1067023 -> 124ms
1067023 -> 126ms
```

## swc Wasm

```
$ yarn test:napi
1067023 -> 314ms
1067023 -> 246ms
1067023 -> 149ms
1067023 -> 120ms
1067023 -> 120ms
1067023 -> 119ms
1067023 -> 121ms
1067023 -> 116ms
1067023 -> 118ms
1067023 -> 121ms
```

## swc Wasm (force optimizing JIT)

```
$ yarn test:wasm2
1067023 -> 118ms
1067023 -> 113ms
1067023 -> 119ms
1067023 -> 121ms
1067023 -> 120ms
1067023 -> 120ms
1067023 -> 116ms
1067023 -> 125ms
1067023 -> 121ms
1067023 -> 123ms
```
