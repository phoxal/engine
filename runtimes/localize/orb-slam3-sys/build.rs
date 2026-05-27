use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=ORB_SLAM3_DIR");
    println!("cargo:rerun-if-env-changed=PANGOLIN_DIR");
    println!("cargo:rerun-if-changed=wrapper/wrapper.h");
    println!("cargo:rerun-if-changed=wrapper/wrapper.cpp");
    println!("cargo:rustc-check-cfg=cfg(orb_slam3_linked)");

    let Some(orb_slam3_dir) = env::var_os("ORB_SLAM3_DIR").map(PathBuf::from) else {
        println!(
            "cargo:warning=ORB_SLAM3_DIR is not set; orb-slam3-sys built metadata-only and will not link ORB-SLAM3"
        );
        return;
    };

    let pangolin_dir = env::var_os("PANGOLIN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/deploy"));
    let orb_lib_dir = orb_slam3_dir.join("lib");
    let orb_include_dir = orb_slam3_dir.join("include");
    let orb_camera_models_include_dir = orb_include_dir.join("CameraModels");
    let dbow2_lib_dir = orb_slam3_dir.join("Thirdparty/DBoW2/lib");
    let g2o_lib_dir = orb_slam3_dir.join("Thirdparty/g2o/lib");
    let sophus_include_dir = orb_slam3_dir.join("Thirdparty/Sophus");
    let pangolin_lib_dir = pangolin_dir.join("lib");
    let pangolin_include_dir = pangolin_dir.join("include");

    if !orb_lib_dir.join("libORB_SLAM3.so").exists() {
        panic!(
            "ORB_SLAM3_DIR must contain lib/libORB_SLAM3.so, checked {}",
            orb_lib_dir.display()
        );
    }
    if !orb_include_dir.is_dir() {
        panic!(
            "ORB_SLAM3_DIR must contain include/, checked {}",
            orb_include_dir.display()
        );
    }

    let opencv = OpenCv::detect();

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("wrapper/wrapper.cpp")
        .include("wrapper")
        .include(&orb_slam3_dir)
        .include(&orb_include_dir)
        .include(&orb_camera_models_include_dir)
        .include(&sophus_include_dir)
        .include(&pangolin_include_dir)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-fPIC");
    for include in &opencv.includes {
        build.include(include);
    }
    for flag in &opencv.cflags {
        build.flag(flag);
    }
    build.compile("orb_slam3_rust_wrapper");

    println!("cargo:rustc-cfg=orb_slam3_linked");
    emit_link_search(&orb_lib_dir);
    emit_link_search(&dbow2_lib_dir);
    emit_link_search(&g2o_lib_dir);
    emit_link_search(&pangolin_lib_dir);
    for lib_dir in &opencv.link_search {
        emit_link_search(lib_dir);
    }

    emit_link_lib("ORB_SLAM3");
    emit_link_lib("DBoW2");
    emit_link_lib("g2o");
    for lib in &opencv.libs {
        emit_link_lib(lib);
    }
    if opencv.libs.is_empty() {
        emit_link_lib("opencv_core");
        emit_link_lib("opencv_imgproc");
        emit_link_lib("opencv_features2d");
        emit_link_lib("opencv_calib3d");
    }
    emit_link_lib("pangolin");
    emit_link_lib("stdc++");
}

struct OpenCv {
    includes: Vec<PathBuf>,
    cflags: Vec<String>,
    link_search: Vec<PathBuf>,
    libs: Vec<String>,
}

impl OpenCv {
    fn detect() -> Self {
        let Ok(output) = Command::new("pkg-config")
            .args(["--cflags", "--libs", "opencv4"])
            .output()
        else {
            return Self::fallback();
        };
        if !output.status.success() {
            return Self::fallback();
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut includes = Vec::new();
        let mut cflags = Vec::new();
        let mut link_search = Vec::new();
        let mut libs = Vec::new();
        for token in stdout.split_whitespace() {
            if let Some(path) = token.strip_prefix("-I") {
                includes.push(PathBuf::from(path));
            } else if let Some(path) = token.strip_prefix("-L") {
                link_search.push(PathBuf::from(path));
            } else if let Some(lib) = token.strip_prefix("-l") {
                libs.push(lib.to_string());
            } else if token.starts_with("-") {
                cflags.push(token.to_string());
            }
        }

        Self {
            includes,
            cflags,
            link_search,
            libs,
        }
    }

    fn fallback() -> Self {
        Self {
            includes: Vec::new(),
            cflags: Vec::new(),
            link_search: Vec::new(),
            libs: vec![
                "opencv_core".to_string(),
                "opencv_imgproc".to_string(),
                "opencv_features2d".to_string(),
                "opencv_calib3d".to_string(),
            ],
        }
    }
}

fn emit_link_search(path: &Path) {
    println!("cargo:rustc-link-search=native={}", path.display());
}

fn emit_link_lib(lib: &str) {
    println!("cargo:rustc-link-lib={lib}");
}
