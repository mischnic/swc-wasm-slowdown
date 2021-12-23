use swc_common::{
    chain, comments::SingleThreadedComments, input::StringInput, sync::Lrc, FileName, Globals,
    Mark, SourceMap,
};
use swc_ecmascript::{
    ast::Module,
    codegen::{text_writer::JsWriter, Config, Emitter},
    parser::{lexer::Lexer, EsConfig, PResult, Parser, Syntax},
    preset_env::preset_env,
    transforms::{
        compat::reserved_words::reserved_words,
        fixer, helpers, hygiene,
        optimization::simplify::{dead_branch_remover, expr_simplifier},
        resolver_with_mark,
    },
    visit::{FoldWith, Visit, VisitWith},
};
// ----------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
use napi::bindgen_prelude::Uint8Array;
#[cfg(not(target_arch = "wasm32"))]
use napi_derive::napi;

#[cfg(not(target_arch = "wasm32"))]
#[napi]
fn run(code: Uint8Array) -> u32 {
    let code = unsafe { std::str::from_utf8_unchecked(&code) };
    compile(code)
}

// ----------------------------------------------------

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(code: &[u8]) -> u32 {
    let code = unsafe { std::str::from_utf8_unchecked(&code) };
    compile(code)
}

// ----------------------------------------------------

fn compile(src: &str) -> u32 {
    // let src = include_str!("../react.development.js");
    let cm = Lrc::<SourceMap>::default();
    let (module, comments) = parse(src, "test.js", &cm).unwrap();

    swc_common::GLOBALS.set(&Globals::new(), || {
        helpers::HELPERS.set(&helpers::Helpers::default(), || {
            let global_mark = Mark::fresh(Mark::root());
            let module = module.fold_with(&mut resolver_with_mark(global_mark));

            // let transform = &mut react::react(
            //     cm.clone(),
            //     Some(&comments),
            //     react::Options {
            //         development: true,
            //         refresh: Some(Default::default()),
            //         ..Default::default()
            //     },
            //     global_mark,
            // );
            // let transform = &mut expr_simplifier();
            // let transform = &mut dead_branch_remover();
            // let transform = &mut chain!(expr_simplifier(Default::default()), dead_branch_remover());
            // let module = module.fold_with(transform);
            // let transform = &mut preset_env(
            //     global_mark,
            //     Some(&comments),
            //     swc_ecmascript::preset_env::Config::default(),
            // );
            // let module = module.fold_with(transform);
            // module.visit_with(&mut Test {});

            let module = module.fold_with(&mut chain!(
                reserved_words(),
                hygiene(),
                fixer(Some(&comments))
            ));

            let code = emit(&module, &comments, cm);
            code.len().try_into().unwrap()
        })
    })
}

// struct Test {}
// impl Visit for Test {
//     fn visit_expr(&mut self, node: &swc_ecmascript::ast::Expr) {
//         if let swc_ecmascript::ast::Expr::Ident(ident) = &node {
//             println!("visit_expr {:#?}", ident);
//         }
//         node.visit_children_with(self);
//     }
//     fn visit_member_expr(&mut self, node: &swc_ecmascript::ast::MemberExpr) {
//         println!("fold_member_expr obj {:#?}", node.obj);
//         node.obj.visit_with(self);
//         if node.computed {
//             println!("fold_member_expr prop {:#?}", node.prop);
//             node.prop.visit_with(self);
//         }
//     }
// }

fn parse(
    code: &str,
    filename: &str,
    cm: &Lrc<SourceMap>,
) -> PResult<(Module, SingleThreadedComments)> {
    let source_file = cm.new_source_file(FileName::Real(filename.into()), code.into());
    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        // Syntax::Typescript(TsConfig {
        //     tsx: true,
        //     ..Default::default()
        // }),
        Default::default(),
        StringInput::from(&*source_file),
        Some(&comments),
    );
    let mut parser = Parser::new_from(lexer);
    match parser.parse_module() {
        Err(err) => Err(err),
        Ok(module) => Ok((module, comments)),
    }
}

fn emit(module: &Module, comments: &SingleThreadedComments, cm: Lrc<SourceMap>) -> String {
    let mut buf = vec![];
    {
        let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
        let config = Config { minify: false };
        let mut emitter = Emitter {
            cfg: config,
            comments: Some(&comments),
            cm,
            wr: writer,
        };
        emitter.emit_module(module).unwrap();
    }

    String::from_utf8(buf).unwrap()
}
