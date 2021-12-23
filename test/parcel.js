const mod = require("../wasm-parcel");

let input = require("fs").readFileSync(
  __dirname + "/../react-dom.development.js"
);

console.log(input.byteLength);

let cfg = {
  filename: "abc",
  code: input,
  module_id: "abc",
  project_root: "abc",
  replace_env: true,
  inline_fs: true,
  insert_node_globals: true,
  is_browser: true,
  is_worker: true,
  env: {},
  is_type_script: true,
  is_jsx: true,
  jsx_pragma: "y",
  jsx_pragma_frag: "x",
  automatic_jsx_runtime: true,
  jsx_import_source: "asd",
  is_development: true,
  react_refresh: true,
  decorators: true,
  targets: {},
  source_maps: true,
  scope_hoist: true,
  source_type: "Module",
  supports_module_workers: true,
  is_library: true,
  is_esm_output: true,
  trace_bailouts: true,
};

(async () => {
  await mod.init();

  for (let i = 0; i < 10; i++) {
    const start = performance.now();

    let len = mod.transform(cfg).code.byteLength;

    const end = performance.now();
    console.log(`${len} ${(end - start).toFixed(2)}ms`);
  }
})();
