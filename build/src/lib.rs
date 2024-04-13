use std::path::PathBuf;

// x
pub fn build() {
    let pointer_width = get_target_pointer_width();

    let lib = get_libs_dir()
        .join(format!("libcpdf-x{pointer_width}"))
        .to_str()
        .unwrap()
        .to_string();

    println!("cargo:rustc-link-lib={lib}");
}

fn get_libs_dir() -> PathBuf {
    PathBuf::from(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("libs")
}

fn get_target_pointer_width<'a>() -> &'a str {
    match true {
        cfg!(target_pointer_width = "32") => "32",
        cfg!(target_pointer_width = "64") => "64",
        _ => panic!("Unsuportted target pointer width."),
    }
}
