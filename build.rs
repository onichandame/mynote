use std::{fs, process::Command};

fn main() {
    println!("compiling spa");
    let spa_root = fs::canonicalize("./frontend").unwrap();
    Command::new("yarn")
        .current_dir(&spa_root)
        .status()
        .unwrap();
    Command::new("yarn")
        .arg("build")
        .current_dir(&spa_root)
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=frontend/src");
}
