from pathlib import Path

import pyarrow as pa
import pyarrow.parquet as pq

from pcompact.compactor import compact_parquet_files
from pcompact.config import CompactionConfig


def create_test_parquet_file(file_path: Path, start_id: int, number_of_rows: int) -> None:
    """
    Creates one small Parquet file for testing.
    """

    table = pa.table(
        {
            "id": list(range(start_id, start_id + number_of_rows)),
            "name": [f"name_{i}" for i in range(number_of_rows)],
            "amount": [float(i) for i in range(number_of_rows)],
        }
    )

    pq.write_table(table, file_path)


def test_compact_parquet_files(tmp_path: Path):
    input_dir = tmp_path / "input"
    output_dir = tmp_path / "output"

    input_dir.mkdir()

    create_test_parquet_file(input_dir / "file_1.parquet", 0, 10)
    create_test_parquet_file(input_dir / "file_2.parquet", 10, 10)
    create_test_parquet_file(input_dir / "file_3.parquet", 20, 10)

    config = CompactionConfig(
        input_dir=input_dir,
        output_dir=output_dir,
        target_size_bytes=1024 * 1024,
        dry_run=False,
    )

    report = compact_parquet_files(config)

    assert report.input_files_count == 3
    assert report.output_files_count >= 1
    assert report.total_rows == 30
    assert output_dir.exists()

    output_files = list(output_dir.glob("*.parquet"))

    assert len(output_files) == report.output_files_count


def test_dry_run_does_not_create_output_files(tmp_path: Path):
    input_dir = tmp_path / "input"
    output_dir = tmp_path / "output"

    input_dir.mkdir()

    create_test_parquet_file(input_dir / "file_1.parquet", 0, 10)
    create_test_parquet_file(input_dir / "file_2.parquet", 10, 10)

    config = CompactionConfig(
        input_dir=input_dir,
        output_dir=output_dir,
        target_size_bytes=1024 * 1024,
        dry_run=True,
    )

    report = compact_parquet_files(config)

    assert report.dry_run is True
    assert report.input_files_count == 2
    assert report.output_files_count >= 1
    assert not output_dir.exists()