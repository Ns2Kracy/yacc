#[allow(dead_code)]
pub const DEPENDENCIES: &[&str] = &[
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
];

pub const CASA_SERVICES: &[&str] = &[
    "casaos-gateway.service",
    "casaos-message-bus.service",
    "casaos-user-service.service",
    "casaos-local-storage.service",
    "casaos-app-management.service",
    "rclone.service",
    // casaos.service must be the last one so update from UI can work
    "casaos.service",
];

#[allow(dead_code)]
pub const CASAOS: &[&str] = &[
    "casaos",
    "casaos-gateway",
    "casaos-user-service",
    "casaos-local-storage",
    "casaos-app-management",
    "casaos-message-bus",
    "rclone",
    "casaos-ui",
    "casaos-cli",
];

#[allow(dead_code)]
pub static CASA_PACKAGES: &[&str] = &[
    "CasaOS-Gateway",
    "CasaOS-MessageBus",
    "CasaOS-UserService",
    "CasaOS-LocalStorage",
    "CasaOS-AppManagement",
    "CasaOS",
    "CasaOS-CLI",
    "CasaOS-UI",
    "CasaOS-AppStore",
];
