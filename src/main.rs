mod menuentry;

use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use glob::glob;
use regex::{Captures, Regex};
use log::{debug, info, warn};
use colored::*;
use crate::menuentry::MenuEntry;

struct UKIInfo {
    name: Arc<str>,
    machine_id: Arc<str>,
    build_id: Arc<str>,
}

impl UKIInfo {
    fn new(name: &str, machine_id: &str, build_id: &str) -> Self {
        UKIInfo {
            name: Arc::from(name),
            machine_id: Arc::from(machine_id),
            build_id: Arc::from(build_id),
        }
    }
}

fn get_info_from_file_name<'h>(prefix: &String, suffix: &String, file_name: &OsStr, pattern: Option<Regex>) -> Option<UKIInfo> {
    let pattern = pattern.unwrap_or(Regex::new(
        format!(r"{}([a-z\-]+?)-([a-z0-9]{{32}})-(rolling){}", regex::escape(prefix.as_str()), regex::escape(suffix)).as_str()
    ).unwrap());

    let (_, [name, machine_id, build_id]) = pattern.captures(file_name.to_str().unwrap())?.extract();
    Some(UKIInfo::new(name, machine_id, build_id))
}

macro_rules! info_from {
    ($prefix:expr, $suffix:expr, $file_name:expr) => { get_info_from_file_name($prefix, $suffix, $file_name, None) };
    ($prefix:expr, $suffix:expr, $file_name:expr, $pattern:expr) => { get_info_from_file_name($prefix, $suffix, $file_name, $pattern) }
}

fn add_uki_entry(prefix: &String, suffix: &String, uki_file: PathBuf, uki_path: String) {
    let filename = uki_file.file_name().unwrap();

    let info_option = info_from!(prefix, suffix, filename);
    if info_option.is_none() {
        return;
    }
    let info = info_option.unwrap();

    info!("{} entry for {}", "adding".green(), info.name.blue());
    std::io::stdout().write(
        MenuEntry::builder()
            .name(format!("{} ({})", whoami::distro(), info.name).as_ref())
            .insmod("fat")
            .chainloader(Path::new(&uki_path).join(filename).to_str().unwrap()).clone()
            .build()
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

    info!("{} in {}", "search".green(), uki_dir.to_str().unwrap().blue());

    for uki_entry in glob(
        format!("{}*{uki_suffix}", glob_path.as_path().to_str().unwrap()).as_str()
    ).expect("failed to glob UKIs") {
        match uki_entry {
            Ok(uki_file) => add_uki_entry(&uki_prefix, &uki_suffix, uki_file, uki_path.clone()),
            Err(e) => warn!("fail: {:?}", e)
        }
    }
}
