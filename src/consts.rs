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

pub const SERVICES: &[&str] = &[
    "casaos.service",
    "casaos-gateway.service",
    "casaos-user-service.service",
    "casaos-local-storage.service",
    "casaos-app-management.service",
    "casaos-message-bus.service",
    "rclone.service",
    "devmon@devmon.service",
];

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