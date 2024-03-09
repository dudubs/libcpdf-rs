use std::{env::current_exe, fs, path::PathBuf};

const REPO: &str = "https://github.com/coherentgraphics/cpdflib-binary";

#[tokio::main]
async fn main() {
    let mode = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let libs_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("libs");

    let target_dir = resolve_target_dir();

    let target_dll_path = target_dir.join(mode).join("libcpdf.dll");

    if fs::metadata(&target_dll_path).is_ok() {
        return;
    }

    let pointer_width = match true {
        cfg!(target_pointer_width = "32") => "32",
        cfg!(target_pointer_width = "64") => "64",
        _ => panic!("Unsuportted target pointer width."),
    };

    let source_dll_path = libs_dir.join(format!("libcpdf-x{pointer_width}.dll"));
    println!("cargo:rustc-link-lib=libs/libcpdf-x{pointer_width}");

    download_binary(&source_dll_path, &pointer_width, false).await;

    std::fs::copy(&source_dll_path, &target_dll_path).unwrap();
}

fn resolve_target_dir() -> PathBuf {
    let mut path = current_exe().unwrap();
    while path.file_name().unwrap() != "target" {
        path = path.parent().unwrap().to_path_buf();
    }
    path
}

async fn download_binary(dll_path: &PathBuf, pointer_width: &str, force: bool) {
    if !force && fs::metadata(&dll_path).is_ok() {
        return;
    }
    let contents = reqwest::get(format!(
        "{REPO}/raw/master/windows{pointer_width}/libcpdf.dll"
    ))
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap();

    tokio::fs::write(dll_path, contents).await.unwrap();
}
