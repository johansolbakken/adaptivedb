use std::io::Write;

fn wal_path() -> std::path::PathBuf {
    ".adaptivedb/wal.txt".into()
}

pub fn write(
    table_name: &str,
    column_name: &str,
    value: &str,
) {
    let path = wal_path();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("failed to open WAL file");
    writeln!(file, "w {} {} {:?}", table_name, column_name, value)
        .expect("failed to write to WAL file");
}