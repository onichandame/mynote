use std::{path::Path, process::Command};

fn main() {
    compile_styles();
}

fn compile_styles() {
    println!("cargo:rerun-if-changed=styles");
    let mut cmd = Command::new("sassc");
    cmd.arg(Path::new("styles").join("app.scss"))
        .arg(Path::new("src").join("app").join("app.css"));
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
