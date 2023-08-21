// #![feature(rustc_private)]
//
// extern crate rustc_session;
// extern crate rustc_interface;
// extern crate rustc_driver;
// extern crate rustc_span;
// extern crate rustc_errors;
// extern crate rustc_hash;
// extern crate rustc_error_codes;
//
// use std::{path, process, str};
//
// use rustc_span::source_map;
// use rustc_session::config::{self, CheckCfg};
// use rustc_hash::{FxHashMap, FxHashSet};
// use rustc_errors::registry;
//
// pub fn generate_wasm_binary() {
//     let out = process::Command::new("rustc")
//         .arg("--print=sysroot")
//         .current_dir(".")
//         .output()
//         .unwrap();
//     let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
//     let config = rustc_interface::Config {
//         opts: config::Options {
//             maybe_sysroot: Some(path::PathBuf::from(sysroot)),
//             ..config::Options::default()
//         },
//         crate_cfg: FxHashSet::default(),
//         crate_check_cfg: CheckCfg::default(),
//         input: config::Input::Str {
//             name: source_map::FileName::Custom("main.rs".into()),
//             input: r#"
//
// fn main() {
//     println!("Hello, world!");
// }
// "#
//             .into(),
//         },
//         output_file: None,
//         // output_dir: None,
//         output_dir: Some(path::PathBuf::from(".")),
//         // output_file: Some(config::OutFileName::Real(path::PathBuf::from("/mnt/h/Programming/Web/Workspace/Projects/Encedeus/sdk-cli/out"))),
//         file_loader: None,
//         locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
//         lint_caps: FxHashMap::default(),
//         parse_sess_created: None,
//         register_lints: None,
//         override_queries: None,
//         registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
//         make_codegen_backend: None,
//         ice_file: None,
//     };
//     
//     rustc_interface::run_compiler(config, |compiler| {
//         compiler.enter(|queries| {
//             queries.global_ctxt().unwrap().enter(|t| {
//                 
//             });
//         });
//     });
// }

use std::process;

pub fn generate_wasm_binary() {
    process::Command::new("cargo")
        .args(["build", "--target", "wasm32-wasi", "--release"])
        .current_dir("/mnt/h/Programming/Web/Workspace/Projects/Encedeus/cli/template-src")
        .spawn()
        .unwrap();
}
