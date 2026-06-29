use anyhow::{Context, Result};
use colored::Colorize;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs;
use std::io::Write;
use std::path::Path;
use tar::Builder;

pub fn run(folder: std::path::PathBuf) -> Result<()> {
    let folder_path = Path::new(&folder);
    if !folder_path.exists() {
        anyhow::bail!("Folder does not exist: {}", folder.display());
    }

    let skill_files: Vec<_> = fs::read_dir(folder_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "skill")
                .unwrap_or(false)
        })
        .collect();

    if skill_files.is_empty() {
        anyhow::bail!("No .skill files found in {}", folder.display());
    }

    let mut builder = Builder::new(Vec::new());
    for entry in &skill_files {
        let path = entry.path();
        let name = path.file_name().unwrap();
        builder
            .append_path_with_name(&path, name)
            .map_err(|e| anyhow::anyhow!(e))
            .with_context(|| format!("Failed to add {} to archive", path.display()))?;
    }

    let data = builder.into_inner()?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;

    let output_name = format!(
        "{}.skill.tar.gz",
        folder_path.file_name().unwrap().to_str().unwrap()
    );
    fs::write(&output_name, compressed)?;
    println!(
        "{} {}",
        "Packed skill archive:".green().bold(),
        output_name.cyan()
    );
    println!("  {} .skill files included", skill_files.len());
    Ok(())
}
