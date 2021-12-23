const mod = require("../wasm");

const usePresetEnv = process.argv[2] === "env";

let input = require("fs").readFileSync(
  __dirname + "/../react-dom.development.js"
);

for (let i = 0; i < 10; i++) {
  const start = process.hrtime.bigint();
  let len = mod.run(input, usePresetEnv);
  const end = process.hrtime.bigint();
  console.log(`${len} -> ${(end - start) / 1000000n}ms`);
}
