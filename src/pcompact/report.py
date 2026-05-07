from dataclasses import dataclass, field
from pathlib import Path

from pcompact.file_utils import human_readable_size


@dataclass
class CompactionReport:
    """
    Stores the final result of the compaction job.
    """

    input_files_count: int
    output_files_count: int
    total_input_size_bytes: int
    total_output_size_bytes: int
    target_size_bytes: int
    total_rows: int
    dry_run: bool
    output_files: list[Path] = field(default_factory=list)

    @property
    def file_compaction_ratio(self) -> float:
        """
        Shows how many input files became one output file on average.

        Example:
            100 input files / 10 output files = 10.0
        """

        if self.output_files_count == 0:
            return 0.0

        return self.input_files_count / self.output_files_count


def print_report(report: CompactionReport) -> None:
    """
    Prints the compaction summary in a clean format.
    """

    mode = "DRY RUN" if report.dry_run else "COMPACTION COMPLETED"

    print()
    print("=" * 60)
    print(f"Parquet Compactor Summary - {mode}")
    print("=" * 60)
    print(f"Input files              : {report.input_files_count}")
    print(f"Output files             : {report.output_files_count}")
    print(f"Total input size         : {human_readable_size(report.total_input_size_bytes)}")
    print(f"Total output size        : {human_readable_size(report.total_output_size_bytes)}")
    print(f"Target output file size  : {human_readable_size(report.target_size_bytes)}")
    print(f"Total rows processed     : {report.total_rows}")
    print(f"File compaction ratio    : {report.file_compaction_ratio:.2f}x")
    print("=" * 60)

    if report.output_files:
        print("Output files:")
        for output_file in report.output_files:
            print(f"  - {output_file}")

    print()