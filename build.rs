use std::fs;

const REPO: &str = "https://github.com/coherentgraphics/cpdflib-binary";

#[tokio::main]
async fn main() {
    let mode = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let target_path = format!("target/{mode}/libcpdf.dll");

    if fs::metadata(&target_path).is_ok() {
        return;
    }

    let pointer_width = match true {
        cfg!(target_pointer_width = "32") => "32",
        cfg!(target_pointer_width = "64") => "64",
        _ => panic!("Unsuportted target pointer width."),
    };

    download_binary(&pointer_width, false).await;
    println!("cargo:rustc-link-lib=libs/libcpdf-x{pointer_width}");

    std::fs::copy(format!("libs/libcpdf-x{pointer_width}.dll"), &target_path).unwrap();
}

async fn download_binary(pointer_width: &str, force: bool) {
    let out_filename = format!("libs/libcpdf-x{pointer_width}.dll");
    if !force && fs::metadata(&out_filename).is_ok() {
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

    tokio::fs::write(out_filename, contents).await.unwrap();
}
