use parquet_compactor_rs::file_utils::{human_readable_size, parse_size};

#[test]
fn test_parse_size_mb() {
    let result = parse_size("128MB").unwrap();

    assert_eq!(result, 128 * 1024 * 1024);
}

#[test]
fn test_parse_size_gb() {
    let result = parse_size("1GB").unwrap();

    assert_eq!(result, 1024 * 1024 * 1024);
}

#[test]
fn test_parse_size_invalid() {
    let result = parse_size("128");

    assert!(result.is_err());
}

#[test]
fn test_human_readable_size_mb() {
    let result = human_readable_size(1024 * 1024);

    assert_eq!(result, "1.00 MB");
}