Run `yarn && yarn build:napi && yarn build:wasm` to build the swc binaries.

Each line is a run, to give the JIT optimzation some time. But ideally, it would be fast on first run already.

## Parsing & Emitting

These times are for a noop roundtrip of parsing into the AST and then reemitting code again, without any transformations (the JS is 918kb).

#### Babel

```
$ yarn test:babel
939464 -> 845ms
939464 -> 487ms
939464 -> 517ms
939464 -> 400ms
939464 -> 403ms
939464 -> 388ms
939464 -> 427ms
939464 -> 483ms
939464 -> 395ms
939464 -> 376ms
```

#### swc NAPI

```
$ yarn test:napi
1067023 -> 111ms
1067023 -> 126ms
1067023 -> 135ms
1067023 -> 129ms
1067023 -> 122ms
```

#### swc Wasm

```
$ yarn test:wasm
1067023 -> 362ms
1067023 -> 309ms
1067023 -> 307ms
1067023 -> 291ms
1067023 -> 234ms
1067023 -> 222ms
1067023 -> 180ms
1067023 -> 127ms
1067023 -> 132ms
1067023 -> 148ms
```

#### swc Wasm (force optimizing JIT)

```
$ yarn test:wasm2
1067023 -> 104ms
1067023 -> 107ms
1067023 -> 104ms
1067023 -> 104ms
1067023 -> 109ms
1067023 -> 118ms
1067023 -> 115ms
1067023 -> 110ms
1067023 -> 112ms
1067023 -> 115ms
```

## Parsing & Emitting + Preset-env

#### Babel

```
$ yarn test:babel env
943548 -> 1889ms
943548 -> 1283ms
943548 -> 1110ms
943548 -> 1103ms
943548 -> 1087ms
943548 -> 1077ms
943548 -> 901ms
943548 -> 1040ms
943548 -> 956ms
943548 -> 1136ms
```

#### swc NAPI

```
$ yarn test:napi env
1093312 -> 294ms
1093312 -> 281ms
1093312 -> 287ms
1093312 -> 282ms
1093312 -> 279ms
```

#### swc Wasm

```
$ yarn test:wasm env
1093312 -> 723ms
1093312 -> 660ms
1093312 -> 492ms
1093312 -> 322ms
1093312 -> 288ms
1093312 -> 287ms
1093312 -> 280ms
1093312 -> 291ms
1093312 -> 292ms
1093312 -> 297ms
```

#### swc Wasm (force optimizing JIT)

```
$ yarn test:wasm2 env
1093312 -> 297ms
1093312 -> 252ms
1093312 -> 321ms
1093312 -> 298ms
1093312 -> 288ms
1093312 -> 293ms
1093312 -> 288ms
1093312 -> 294ms
1093312 -> 292ms
1093312 -> 289ms
```
