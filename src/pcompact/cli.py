import logging
from pathlib import Path

import typer

from pcompact.compactor import compact_parquet_files
from pcompact.config import CompactionConfig
from pcompact.file_utils import parse_size
from pcompact.report import print_report


def setup_logging(verbose: bool) -> None:
    """
    Configures logging.

    If verbose=True, show detailed DEBUG logs.
    Otherwise, show normal INFO logs.
    """

    log_level = logging.DEBUG if verbose else logging.INFO

    logging.basicConfig(
        level=log_level,
        format="%(levelname)s - %(message)s",
    )


def run_compaction(
    input_dir: Path = typer.Option(
        ...,
        "--input-dir",
        "-i",
        exists=True,
        file_okay=False,
        dir_okay=True,
        readable=True,
        help="Input directory containing small Parquet files.",
    ),
    output_dir: Path = typer.Option(
        ...,
        "--output-dir",
        "-o",
        file_okay=False,
        dir_okay=True,
        help="Output directory for compacted Parquet files.",
    ),
    target_size: str = typer.Option(
        "128MB",
        "--target-size",
        "-t",
        help="Target output file size. Example: 128MB, 256MB, 1GB.",
    ),
    dry_run: bool = typer.Option(
        False,
        "--dry-run",
        help="Preview compaction plan without writing output files.",
    ),
    verbose: bool = typer.Option(
        False,
        "--verbose",
        "-v",
        help="Show detailed logs.",
    ),
) -> None:
    """
    Compacts many small Parquet files into fewer larger Parquet files.
    """

    setup_logging(verbose=verbose)

    try:
        target_size_bytes = parse_size(target_size)

        config = CompactionConfig(
            input_dir=input_dir,
            output_dir=output_dir,
            target_size_bytes=target_size_bytes,
            dry_run=dry_run,
            verbose=verbose,
        )

        report = compact_parquet_files(config)

        print_report(report)

    except Exception as error:
        typer.echo(f"Error: {error}", err=True)
        raise typer.Exit(code=1)


def cli_entry() -> None:
    """
    Entry point used by pyproject.toml.

    This allows users to run:
        pcompact -i ./small_files -o ./compacted --target-size 128MB
    """

    typer.run(run_compaction)


if __name__ == "__main__":
    cli_entry()