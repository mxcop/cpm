use std::{process::{Command, Output}, io, path::Path};

use super::project::Project;

/// Build a cpm project using `gcc`
pub(crate) fn build(ctx: Project) -> io::Result<Output> {
    let mut cmd = Command::new("gcc");

    for file in ctx.source_files {
        cmd.arg(file);
    }

    cmd.arg("-o");
    cmd.arg(Path::join(Path::new(&ctx.project_dir), "./target/program"));

    cmd.arg("-I");
    cmd.arg(ctx.include_dir);

    let process = cmd.spawn().expect("failed to spawn gcc");
    process.wait_with_output()
}