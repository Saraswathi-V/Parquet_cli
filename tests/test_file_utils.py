from pathlib import Path

import pytest

from pcompact.file_utils import (
    parse_size,
    human_readable_size,
    find_parquet_files,
)


def test_parse_size_mb():
    result = parse_size("128MB")

    assert result == 128 * 1024 * 1024


def test_parse_size_gb():
    result = parse_size("1GB")

    assert result == 1024 * 1024 * 1024


def test_parse_size_invalid_value():
    with pytest.raises(ValueError):
        parse_size("128")


def test_human_readable_size():
    result = human_readable_size(1024 * 1024)

    assert result == "1.00 MB"


def test_find_parquet_files(tmp_path: Path):
    parquet_file = tmp_path / "data.parquet"
    text_file = tmp_path / "notes.txt"

    parquet_file.write_text("fake parquet content")
    text_file.write_text("not a parquet file")

    result = find_parquet_files(tmp_path)

    assert len(result) == 1
    assert result[0].name == "data.parquet"