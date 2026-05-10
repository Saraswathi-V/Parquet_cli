use crate::config::Config;
use crate::file_utils::{
    clean_old_parquet_files, create_output_directory, file_size, find_parquet_files, total_size,
};
use crate::report::CompactionReport;
use anyhow::{anyhow, Result};
use arrow_schema::SchemaRef;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub fn group_files_by_target_size(
    parquet_files: &[PathBuf],
    target_size_bytes: u64,
) -> Result<Vec<Vec<PathBuf>>> {
    let mut groups: Vec<Vec<PathBuf>> = Vec::new();
    let mut current_group: Vec<PathBuf> = Vec::new();
    let mut current_group_size = 0_u64;

    for file_path in parquet_files {
        let current_file_size = file_size(file_path)?;

        if !current_group.is_empty()
            && current_group_size + current_file_size > target_size_bytes
        {
            groups.push(current_group);

            current_group = vec![file_path.clone()];
            current_group_size = current_file_size;
        } else {
            current_group.push(file_path.clone());
            current_group_size += current_file_size;
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    Ok(groups)
}

fn validate_schema(
    expected_schema: &mut Option<SchemaRef>,
    current_schema: SchemaRef,
    input_file: &Path,
) -> Result<()> {
    match expected_schema {
        None => {
            *expected_schema = Some(current_schema);
            Ok(())
        }
        Some(schema) => {
            if schema.as_ref() != current_schema.as_ref() {
                Err(anyhow!(
                    "Schema mismatch found in file: {}",
                    input_file.display()
                ))
            } else {
                Ok(())
            }
        }
    }
}

fn write_compacted_file(input_files: &[PathBuf], output_file: &Path) -> Result<(u64, usize)> {
    let mut writer: Option<ArrowWriter<File>> = None;
    let mut expected_schema: Option<SchemaRef> = None;
    let mut total_rows_written = 0_usize;

    let writer_properties = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .set_dictionary_enabled(true)
        .build();

    for input_file in input_files {
        debug!("Reading input file: {}", input_file.display());

        let file = File::open(input_file)?;

        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?.with_batch_size(8192);

        let current_schema = builder.schema().clone();

        validate_schema(&mut expected_schema, current_schema.clone(), input_file)?;

        if writer.is_none() {
            let output = File::create(output_file)?;

            writer = Some(ArrowWriter::try_new(
                output,
                current_schema,
                Some(writer_properties.clone()),
            )?);
        }

        let mut reader = builder.build()?;

        while let Some(batch_result) = reader.next() {
            let batch = batch_result?;

            total_rows_written += batch.num_rows();

            if let Some(active_writer) = writer.as_mut() {
                active_writer.write(&batch)?;
            }
        }
    }

    if let Some(active_writer) = writer {
        active_writer.close()?;
    }

    let output_size_bytes = fs::metadata(output_file)?.len();

    Ok((output_size_bytes, total_rows_written))
}

pub fn compact_parquet_files(config: &Config) -> Result<CompactionReport> {
    let parquet_files = find_parquet_files(&config.input_dir)?;

    if parquet_files.is_empty() {
        return Err(anyhow!(
            "No Parquet files found in input directory: {}",
            config.input_dir.display()
        ));
    }

    let total_input_size = total_size(&parquet_files)?;

    info!("Found {} Parquet files", parquet_files.len());
    info!("Total input size: {} bytes", total_input_size);

    let file_groups = group_files_by_target_size(&parquet_files, config.target_size_bytes)?;

    info!("Planned output files: {}", file_groups.len());

    if config.dry_run {
        return Ok(CompactionReport {
            input_files_count: parquet_files.len(),
            output_files_count: file_groups.len(),
            total_input_size_bytes: total_input_size,
            total_output_size_bytes: 0,
            target_size_bytes: config.target_size_bytes,
            total_rows: 0,
            dry_run: true,
            output_files: vec![],
        });
    }

    create_output_directory(&config.output_dir)?;
    clean_old_parquet_files(&config.output_dir)?;

    let mut output_files = Vec::new();
    let mut total_output_size = 0_u64;
    let mut total_rows = 0_usize;

    for (index, group) in file_groups.iter().enumerate() {
        let output_file = config
            .output_dir
            .join(format!("part-{:05}.parquet", index + 1));

        info!(
            "Writing output file {} using {} input files",
            output_file.display(),
            group.len()
        );

        let (output_size, rows_written) = write_compacted_file(group, &output_file)?;

        output_files.push(output_file);
        total_output_size += output_size;
        total_rows += rows_written;
    }

    Ok(CompactionReport {
        input_files_count: parquet_files.len(),
        output_files_count: output_files.len(),
        total_input_size_bytes: total_input_size,
        total_output_size_bytes: total_output_size,
        target_size_bytes: config.target_size_bytes,
        total_rows,
        dry_run: false,
        output_files,
    })
}