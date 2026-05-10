use anyhow::Result;
use arrow_array::{ArrayRef, Float64Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn clean_old_parquet_files(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();

        let is_parquet = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.eq_ignore_ascii_case("parquet"))
            .unwrap_or(false);

        if is_parquet {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn random_text(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn build_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("transaction_id", DataType::Int64, false),
        Field::new("customer_id", DataType::Utf8, false),
        Field::new("city", DataType::Utf8, false),
        Field::new("status", DataType::Utf8, false),
        Field::new("product_category", DataType::Utf8, false),
        Field::new("amount", DataType::Float64, false),
        Field::new("transaction_notes", DataType::Utf8, false),
    ]))
}

fn generate_sample_parquet_files(
    output_dir: &Path,
    number_of_files: usize,
    rows_per_file: usize,
) -> Result<()> {
    clean_old_parquet_files(output_dir)?;

    let schema = build_schema();

    let cities = ["Atlanta", "Kennesaw", "Marietta", "Dallas", "Austin"];
    let statuses = ["COMPLETED", "PENDING", "FAILED", "REFUNDED"];
    let categories = ["Electronics", "Books", "Clothing", "Grocery", "Sports"];

    let repeated_note = "Parquet compaction demo transaction record. \
                         This repeated text helps demonstrate compression. ";

    let input_properties = WriterProperties::builder()
        .set_compression(Compression::UNCOMPRESSED)
        .set_dictionary_enabled(false)
        .build();

    let mut total_size_bytes = 0_u64;
    let mut rng = thread_rng();

    for file_number in 1..=number_of_files {
        let start_id = ((file_number - 1) * rows_per_file) as i64;
        let end_id = start_id + rows_per_file as i64;

        let transaction_ids = Int64Array::from_iter_values(start_id..end_id);

        let mut customer_ids = Vec::with_capacity(rows_per_file);
        let mut city_values = Vec::with_capacity(rows_per_file);
        let mut status_values = Vec::with_capacity(rows_per_file);
        let mut category_values = Vec::with_capacity(rows_per_file);
        let mut amounts = Vec::with_capacity(rows_per_file);
        let mut notes = Vec::with_capacity(rows_per_file);

        for _ in 0..rows_per_file {
            customer_ids.push(format!("CUST_{}", rng.gen_range(1..=2000)));

            city_values.push(cities[rng.gen_range(0..cities.len())].to_string());

            status_values.push(statuses[rng.gen_range(0..statuses.len())].to_string());

            category_values.push(categories[rng.gen_range(0..categories.len())].to_string());

            amounts.push(rng.gen_range(10.0..5000.0));

            notes.push(format!(
                "{} REF_{}",
                repeated_note.repeat(3),
                random_text(80)
            ));
        }

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(transaction_ids) as ArrayRef,
                Arc::new(StringArray::from(customer_ids)) as ArrayRef,
                Arc::new(StringArray::from(city_values)) as ArrayRef,
                Arc::new(StringArray::from(status_values)) as ArrayRef,
                Arc::new(StringArray::from(category_values)) as ArrayRef,
                Arc::new(Float64Array::from(amounts)) as ArrayRef,
                Arc::new(StringArray::from(notes)) as ArrayRef,
            ],
        )?;

        let output_file = output_dir.join(format!("small_file_{:03}.parquet", file_number));

        let file = File::create(&output_file)?;

        let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(input_properties.clone()))?;

        writer.write(&batch)?;
        writer.close()?;

        let file_size = fs::metadata(&output_file)?.len();
        total_size_bytes += file_size;

        println!(
            "Created: {} | Size: {:.2} MB",
            output_file.display(),
            file_size as f64 / (1024_f64 * 1024_f64)
        );
    }

    println!();
    println!("============================================================");
    println!("Generated {} Parquet files", number_of_files);
    println!("Rows per file: {}", rows_per_file);
    println!("Total rows: {}", number_of_files * rows_per_file);
    println!(
        "Total input size: {:.2} MB",
        total_size_bytes as f64 / (1024_f64 * 1024_f64)
    );
    println!("============================================================");

    Ok(())
}

fn main() -> Result<()> {
    let output_dir = PathBuf::from("sample_data/small_files");

    generate_sample_parquet_files(&output_dir, 100, 5000)?;

    Ok(())
}