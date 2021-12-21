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
use napi::{CallContext, JsNumber, JsObject, JsString};
#[cfg(not(target_arch = "wasm32"))]
use napi_derive::{js_function, module_exports};

#[cfg(not(target_arch = "wasm32"))]
#[js_function(1)]
fn run(ctx: CallContext) -> napi::Result<JsNumber> {
    let arg = ctx.get::<JsString>(0)?.into_utf8()?;
    let res = compile(arg.as_str()?);
    ctx.env.create_uint32(res)
}

#[cfg(not(target_arch = "wasm32"))]
#[module_exports]
fn init(mut exports: JsObject) -> napi::Result<()> {
    exports.create_named_method("run", run)?;
    Ok(())
}

// ----------------------------------------------------

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(src: &str) -> u32 {
    compile(src)
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
            let transform = &mut chain!(expr_simplifier(Default::default()), dead_branch_remover());
            let module = module.fold_with(transform);
            let transform = &mut preset_env(
                global_mark,
                Some(&comments),
                swc_ecmascript::preset_env::Config::default(),
            );
            let module = module.fold_with(transform);
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
