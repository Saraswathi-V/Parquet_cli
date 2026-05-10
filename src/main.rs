use anyhow::Result;
use clap::Parser;
use parquet_compactor_rs::cli::Cli;
use parquet_compactor_rs::compactor::compact_parquet_files;
use parquet_compactor_rs::config::Config;
use parquet_compactor_rs::report::print_report;
use tracing_subscriber::FmtSubscriber;

fn setup_logging(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    setup_logging(cli.verbose);

    let config = Config::from_cli(cli)?;

    let report = compact_parquet_files(&config)?;

    print_report(&report);

    Ok(())
}