use lazy_static::lazy_static;
use std::borrow::ToOwned;

lazy_static! {
    pub static ref CONF_PATH: String = "/etc/casaos".to_owned();
    pub static ref RUN_PATH: String = "/var/run/casaos".to_owned();
    pub static ref LOG_PATH: String = "/var/log/casaos".to_owned();
    pub static ref HELPER_PATH: String = "/usr/share/casaos".to_owned();
    pub static ref MANIFEST: String = "/var/lib/casaos/manifest".to_owned();
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
    pub static ref CASAOS: Vec<String> = vec![
        "casaos",
        "casaos-gateway",
        "casaos-user-service",
        "casaos-local-storage",
        "casaos-app-management",
        "casaos-message-bus",
        "rclone",
        "casaos-ui",
        "casaos-cli",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    pub static ref SERVICES: Vec<String> = vec![
        "casaos.service",
        "casaos-gateway.service",
        "casaos-user-service.service",
        "casaos-local-storage.service",
        "casaos-app-management.service",
        "casaos-message-bus.service",
        "rclone.service",
        "devmon@devmon.service"
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
}
