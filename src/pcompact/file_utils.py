import re
from pathlib import Path


SIZE_UNITS = {
    "B": 1,
    "KB": 1024,
    "MB": 1024 * 1024,
    "GB": 1024 * 1024 * 1024,
}


def parse_size(size_text: str) -> int:
    """
    Converts a human-readable size like '128MB' or '1GB'
    into bytes.

    Example:
        128MB -> 134217728 bytes
    """

    cleaned_text = size_text.strip().upper()

    match = re.fullmatch(r"(\d+(?:\.\d+)?)(B|KB|MB|GB)", cleaned_text)

    if not match:
        raise ValueError(
            "Invalid size format. Use formats like 128MB, 256MB, or 1GB."
        )

    number_part = float(match.group(1))
    unit_part = match.group(2)

    return int(number_part * SIZE_UNITS[unit_part])


def human_readable_size(size_bytes: int) -> str:
    """
    Converts bytes into a readable format.

    Example:
        134217728 -> 128.00 MB
    """

    if size_bytes < 1024:
        return f"{size_bytes} B"

    if size_bytes < 1024 * 1024:
        return f"{size_bytes / 1024:.2f} KB"

    if size_bytes < 1024 * 1024 * 1024:
        return f"{size_bytes / (1024 * 1024):.2f} MB"

    return f"{size_bytes / (1024 * 1024 * 1024):.2f} GB"


def find_parquet_files(input_dir: Path) -> list[Path]:
    """
    Finds all Parquet files inside the input directory.

    It searches recursively, meaning it also checks subfolders.
    """

    if not input_dir.exists():
        raise FileNotFoundError(f"Input directory does not exist: {input_dir}")

    if not input_dir.is_dir():
        raise NotADirectoryError(f"Input path is not a directory: {input_dir}")

    parquet_files = sorted(input_dir.rglob("*.parquet"))

    return parquet_files


def get_file_size(file_path: Path) -> int:
    """
    Returns the size of one file in bytes.
    """

    return file_path.stat().st_size


def get_total_size(file_paths: list[Path]) -> int:
    """
    Returns the total size of all given files.
    """

    return sum(get_file_size(file_path) for file_path in file_paths)


def create_output_directory(output_dir: Path) -> None:
    """
    Creates the output directory if it does not already exist.
    """

    output_dir.mkdir(parents=True, exist_ok=True)