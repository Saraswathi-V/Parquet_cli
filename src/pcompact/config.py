from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class CompactionConfig:
    """
    Stores all configuration values needed for the compaction job.

    frozen=True means once the object is created,
    its values should not be changed accidentally.
    """

    input_dir: Path
    output_dir: Path
    target_size_bytes: int
    dry_run: bool = False
    verbose: bool = False
    compression: str = "snappy"