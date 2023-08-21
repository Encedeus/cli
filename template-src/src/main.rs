use include_dir::{include_dir, Dir};

fn main() {
    use encedeus_js_runtime as q;

    static JS_SRC: Dir = include_dir!("/mnt/h/Programming/Web/Workspace/Projects/Encedeus/test/js");

    let mut rt = q::Runtime::new(); 
    rt.run_with_context(|ctx| {
        let file_path = String::from(JS_SRC.get_file("main.js").unwrap().path().to_str().unwrap());
        
        let code = JS_SRC.get_file("main.js").unwrap().contents_utf8().unwrap().to_string();

        ctx.put_args(vec![file_path.clone()]);
        ctx.eval_module_str(code, &file_path);
        ctx.js_loop().unwrap();
    });
}
