use crate::consts::SERVICES;
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
    if dialoguer::Confirm::new()
        .with_prompt("Do you want delete all containers?")
        .default(true)
        .show_default(true)
        .interact()
        .unwrap()
    {
        print_info!("Start deleting all containers.");
        uninstall_containers().unwrap();
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
        remove_images(true).unwrap();
    } else {
        print_info!("Start deeleting unused images.");
        remove_images(false).unwrap();
    }

    // stop and disable services
    stop_and_remove_service()?;

    // remove casaos files
    remove_files().unwrap();

    Ok(())
}

fn detect_casaos() -> anyhow::Result<bool, Error> {
    // detect casaos files
    let exist = std::path::Path::new("/usr/bin/casaos").exists();

    Ok(exist)
}

fn uninstall_containers() -> anyhow::Result<(), Error> {
    let command = std::process::Command::new("docker")
        .arg("stop")
        .arg("$(docker ps -aq)")
        .output()
        .unwrap();

    if !command.status.success() {
        print_warn!("Failed to stop containers.");
    }

    // remove all containers
    let command = std::process::Command::new("docker")
        .arg("rm")
        .arg("$(docker ps -aq)")
        .output()
        .unwrap();

    if !command.status.success() {
        print_warn!("Failed to delete all containers.");
    }

    Ok(())
}

fn remove_images(confirm: bool) -> anyhow::Result<(), Error> {
    if !confirm {
        let command = std::process::Command::new("docker")
            .arg("image")
            .arg("prune")
            .arg("-af")
            .output()?;

        if !command.status.success() {
            print_warn!("Failed to remove unused images.");
        }
        return Ok(());
    }

    let command = std::process::Command::new("docker")
        .arg("rmi")
        .arg("$(docker images -aq)")
        .output()?;

    if !command.status.success() {
        print_warn!("Failed to remove all images.");
    }

    Ok(())
}

fn stop_and_remove_service() -> anyhow::Result<(), Error> {
    let services = SERVICES;

    for service in services {
        let command = std::process::Command::new("systemctl")
            .args(vec!["disable --now", service])
            .output()?;

        if !command.status.success() {
            print_warn!("Failed to stop and disable service: {}", service);
        }
    }

    Ok(())
}

fn remove_files() -> anyhow::Result<(), Error> {
    let files = vec![
        "/usr/lib/systemd/system/casaos.service",
        "lib/systemd/system/casaos.service",
        "/etc/systemd/system/casaos.service",
        "/etc/casaOS",
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
        "etc/casaos",
        "/var/run/casaos",
        "/usr/bin/casaos-uninstall",
    ];
    for file in files {
        std::process::Command::new("sudo")
            .arg("rm")
            .arg("-rf")
            .arg(file)
            .output()?;
    }

    let manifest = std::fs::read_to_string("/etc/casaos/manifest")?;
    for line in manifest.lines() {
        std::process::Command::new("sudo")
            .arg("rm")
            .arg("-rf")
            .arg(line)
            .output()?;
    }
    std::fs::remove_file("/var/lib/casaos/manifest")?;

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