use crate::file_utils::human_readable_size;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CompactionReport {
    pub input_files_count: usize,
    pub output_files_count: usize,
    pub total_input_size_bytes: u64,
    pub total_output_size_bytes: u64,
    pub target_size_bytes: u64,
    pub total_rows: usize,
    pub dry_run: bool,
    pub output_files: Vec<PathBuf>,
}

impl CompactionReport {
    pub fn file_compaction_ratio(&self) -> f64 {
        if self.output_files_count == 0 {
            return 0.0;
        }

        self.input_files_count as f64 / self.output_files_count as f64
    }
}

pub fn print_report(report: &CompactionReport) {
    let mode = if report.dry_run {
        "DRY RUN"
    } else {
        "COMPACTION COMPLETED"
    };

    println!();
    println!("============================================================");
    println!("Parquet Compactor Summary - {}", mode);
    println!("============================================================");
    println!("Input files              : {}", report.input_files_count);
    println!("Output files             : {}", report.output_files_count);
    println!(
        "Total input size         : {}",
        human_readable_size(report.total_input_size_bytes)
    );
    println!(
        "Total output size        : {}",
        human_readable_size(report.total_output_size_bytes)
    );
    println!(
        "Target output file size  : {}",
        human_readable_size(report.target_size_bytes)
    );
    println!("Total rows processed     : {}", report.total_rows);
    println!(
        "File compaction ratio    : {:.2}x",
        report.file_compaction_ratio()
    );
    println!("============================================================");

    if !report.output_files.is_empty() {
        println!("Output files:");

        for output_file in &report.output_files {
            println!("  - {}", output_file.display());
        }
    }

    println!();
}