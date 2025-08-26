use toolchest::{hash, io};

fn main() {
    let s = "hello";
    println!(
        "djb2={} fnv1a={}",
        hash::djb2(s.as_bytes()),
        hash::fnv1a(s.as_bytes())
    );

    let _ = io::ensure_dir("target/tmp");
    let _ = io::write_atomic("target/tmp/example.txt", b"content");
    #[cfg(feature = "fs")]
    {
        let files = io::find_files("target", "example").unwrap();
        println!("found {} files", files.len());
    }
}
