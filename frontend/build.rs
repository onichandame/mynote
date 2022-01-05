use std::{fs, process::Command};

fn main() {
    println!("compiling spa");
    let spa_root = fs::canonicalize("./spa").unwrap();
    Command::new("yarn")
        .current_dir(&spa_root)
        .status()
        .unwrap();
    Command::new("yarn")
        .arg("build")
        .current_dir(&spa_root)
        .status()
        .unwrap();
}
