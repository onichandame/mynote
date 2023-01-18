use std::{fs, path::Path, process::Command};

static GEN_DIR: &str = "gen";

fn main() {
    prepare();
    compile_styles();
}

fn prepare() {
    fs::create_dir_all(GEN_DIR).unwrap();
}

fn compile_styles() {
    println!("cargo:rerun-if-changed=styles");
    let mut cmd = Command::new("sassc");
    cmd.arg(Path::new("styles").join("app.scss"))
        .arg(Path::new(GEN_DIR).join("app.css"));
    run_cmd(cmd);
}

fn run_cmd(mut cmd: Command) {
    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                panic!("command exited with error: {:?}", cmd)
            }
        }
        Err(e) => panic!("command failed to run: {:?}. {:?}", cmd, e),
    }
}
