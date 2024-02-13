use std::ffi::OsStr;
use std::sync::Arc;
use regex::Regex;
use crate::UKIInfo;

impl UKIInfo {
    fn new(name: &str, machine_id: &str, build_id: &str) -> Self {
        UKIInfo {
            name: Arc::from(name),
            machine_id: Arc::from(machine_id),
            build_id: Arc::from(build_id),
        }
    }
}

pub fn get_info_from_file_name<'h>(prefix: &String, suffix: &String, file_name: &OsStr, pattern: Option<Regex>) -> Option<UKIInfo> {
    let pattern = pattern.unwrap_or(Regex::new(
        format!(r"{}([a-z\-]+?)-([a-z0-9]{{32}})-([0-9A-Za-z_]*){}", regex::escape(prefix.as_str()), regex::escape(suffix)).as_str()
    ).unwrap());

    let (_, [name, machine_id, build_id]) = pattern.captures(file_name.to_str().unwrap())?.extract();
    Some(UKIInfo::new(name, machine_id, build_id))
}

#[macro_export]
macro_rules! info_from {
    ($prefix:expr, $suffix:expr, $file_name:expr) => { $crate::info::get_info_from_file_name($prefix, $suffix, $file_name, None) };
    ($prefix:expr, $suffix:expr, $file_name:expr, $pattern:expr) => { $crate::info::get_info_from_file_name($prefix, $suffix, $file_name, $pattern) }
}