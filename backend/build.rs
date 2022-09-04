use std::path::Path;
use std::process::Command;

const FRONTEND_DIR: &str = "../frontend";

/// build script which is being run ahead of ./src/main.rs
/// compiles frontend in order to serve to client
fn main() {
    println!("cargo:rerun-if-changed={}/src", FRONTEND_DIR);
    println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);

    build_frontend(FRONTEND_DIR);
}

fn build_frontend<P: AsRef<Path>>(source: P) {
    Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(source.as_ref())
        .status()
        .expect("Failed to build Frontend");
}
