use crate::consts::CASA_SERVICES;
use crate::utils::confirm::confirm_default_no;
use crate::{print_error, print_info, print_output, print_warn};
use anyhow::Error;

/// Uninstall CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args;

pub fn run(_cmd: Args) -> anyhow::Result<(), Error> {
    let uninstall = uninstall_casaos();
    match uninstall {
        Ok(_) => {
            print_output!("Uninstall CasaOS successfully.");
        }
        Err(e) => {
            print_error!("Failed to uninstall CasaOS.\n{:?}", e);
        }
    }
    Ok(())
}

fn uninstall_casaos() -> anyhow::Result<(), Error> {
    // detect casaos files
    let exist = detect_casaos()?;
    if !exist {
        print_error!("CasaOS is not detected, exit the script.");
    }

    print_output!("This script will delete the containers you no longer use, and the CasaOS configuration files.");

    // stop and remove all containers

    match confirm_default_no("Do you want delete all containers?") {
        Ok(true) => {
            print_info!("Start deleting all containers.");
            uninstall_containers()?;
        }
        Ok(false) => {
            print_info!("Skip delete containers.");
        }
        Err(e) => {
            print_error!("Failed to confirm.\n{:?}", e);
        }
    }

    // remove images
    if dialoguer::Confirm::new()
        .with_prompt("Do you want delete all images?")
        .default(true)
        .show_default(true)
        .interact()
        .unwrap()
    {
        print_info!("Start deleting all images.");
        remove_images(true)?;
    } else {
        print_info!("Start deleting unused images.");
        remove_images(false)?;
    }

    // stop and disable services
    stop_and_remove_service()?;

    // remove casaos files
    remove_files()?;

    Ok(())
}

fn detect_casaos() -> anyhow::Result<bool, Error> {
    Ok(std::path::Path::new("/usr/bin/casaos").exists())
}

fn uninstall_containers() -> anyhow::Result<(), Error> {
    let command = std::process::Command::new("docker")
        .args(["stop", "$(docker ps -aq)"])
        .status()?
        .success();

    if !command {
        print_warn!("Failed to stop containers.");
    }

    // remove all containers
    let command = std::process::Command::new("docker")
        .args(["rm", "$(docker ps -aq)"])
        .status()?
        .success();

    if !command {
        print_warn!("Failed to delete all containers.");
    }

    Ok(())
}

fn remove_images(confirm: bool) -> anyhow::Result<(), Error> {
    if !confirm {
        let command = std::process::Command::new("docker")
            .args(["image", "prune", "-af"])
            .status()?
            .success();

        if !command {
            print_warn!("Failed to remove unused images.");
        }
        return Ok(());
    }

    let command = std::process::Command::new("docker")
        .args(["rmi", "$(docker images -aq)"])
        .status()?
        .success();

    if !command {
        print_warn!("Failed to remove all images.");
    }

    Ok(())
}

fn stop_and_remove_service() -> anyhow::Result<(), Error> {
    let services = CASA_SERVICES;

    for service in services {
        print_info!("Stopping {} ...", service);
        let command = std::process::Command::new("systemctl")
            .args(["stop", service])
            .status()?
            .success();

        if !command {
            print_warn!("Failed to stop service: {}", service);
        }

        print_info!("Disabling {} ...", service);
        let command = std::process::Command::new("systemctl")
            .args(["disable", service])
            .status()?
            .success();

        if !command {
            print_warn!("Failed to disable service: {}", service);
        }
    }

    Ok(())
}

fn remove_files() -> anyhow::Result<(), Error> {
    let files = vec![
        "/usr/lib/systemd/system/casaos.service",
        "lib/systemd/system/casaos.service",
        "/etc/systemd/system/casaos.service",
        "/etc/casaos",
        "/etc/udev/rules.d/11-usb-mount.rules",
        "/etc/systemd/system/usb-mount@.service",
        "/usr/local/bin/casaos",
        "/etc/casaos.conf",
        "/var/lib/casaos/[0-9]*",
        "/var/lib/casaos/db",
        "/var/lib/casaos/*.db",
        "/var/lib/casaos/www",
        "/var/lib/casaos/migration",
        "/usr/share/casaos",
        "/var/log/casaos",
        "/etc/casaos",
        "/var/run/casaos",
        "/usr/bin/casaos-uninstall",
    ];
    for file in files {
        std::fs::remove_dir_all(file)?;
    }

    let manifest = std::fs::read_to_string("/etc/casaos/manifest")?;
    for line in manifest.lines() {
        std::fs::remove_dir_all(line)?;
    }
    std::fs::remove_file("/var/lib/casaos/manifest")?;
    std::fs::remove_dir_all("/var/lib/casaos")?;

    if dialoguer::Confirm::new()
        .with_prompt("Do you want delete all app data?")
        .default(true)
        .show_default(true)
        .interact()
        .unwrap()
    {
        std::fs::remove_dir_all("/DATA/AppData")?;
    }

    Ok(())
}
