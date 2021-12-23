const mod = require("../index.darwin-x64.node");

let input = require("fs").readFileSync(
  __dirname + "/../react-dom.development.js"
);

for (let i = 0; i < 5; i++) {
  const start = process.hrtime.bigint();
  let len = mod.run(input);
  const end = process.hrtime.bigint();
  console.log(`${len} -> ${(end - start) / 1000000n}ms`);
}
