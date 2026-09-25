#[macro_export]
macro_rules! write {
    ($path: literal, $content: expr) => {{
        fn __write_inner() -> Result<(), std::io::Error> {
            use std::io::Write;
            use std::fs::File;
            let mut __file = File::create($crate::build_path!($path))?;
            __file.write_all($content)
        }

        __write_inner()
    }};
}

#[macro_export]
macro_rules! build_path {
    ($path: literal) => {{
        std::env::var("OUT_DIR").unwrap() + "/" + $path
    }};
}