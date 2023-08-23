extern crate swc_common;

use std::path::Path;
use std::process;
use swc::Compiler;
use swc::config::{IsModule, SourceMapsConfig};
use swc_common::errors::{ColorConfig, Handler};
use swc_common::{FileName, FilePathMapping, GLOBALS, Mark, SourceMap};
use swc_common::sync::Lrc;
use swc_ecma_parser::Syntax;
use swc_ecma_ast::EsVersion;
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

pub fn generate_wasm_binary() {
    process::Command::new("cargo")
        .args(["build", "--target", "wasm32-wasi", "--release"])
        .current_dir("~/Projects/Encedeus/cli/template-src")
        .spawn()
        .unwrap();
}

pub fn run_swc() {
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let path = Path::new("template-src/src/main.ts");
    let source = cm.new_source_file(FileName::Real(path.to_path_buf()), std::fs::read_to_string(path).unwrap());

    GLOBALS.set(&Default::default(), || {
        let compiler = Compiler::new(cm.clone());
        let res = compiler.parse_js(
            source,
            &handler,
            EsVersion::Es2020,
            if true {
                Syntax::Typescript(Default::default())
            } else {
                Syntax::Es(Default::default())
            },
            IsModule::Bool(true),
            Some(compiler.comments()),
        ).expect("Compilation failed");

        let module = res.module().unwrap();

        let top_level_mark = Mark::new();
        let module = module.fold_with(&mut strip(top_level_mark));

        let out = compiler.print(
            &module,
            None,
            None,
            false,
            EsVersion::Es2020,
            SourceMapsConfig::Bool(true),
            &Default::default(),
            None,
            true,
            Some(compiler.comments()),
            false,
            false,
            "",
        ).unwrap();
        std::fs::write(Path::new("template-src/src/main.js"), out.code).unwrap();
    });
}