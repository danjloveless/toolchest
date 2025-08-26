//! IO helpers (std-only).
//!
//! Utilities for simple file and directory operations, including reading
//! lines, atomic writes, ensuring directories exist, copying directory trees,
//! and optional recursive file search (behind the `fs` feature).
//!
//! Examples:
//! ```rust
//! use toolchest::io::{ensure_dir, write_atomic, read_lines};
//! use std::path::PathBuf;
//! use std::fs;
//!
//! let dir = PathBuf::from("target/tmp_docs_io");
//! let _ = ensure_dir(&dir);
//! let file = dir.join("hello.txt");
//! write_atomic(&file, b"hello\nworld\n").unwrap();
//! let lines = read_lines(&file).unwrap();
//! assert_eq!(lines, vec!["hello", "world"]);
//! fs::remove_file(&file).ok();
//! fs::remove_dir(&dir).ok();
//! ```

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Read a text file as lines into `Vec<String>`.
///
/// Example:
/// ```rust
/// use toolchest::io::{write_atomic, read_lines};
/// let path = std::path::PathBuf::from("target/tmp_read_lines.txt");
/// write_atomic(&path, b"a\nb\n").unwrap();
/// assert_eq!(read_lines(&path).unwrap(), vec!["a", "b"]);
/// std::fs::remove_file(&path).ok();
/// ```
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|s| s.to_string())
        .collect())
}

/// Atomically write data to a file.
///
/// Writes to a temporary file and then renames into place.
///
/// Example:
/// ```rust
/// use toolchest::io::write_atomic;
/// let path = std::path::PathBuf::from("target/tmp_atomic.txt");
/// write_atomic(&path, b"hello").unwrap();
/// let s = std::fs::read_to_string(&path).unwrap();
/// assert_eq!(s, "hello");
/// std::fs::remove_file(&path).ok();
/// ```
pub fn write_atomic<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    let mut tmp = PathBuf::from(path);
    tmp.set_extension(".tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    fs::rename(tmp, path)?;
    Ok(())
}

/// Create a directory and parents if needed.
///
/// Example:
/// ```rust
/// use toolchest::io::ensure_dir;
/// let p = std::path::PathBuf::from("target/tmp_dir/docs");
/// ensure_dir(&p).unwrap();
/// assert!(p.exists());
/// std::fs::remove_dir_all("target/tmp_dir").ok();
/// ```
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Recursively copy a directory tree.
///
/// Example (best-effort cleanup):
/// ```rust
/// use toolchest::io::{ensure_dir, copy_dir, write_atomic};
/// use std::path::PathBuf;
/// let src = PathBuf::from("target/tmp_src");
/// let dst = PathBuf::from("target/tmp_dst");
/// ensure_dir(&src).unwrap();
/// write_atomic(src.join("a.txt"), b"hi").unwrap();
/// copy_dir(&src, &dst).unwrap();
/// assert!(dst.join("a.txt").exists());
/// std::fs::remove_dir_all(&src).ok();
/// std::fs::remove_dir_all(&dst).ok();
/// ```
pub fn copy_dir<P: AsRef<Path>>(src: P, dst: P) -> io::Result<()> {
    fn rec(src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let sp = entry.path();
            let dp = dst.join(entry.file_name());
            if ty.is_dir() {
                rec(&sp, &dp)?;
            } else {
                fs::copy(&sp, &dp)?;
            }
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
        let entry = entry.map_err(|e| io::Error::other(e.to_string()))?;
        if entry.file_type().is_file() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(&pat) {
                out.push(entry.path().to_path_buf());
            }
        }
    }
    Ok(out)
}

#[cfg(not(feature = "fs"))]
/// Find files under root matching pattern (stub)
pub fn find_files<P: AsRef<Path>>(_root: P, _pattern: &str) -> io::Result<Vec<PathBuf>> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "find_files requires the 'fs' feature",
    ))
}
