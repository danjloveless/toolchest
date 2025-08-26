//! IO helpers (std-only)

use std::fs; use std::io::{self, Write}; use std::path::{Path, PathBuf};

/// Read a text file as lines into `Vec<String>`
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> { Ok(fs::read_to_string(path)?.lines().map(|s| s.to_string()).collect()) }

/// Atomically write data to a file
pub fn write_atomic<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    let mut tmp = PathBuf::from(path); tmp.set_extension(".tmp");
    { let mut f = fs::File::create(&tmp)?; f.write_all(data)?; f.sync_all()?; }
    fs::rename(tmp, path)?; Ok(())
}

/// Create a directory and parents if needed
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> { fs::create_dir_all(path) }

/// Recursively copy a directory tree
pub fn copy_dir<P: AsRef<Path>>(src: P, dst: P) -> io::Result<()> {
    fn rec(src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?; let ty = entry.file_type()?; let sp = entry.path(); let dp = dst.join(entry.file_name());
            if ty.is_dir() { rec(&sp, &dp)?; } else { fs::copy(&sp, &dp)?; }
        }
        Ok(())
    }
    rec(src.as_ref(), dst.as_ref())
}

#[cfg(feature = "fs")]
/// Recursively find files whose names contain `pattern` (case-insensitive)
pub fn find_files<P: AsRef<Path>>(root: P, pattern: &str) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let pat = pattern.to_lowercase();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        if entry.file_type().is_file() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(&pat) { out.push(entry.path().to_path_buf()); }
        }
    }
    Ok(out)
}

#[cfg(not(feature = "fs"))]
/// Find files under root matching pattern (stub)
pub fn find_files<P: AsRef<Path>>(_root: P, _pattern: &str) -> io::Result<Vec<PathBuf>> {
    Err(io::Error::new(io::ErrorKind::Other, "find_files requires the 'fs' feature"))
}


