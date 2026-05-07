import logging
from pathlib import Path

import pyarrow.parquet as pq

from pcompact.config import CompactionConfig
from pcompact.file_utils import (
    create_output_directory,
    find_parquet_files,
    get_file_size,
    get_total_size,
)
from pcompact.report import CompactionReport


logger = logging.getLogger(__name__)


def group_files_by_target_size(
    parquet_files: list[Path],
    target_size_bytes: int,
) -> list[list[Path]]:
    """
    Groups small Parquet files together until each group is close
    to the target size.

    Important:
    This uses input file size as an approximation.
    Actual output size may be slightly different because of compression.
    """

    groups: list[list[Path]] = []
    current_group: list[Path] = []
    current_group_size = 0

    for file_path in parquet_files:
        file_size = get_file_size(file_path)

        if current_group and current_group_size + file_size > target_size_bytes:
            groups.append(current_group)

            current_group = [file_path]
            current_group_size = file_size
        else:
            current_group.append(file_path)
            current_group_size += file_size

    if current_group:
        groups.append(current_group)

    return groups


def write_compacted_file(
    input_files: list[Path],
    output_file: Path,
    compression: str,
) -> tuple[int, int]:
    """
    Reads many small Parquet files and writes them into one larger Parquet file.

    Returns:
        output_size_bytes, total_rows_written
    """

    writer = None
    expected_schema = None
    total_rows_written = 0

    try:
        for input_file in input_files:
            logger.debug("Reading input file: %s", input_file)

            table = pq.read_table(input_file)

            if expected_schema is None:
                expected_schema = table.schema

            elif not table.schema.equals(expected_schema, check_metadata=False):
                raise ValueError(
                    f"Schema mismatch found in file: {input_file}. "
                    "All input Parquet files must have the same schema."
                )

            if writer is None:
                writer = pq.ParquetWriter(
                    where=output_file,
                    schema=expected_schema,
                    compression=compression,
                )

            writer.write_table(table)
            total_rows_written += table.num_rows

    finally:
        if writer is not None:
            writer.close()

    output_size_bytes = output_file.stat().st_size

    return output_size_bytes, total_rows_written


def compact_parquet_files(config: CompactionConfig) -> CompactionReport:
    """
    Main function that performs the full compaction process.
    """

    parquet_files = find_parquet_files(config.input_dir)

    if not parquet_files:
        raise FileNotFoundError(
            f"No Parquet files found in input directory: {config.input_dir}"
        )

    total_input_size = get_total_size(parquet_files)

    logger.info("Found %s Parquet files", len(parquet_files))
    logger.info("Total input size: %s bytes", total_input_size)

    file_groups = group_files_by_target_size(
        parquet_files=parquet_files,
        target_size_bytes=config.target_size_bytes,
    )

    logger.info("Planned output files: %s", len(file_groups))

    if config.dry_run:
        return CompactionReport(
            input_files_count=len(parquet_files),
            output_files_count=len(file_groups),
            total_input_size_bytes=total_input_size,
            total_output_size_bytes=0,
            target_size_bytes=config.target_size_bytes,
            total_rows=0,
            dry_run=True,
            output_files=[],
        )

    create_output_directory(config.output_dir)

    output_files: list[Path] = []
    total_output_size = 0
    total_rows = 0

    for group_number, input_file_group in enumerate(file_groups, start=1):
        output_file = config.output_dir / f"part-{group_number:05d}.parquet"

        logger.info(
            "Writing output file %s using %s input files",
            output_file,
            len(input_file_group),
        )

        output_size, rows_written = write_compacted_file(
            input_files=input_file_group,
            output_file=output_file,
            compression=config.compression,
        )

        output_files.append(output_file)
        total_output_size += output_size
        total_rows += rows_written

    return CompactionReport(
        input_files_count=len(parquet_files),
        output_files_count=len(output_files),
        total_input_size_bytes=total_input_size,
        total_output_size_bytes=total_output_size,
        target_size_bytes=config.target_size_bytes,
        total_rows=total_rows,
        dry_run=False,
        output_files=output_files,
    )