use crate::detect::{detect_project, print_inspect};

pub fn run() -> anyhow::Result<()> {
    let info = detect_project()?;
    print_inspect(&info);
    Ok(())
}