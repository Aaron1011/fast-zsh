extern crate bindgen;

use std::path::PathBuf;
use std::env;
use std::fs;
use std::process::Command;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    compile_zsh();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .hide_type("max_align_t")
        .unstable_rust(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}

fn compile_zsh() {
    let old_dir = env::current_dir().unwrap();
    let zsh_dir = old_dir.join("zsh");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let zsh_install_dir = out_dir.join("zsh_install");

    fs::create_dir_all(&zsh_install_dir).unwrap();

    let zsh_install_str = zsh_install_dir.into_os_string().into_string().unwrap();

    env::set_current_dir(zsh_dir.clone()).unwrap();

    run_command(&mut Command::new(zsh_dir.join("Util/preconfig")),
        "Failed to run zsh/Util/preconfig");

    run_command(
        Command::new(zsh_dir.join("./configure"))
            .args(&[format!("--prefix={}", zsh_install_str), format!("--exec-prefix={}", zsh_install_str)]),
        "Failed to run zsh/configure");



    run_command(
        &mut Command::new("make"),
        "Failed to run make");

    run_command(
        Command::new("make")
            .args(&["install"]),
        "Failed to run make install");


    env::set_current_dir(old_dir).unwrap();

}

fn run_command(command: &mut Command, err: &'static str) {
    assert!(command.status().unwrap().success(), err);
}
