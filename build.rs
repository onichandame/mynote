use std::{fs, process::Command};

fn main() {
    let fe_root = fs::canonicalize(".").unwrap();
    let yarn_out = Command::new("yarn").current_dir(&fe_root).output().unwrap();
    if !yarn_out.status.success() {
        panic!("{}", std::str::from_utf8(&yarn_out.stdout).unwrap())
    }
    let build_out = Command::new("yarn")
        .arg("workspace")
        .arg("frontend")
        .arg("build")
        .current_dir(&fe_root)
        .output()
        .unwrap();
    if !build_out.status.success() {
        panic!("{}", std::str::from_utf8(&build_out.stdout).unwrap())
    }
    println!("cargo:rerun-if-changed=frontend/src");
}
