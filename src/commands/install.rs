use crate::{print_error, print_info, print_output, print_warn};
use anyhow::Error;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{ops::Div, thread, time::Instant};
use sys_info::linux_os_release;

/// Install CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args {}

pub fn run(_cmd: Args) -> anyhow::Result<(), Error> {
    // clear screen
    console::Term::stdout().clear_screen()?;

    let mut rng = rand::thread_rng();
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(spinner_style);

    let download_domain = get_download_domain()?;
    let arch = check_arch()?;
    let distro = check_distro()?;
    let memory = check_memory()?;
    let disk = check_disk()?;

    let system_info = vec![
        format!("Your architecture is: {}", arch),
        format!("Your Linux distribution is: {}", distro),
        format!("Free memory is: {}", memory),
        format!("Free disk is: {}", disk),
    ];

    // 5.update dependencies
    print_output!("{} Updating dependencies...", style("[5/5]").bold().dim());
    match update_denpendencies(distro.as_str()) {
        Ok(_) => {
            print_output!(
                "{} Dependencies update completed.",
                style("[5/5]").bold().dim()
            );
        }
        Err(e) => {
            print_error!("Dependencies update failed: {}", e);
        }
    }

    // 6.download CasaOS
    print_output!("{} Downloading CasaOS...", style("[6/6]").bold().dim());
    match download_casaos(download_domain.as_str(), arch.as_str()) {
        Ok(_) => {
            print_output!("{} CasaOS download completed.", style("[6/6]").bold().dim());
        }
        Err(e) => {
            print_error!("CasaOS download failed: {}", e);
        }
    }
    Ok(())
}

/// Get the download domain by region.
/// For China, use Aliyun OSS.
/// For other regions, use Github.
fn get_download_domain() -> anyhow::Result<String> {
    let command = std::process::Command::new("curl")
        .arg("-s")
        .arg("ipconfig.io/country")
        .output()?;
    let region = String::from_utf8(command.stdout)?;
    if region == "CN" || region == "China" {
        return Ok("https://casaos.oss-cn-shanghai.aliyuncs.com/".to_string());
    }
    Ok("https://github.com/".to_string())
}

/// Check architecture, only amd64, arm64 and arm-7 are supported.
/// For example, amd64
/// For example, arm64
/// For example, arm-7
fn check_arch() -> anyhow::Result<String> {
    let env_arch = std::env::consts::ARCH;
    let supported_archs = vec!["x86_64", "aarch64", "armv7h"];
    let arch = match env_arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "arm" => "arm-7",
        _ => env_arch,
    };

    if !supported_archs.contains(&env_arch) {
        print_error!("Unsupported architecture: {}", arch);
    }

    Ok(arch.to_string())
}

/// Check distro
/// For example, Ubuntu
/// For example, Arch
fn check_distro() -> anyhow::Result<String> {
    Ok(linux_os_release().unwrap().id.unwrap())
}

/// Check memory
/// For example, 2G
/// For example, 4G
fn check_memory() -> anyhow::Result<String> {
    let memory = (sys_info::mem_info().unwrap().total as f64).div(1024.0);

    if memory < 1024.0 {
        Ok(format!("{}MB", memory))
    } else {
        Ok(format!("{:.2}G", memory / 1024.0))
    }
}

/// Check disk
fn check_disk() -> anyhow::Result<String> {
    let disk = (sys_info::disk_info().unwrap().free as f64)
        .div(1024.0)
        .div(1024.0);

    Ok(format!("{:.2}G", disk))
}

fn update_denpendencies(distro: &str) -> anyhow::Result<()> {
    // update denpendencies
    match distro {
        "debian" | "ubuntu" | "raspbian" | "trisquel" => {
            let command = std::process::Command::new("apt").arg("update").output()?;
            if !command.status.success() {
                print_error!("Failed to update denpendencies.");
            }
        }
        "arch" => {
            let command = std::process::Command::new("pacman").arg("-Syu").output()?;
            if !command.status.success() {
                print_error!("Failed to update denpendencies.");
            }
        }
        _ => {
            print_error!("Unsupported distro: {}", distro);
        }
    }
    Ok(())
}

fn download_casaos(download_domain: &str, arch: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let packages = vec![
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-Gateway/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-gateway-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-MessageBus/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-message-bus-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-UserService/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-user-service-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-LocalStorage/releases/download/v0.4.3/linux-${TARGET_ARCH}-casaos-local-storage-v0.4.3.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-AppManagement/releases/download/v0.4.3/linux-${TARGET_ARCH}-casaos-app-management-v0.4.3.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS/releases/download/v0.4.3-1/linux-${TARGET_ARCH}-casaos-v0.4.3-1.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-CLI/releases/download/v0.4.3-alpha2/linux-${TARGET_ARCH}-casaos-cli-v0.4.3-alpha2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}IceWhaleTech/CasaOS-UI/releases/download/v0.4.3/linux-all-casaos-v0.4.3.tar.gz"
    ].into_iter()
    .map(|x| x.replace("${CASA_DOWNLOAD_DOMAIN}", download_domain))
    .map(|x| x.replace("${TARGET_ARCH}", arch))
    .collect::<Vec<String>>();

    let sizes = {
        let mut sizes = vec![];

        for package in packages.iter() {
            let command = std::process::Command::new("curl")
                .arg("-s")
                .arg("-I")
                .arg(package)
                .output()?;
            let stdout = String::from_utf8(command.stdout)?;
            let size = stdout
                .split("\r\n")
                .filter(|x| x.starts_with("Content-Length"))
                .map(|x| x.split(':').collect::<Vec<&str>>()[1].trim())
                .collect::<Vec<&str>>()[0]
                .parse::<u64>()?;
            sizes.push(size);
        }
        sizes
    };

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let pb = m.add(ProgressBar::new(256));
    pb.set_style(sty);

    Ok(())
}
