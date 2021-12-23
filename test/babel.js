const babel = require("@babel/core");

let input = require("fs").readFileSync(
  __dirname + "/../react-dom.development.js",
  "utf8"
);

for (let i = 0; i < 10; i++) {
  const start = process.hrtime.bigint();
  let len = babel.transformSync(input, {compact: false}).code.length;
  const end = process.hrtime.bigint();
  console.log(`${len} -> ${(end - start) / 1000000n}ms`);
}
