mod util;
mod info;

use std::env;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use log::{debug, info, warn};
use colored::*;
use glob::glob;
use once_cell::sync::Lazy;
use grub_mkconfig_lib::menu_entry::MenuEntry;
use crate::util::get_os;

#[allow(dead_code)]
struct UKIInfo {
    name: Arc<str>,
    machine_id: Arc<str>,
    build_id: Arc<str>,
}

static OS: Lazy<Arc<String>> = Lazy::new(||Arc::from(get_os()));

fn add_uki_entry(prefix: &String, suffix: &String, uki_file: PathBuf, uki_path: String) {
    let filename = uki_file.file_name().unwrap();

    let info_option = info_from!(prefix, suffix, filename);
    if info_option.is_none() {
        return;
    }
    let info = info_option.unwrap();

    info!("{} entry for {}", "adding".green(), info.name.blue());
    debug!("{} build: {}, {}", info.name.blue(), info.build_id.cyan(), info.machine_id.cyan());
    std::io::stdout().write(
        MenuEntry::builder()
            .name(format!("{} ({})", OS.deref(), info.name).as_ref())
            .insmod("fat")
            .save_default()
            .chainloader(Path::new(&uki_path).join(filename).to_str().unwrap())
            .generate()
            .as_ref()
    ).unwrap();
    std::io::stdout().write(b"\n").unwrap();
}

fn main() {
    env_logger::init();

    let binding = env::var("ESP").unwrap_or("/boot".into());
    let esp = Path::new(&binding);
    let uki_path = env::var("UKI_PATH").unwrap_or("EFI/Linux/".into());
    let uki_dir = esp.join(uki_path.clone());
    let uki_prefix = env::var("UKI_PREFIX").unwrap_or("uki-".into());
    let uki_suffix = ".efi".to_string();
    let glob_path = uki_dir.join(uki_prefix.clone());

    info!("{} in {}", "search".green(), uki_dir.to_string_lossy().blue());

    for uki_entry in glob(
        format!("{}*{uki_suffix}", glob_path.as_path().to_str().unwrap()).as_str()
    ).expect("failed to glob UKIs") {
        match uki_entry {
            Ok(uki_file) => add_uki_entry(&uki_prefix, &uki_suffix, uki_file, uki_path.clone()),
            Err(e) => warn!("fail: {:?}", e)
        }
    }
}
