use crate::cli::Cli;
use crate::file_utils::parse_size;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub target_size_bytes: u64,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self> {
        let target_size_bytes = parse_size(&cli.target_size)?;

        Ok(Self {
            input_dir: cli.input_dir,
            output_dir: cli.output_dir,
            target_size_bytes,
            dry_run: cli.dry_run,
            verbose: cli.verbose,
        })
    }
}