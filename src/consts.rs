use lazy_static::lazy_static;
use std::borrow::ToOwned;

lazy_static! {
    pub static ref TEMP_PATH: String = String::from("/tmp/casaos");
    pub static ref CONFIG_PATH: String = String::from("/etc/casaos");
    pub static ref DEPENDENCIES: Vec<String> = vec![
        "wget",
        "curl",
        "smartmontools",
        "parted",
        "ntfs-3g",
        "net-tools",
        "udevil",
        "samba",
        "cifs-utils",
        "mergerfs",
        "unzip",
        "apparmor",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
}