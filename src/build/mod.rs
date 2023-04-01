use std::{fs, path::Path};

use crate::build::project::Project;

mod gcc;
mod project;

const SOURCE_PATH: &'static str = "./src/";
const INCLUDE_PATH: &'static str = "./inc/";

/// Run the build command
pub(crate) fn run(path: Option<String>) {
    let path_str = path.unwrap_or(".".into());
    let path = Path::new(&path_str);

    // Get the source and include folders:
    let source = Path::join(path, SOURCE_PATH);
    let include = Path::join(path, INCLUDE_PATH);

    // Create the project context:
    let mut ctx = Project::new(
        path.to_string_lossy().to_string(),
        include.to_string_lossy().to_string(),
    );

    walk_src(&mut ctx.source_files, &source).expect("failed to walk source dir");

    fs::create_dir_all(Path::join(Path::new(&ctx.project_dir), "./target/")).expect("failed to create target dir");

    println!("~ Building `{}` using gcc", path_str);

    gcc::build(ctx).expect("failed to build with gcc");

    println!("~ Finished building `{}` using gcc", path_str);
    println!("Output file: `{}/target/program(.exe)`", path_str);
}

/// Recursively walk through all source files in a directory.
fn walk_src(
    files: &mut Vec<String>,
    dir: &Path,
) -> std::io::Result<()> {
    if dir.is_dir() {
        let dir = fs::read_dir(dir)?;

        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_src(files, &path)?;
            } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("c") {
                files.push(entry.path().to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}
