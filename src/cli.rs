use clap::Parser;
use std::path::PathBuf;

/// Parquet Compactor CLI
///
/// Reads many small Parquet files and compacts them into fewer optimized files.
#[derive(Parser, Debug)]
#[command(name = "pcompact")]
#[command(version = "0.1.0")]
#[command(about = "Compact many small Parquet files into fewer optimized files")]
pub struct Cli {
    /// Input directory containing small Parquet files
    #[arg(short = 'i', long = "input")]
    pub input_dir: PathBuf,

    /// Output directory for compacted Parquet files
    #[arg(short = 'o', long = "output")]
    pub output_dir: PathBuf,

    /// Target output file size, for example: 128MB, 256MB, 1GB
    #[arg(long = "target-size", default_value = "128MB")]
    pub target_size: String,

    /// Preview compaction plan without writing output files
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,

    /// Show detailed logs
    #[arg(long = "verbose", short = 'v', default_value_t = false)]
    pub verbose: bool,
}