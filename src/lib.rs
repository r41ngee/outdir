#![doc = include_str!("../README.md")]
pub mod build_macros;

#[macro_export]
macro_rules! outdir {
    ($x: literal) => {
        include!($crate::path!($x));
        compile_error!();
    };
}

#[macro_export]
macro_rules! outdir_bytes {
    ($x: literal) => {
        include_bytes!($crate::path!($x))
    };
}

#[macro_export]
macro_rules! outdir_str {
    ($x: literal) => {
        include_str!($crate::path!($x))
    };
}

#[macro_export]
macro_rules! path {
    ($path: literal) => {
        concat!(env!("OUT_DIR"), "/", $path)
    };
}