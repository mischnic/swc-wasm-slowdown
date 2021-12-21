const mod = require("../wasm");

let input = require("fs").readFileSync(
  __dirname + "/../react.development.js",
  "utf8"
);

for (let i = 0; i < 10; i++) {
  const start = process.hrtime.bigint();
  let len = mod.run(input);
  const end = process.hrtime.bigint();
  console.log(`${len} -> ${(end - start) / 1000000n}ms`);
}
