use std::env;

pub fn get_os() -> String {
    if env::var("GRUB_DISTRIBUTOR").is_err() {
        let distro = whoami::distro();
        if distro.starts_with("Unknown") {
            "Linux".to_string()
        } else {
            distro
        }
    } else {
        format!("{} Linux", env::var("GRUB_DISTRIBUTOR").unwrap())
    }
}