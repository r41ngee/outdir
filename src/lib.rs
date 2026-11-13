#![doc = include_str!("../README.md")]
mod build_macros;


/// Macro used to read from `build.rs` outputted Rust source file
///
/// Shortening of `include!(concat!(env!("OUT_DIR"), "/", "path"))`.
/// # Example
/// ```ignore
/// // build.rs
/// outdir::write!("some_module.rs", rust_content.as_bytes());
///
/// // In your crate you can use this:
/// outdir::outdir!("some_module.rs");
/// ```
#[macro_export]
macro_rules! outdir {
    ($x: literal) => {
        include!($crate::path!($x));
    };
}

/// Macro used to read from `build.rs` outputted binary file as const `&[u8]`
///
/// Shortening of `include_bytes!(concat!(env!("OUT_DIR"), "/", "path"))`.
/// # Example
/// ```ignore
/// // build.rs
/// outdir::write!("some_binary.bin", byte_slice);
///
/// // In your crate you can use this:
/// const BINARY_CONTENT: &[u8] = outdir::outdir_bytes!("some_binary.bin");
/// ```
#[macro_export]
macro_rules! outdir_bytes {
    ($x: literal) => {
        include_bytes!($crate::path!($x))
    };
}

/// Macro used to read from `build.rs` outputted string as const [`&str`]
///
/// Shortening of `include_str!(concat!(env!("OUT_DIR"), "/", "path"))`.
/// # Example
/// ```ignore
/// // build.rs
/// outdir::write!("some_text.txt", some_string.as_bytes());
///
/// // In your crate you can use this:
/// const MESSAGE: &str = outdir::outdir_str!("some_text.txt");
/// ```
#[macro_export]
macro_rules! outdir_str {
    ($x: literal) => {
        include_str!($crate::path!($x))
    };
}

#[macro_export]
/// Internal macro used to calculate path in `OUT_DIR`
macro_rules! path {
    ($path: literal) => {
        concat!(env!("OUT_DIR"), "/", $path)
    };
}