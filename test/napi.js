const mod = require("../index.darwin-x64.node");

const usePresetEnv = process.argv[2] === "env";

let input = require("fs").readFileSync(
  __dirname + "/../react-dom.development.js"
);

for (let i = 0; i < 5; i++) {
  const start = process.hrtime.bigint();
  let len = mod.run(input, usePresetEnv);
  const end = process.hrtime.bigint();
  console.log(`${len} -> ${(end - start) / 1000000n}ms`);
}
