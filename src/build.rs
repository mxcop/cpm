use std::{path::Path, process::Command};

/// Run the build command
pub(crate) fn run(path: Option<String>) {
    let path = path.unwrap_or(".".into());
    let path = Path::new(&path);

    println!("~ Building `{}`", path.to_string_lossy());

    let source = Path::join(path, Path::new("./src/"));
    let include = Path::join(path, Path::new("./inc/"));

    Command::new("gcc")
        .args([
            Path::join(&source, "main.c"), 
            Path::join(&source, "test.c")
        ])
        .arg("-o")
        .arg("program")
        .arg("-I")
        .arg(include.to_string_lossy().to_string())
        .spawn()
        .expect("failed to build")
        .wait_with_output().unwrap();
}