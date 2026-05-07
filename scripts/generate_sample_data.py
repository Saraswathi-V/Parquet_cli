from pathlib import Path
import random
import string

import pyarrow as pa
import pyarrow.parquet as pq


def clean_old_parquet_files(output_dir: Path) -> None:
    """
    Deletes only old Parquet files from the output directory.
    """

    output_dir.mkdir(parents=True, exist_ok=True)

    for parquet_file in output_dir.glob("*.parquet"):
        parquet_file.unlink()


def random_text(length: int = 120) -> str:
    """
    Creates partly random text.
    Random text makes the input larger.
    Repeated patterns still allow compression in the output.
    """

    letters = string.ascii_letters + string.digits
    return "".join(random.choices(letters, k=length))


def generate_sample_parquet_files(
    output_dir: Path,
    number_of_files: int = 100,
    rows_per_file: int = 5000,
) -> None:
    """
    Generates demo Parquet data.

    Goal:
    - Input size around a few hundred MB
    - Output size clearly smaller after compaction
    """

    clean_old_parquet_files(output_dir)

    cities = ["Atlanta", "Kennesaw", "Marietta", "Dallas", "Austin"]
    statuses = ["COMPLETED", "PENDING", "FAILED", "REFUNDED"]
    categories = ["Electronics", "Books", "Clothing", "Grocery", "Sports"]

    repeated_note = (
        "Parquet compaction demo transaction record. "
        "This text repeats enough to allow output compression. "
    ) * 3

    total_size_bytes = 0

    for file_number in range(1, number_of_files + 1):
        transaction_ids = list(
            range(
                (file_number - 1) * rows_per_file,
                file_number * rows_per_file,
            )
        )

        customer_ids = [
            f"CUST_{random.randint(1, 2000)}"
            for _ in range(rows_per_file)
        ]

        city_values = [
            random.choice(cities)
            for _ in range(rows_per_file)
        ]

        status_values = [
            random.choice(statuses)
            for _ in range(rows_per_file)
        ]

        category_values = [
            random.choice(categories)
            for _ in range(rows_per_file)
        ]

        amounts = [
            round(random.uniform(10.0, 5000.0), 2)
            for _ in range(rows_per_file)
        ]

        # Mixed text:
        # repeated part helps compression
        # random part keeps input size realistic
        transaction_notes = [
            repeated_note + " REF_" + random_text(80)
            for _ in range(rows_per_file)
        ]

        table = pa.table(
            {
                "transaction_id": transaction_ids,
                "customer_id": customer_ids,
                "city": city_values,
                "status": status_values,
                "product_category": category_values,
                "amount": amounts,
                "transaction_notes": transaction_notes,
            }
        )

        output_file = output_dir / f"small_file_{file_number:03d}.parquet"

        # IMPORTANT:
        # Input files are intentionally uncompressed and inefficient.
        # This helps show size reduction after compaction.
        pq.write_table(
            table,
            output_file,
            compression=None,
            use_dictionary=False,
        )

        file_size_mb = output_file.stat().st_size / (1024 * 1024)
        total_size_bytes += output_file.stat().st_size

        print(f"Created: {output_file} | Size: {file_size_mb:.2f} MB")

    total_size_mb = total_size_bytes / (1024 * 1024)

    print()
    print("=" * 60)
    print(f"Generated {number_of_files} Parquet files")
    print(f"Rows per file: {rows_per_file}")
    print(f"Total rows: {number_of_files * rows_per_file}")
    print(f"Total input size: {total_size_mb:.2f} MB")
    print("=" * 60)


if __name__ == "__main__":
    generate_sample_parquet_files(
        output_dir=Path("sample_data/small_files"),
        number_of_files=100,
        rows_per_file=5000,
    )
    