extern crate curl;
extern crate flate2;
extern crate tar;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use curl::easy::Easy;
use flate2::read::GzDecoder;
use tar::Archive;

const LIBRARY: &'static str = "tensorflow";
// `VERSION` and `TAG` are separate because the tag is not always `'v' + VERSION`.
const VERSION: &'static str = "1.0.0";

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));
macro_rules! log {
    ($fmt:expr) => (println!(concat!("libtensorflow-sys/build.rs:{}: ", $fmt), line!()));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("libtensorflow-sys/build.rs:{}: ", $fmt),
    line!(), $($arg)*));
}
macro_rules! log_var(($var:ident) => (log!(concat!(stringify!($var), " = {:?}"), $var)));

fn main() {
    install_prebuilt();
}

fn remove_suffix(value: &mut String, suffix: &str) {
    if value.ends_with(suffix) {
        let n = value.len();
        value.truncate(n - suffix.len());
    }
}

fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
    let file = File::open(archive_path).unwrap();
    let unzipped = GzDecoder::new(file).unwrap();
    let mut a = Archive::new(unzipped);
    a.unpack(extract_to).unwrap();
}

// Downloads and unpacks a prebuilt binary. Only works for certain platforms.
fn install_prebuilt() {
    // Figure out the file names.
    let os = match env::consts::OS {
        "macos" => "darwin",
        x => x,
    };
    let proc_type = "cpu";
    let binary_url = format!(
        "https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-{}-{}-{}-{}.tar.gz",
        proc_type, os, env::consts::ARCH, VERSION);
    log_var!(binary_url);
    let short_file_name = binary_url.split("/").last().unwrap();
    let mut base_name = short_file_name.to_string();
    remove_suffix(&mut base_name, ".tar.gz");
    log_var!(base_name);
    let download_dir = match env::var("TF_RUST_DOWNLOAD_DIR") {
        Ok(s) => PathBuf::from(s),
        Err(_) => PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("target"),
    };
    if !download_dir.exists() {
        fs::create_dir(&download_dir).unwrap();
    }
    let file_name = download_dir.join(short_file_name);
    log_var!(file_name);

    // Download the tarball.
    if !file_name.exists() {
        let f = File::create(&file_name).unwrap();
        let mut writer = BufWriter::new(f);
        let mut easy = Easy::new();
        easy.url(&binary_url).unwrap();
        easy.write_function(move |data| {
            Ok(writer.write(data).unwrap())
        }).unwrap();
        easy.perform().unwrap();

        let response_code = easy.response_code().unwrap();
        if response_code != 200 {
            panic!("Unexpected response code {} for {}", response_code, binary_url);
        }
    }

    // Extract the tarball.
    let unpacked_dir = download_dir.join(base_name);
    let lib_dir = unpacked_dir.join("lib");
    if !lib_dir.join(format!("lib{}.so", LIBRARY)).exists() {
        extract(file_name, &unpacked_dir);
    }

    //run("find", |command| command); // TODO: remove
    run("ls", |command| {
        command.arg("-l").arg(lib_dir.to_str().unwrap())
        }); // TODO: remove

    println!("cargo:rustc-link-lib=dylib={}", LIBRARY);
    println!("cargo:rustc-link-search={}", lib_dir.display());
}

fn run<F>(name: &str, mut configure: F)
    where F: FnMut(&mut Command) -> &mut Command
{
    let mut command = Command::new(name);
    let configured = configure(&mut command);
    log!("Executing {:?}", configured);
    if !ok!(configured.status()).success() {
        panic!("failed to execute {:?}", configured);
    }
    log!("Command {:?} finished successfully", configured);
}
