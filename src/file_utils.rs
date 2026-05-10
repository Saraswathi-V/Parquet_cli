use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn parse_size(size_text: &str) -> Result<u64> {
    let cleaned = size_text.trim().to_uppercase();

    let mut number_part = String::new();
    let mut unit_part = String::new();

    for character in cleaned.chars() {
        if character.is_ascii_digit() || character == '.' {
            number_part.push(character);
        } else if !character.is_whitespace() {
            unit_part.push(character);
        }
    }

    if number_part.is_empty() || unit_part.is_empty() {
        return Err(anyhow!(
            "Invalid size format. Use examples like 128MB, 256MB, or 1GB."
        ));
    }

    let number: f64 = number_part.parse()?;

    let multiplier = match unit_part.as_str() {
        "B" => 1_f64,
        "KB" => 1024_f64,
        "MB" => 1024_f64 * 1024_f64,
        "GB" => 1024_f64 * 1024_f64 * 1024_f64,
        _ => {
            return Err(anyhow!(
                "Unsupported size unit: {}. Use B, KB, MB, or GB.",
                unit_part
            ))
        }
    };

    Ok((number * multiplier) as u64)
}

pub fn human_readable_size(size_bytes: u64) -> String {
    let size = size_bytes as f64;

    if size_bytes < 1024 {
        format!("{} B", size_bytes)
    } else if size_bytes < 1024 * 1024 {
        format!("{:.2} KB", size / 1024_f64)
    } else if size_bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size / (1024_f64 * 1024_f64))
    } else {
        format!("{:.2} GB", size / (1024_f64 * 1024_f64 * 1024_f64))
    }
}

pub fn find_parquet_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    if !input_dir.exists() {
        return Err(anyhow!(
            "Input directory does not exist: {}",
            input_dir.display()
        ));
    }

    if !input_dir.is_dir() {
        return Err(anyhow!(
            "Input path is not a directory: {}",
            input_dir.display()
        ));
    }

    let mut parquet_files = Vec::new();

    for entry in WalkDir::new(input_dir) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let path = entry.path();

            let is_parquet = path
                .extension()
                .and_then(|extension| extension.to_str())
                .map(|extension| extension.eq_ignore_ascii_case("parquet"))
                .unwrap_or(false);

            if is_parquet {
                parquet_files.push(path.to_path_buf());
            }
        }
    }

    parquet_files.sort();

    Ok(parquet_files)
}

pub fn file_size(file_path: &Path) -> Result<u64> {
    Ok(fs::metadata(file_path)?.len())
}

pub fn total_size(file_paths: &[PathBuf]) -> Result<u64> {
    let mut total = 0;

    for file_path in file_paths {
        total += file_size(file_path)?;
    }

    Ok(total)
}

pub fn create_output_directory(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    Ok(())
}

pub fn clean_old_parquet_files(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();

        let is_parquet = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.eq_ignore_ascii_case("parquet"))
            .unwrap_or(false);

        if is_parquet {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}