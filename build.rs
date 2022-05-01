use std::{fs, process::Command};

fn main() {
    println!("compiling spa");
    let fe_root = fs::canonicalize(".").unwrap();
    Command::new("yarn").current_dir(&fe_root).status().unwrap();
    Command::new("yarn")
        .arg("workspace")
        .arg("frontend")
        .arg("build")
        .current_dir(&fe_root)
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=frontend/src");
}
