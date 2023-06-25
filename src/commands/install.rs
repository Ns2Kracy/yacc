use crate::{
    print_error, print_info, print_output, print_warn, utils::confirm::confirm_default_no,
};
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{
    header::{CONTENT_LENGTH, RANGE},
    Client,
};
use std::{ops::Div, path::Path};
use sys_info::linux_os_release;
use tokio::{fs, io::AsyncWriteExt};

/// Install CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args {}

pub async fn run(_cmd: Args) -> anyhow::Result<(), anyhow::Error> {
    // clear screen
    console::Term::stdout().clear_screen()?;

    // Step 0 : Get Download Url Domain
    let download_domain = get_download_domain()?;
    let arch = check_arch()?;
    let distro = check_distro()?;

    // Step 1: Check ARCH
    print_output!(
        "Your hardware architecture is: {}",
        style(arch.clone()).bold()
    );

    // Step 2: Check OS
    if !cfg!(target_os = "linux") {
        print_error!("This is only for Linux.");
    }
    print_output!("Your System is: {}", style(distro.clone()).bold());

    // Step 3: Check Distribution
    print_output!(
        "Your Linux Distribution is: {}",
        style(distro.clone()).bold()
    );

    // Step 4: Check System Required: Memory, Disk
    match check_memory() {
        Ok(_) => print_output!("Memory capacity check passed."),
        Err(e) => {
            print_error!("{}", e);
        }
    }
    match check_disk() {
        Ok(_) => print_output!("Disk capacity check passed."),
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // Step 5: Install Depends
    print_output!("{} Updating dependencies...", style("[5/5]").bold().dim());

    // Step 6: Check And Install Docker

    // Step 7: Configuration Addon

    // Step 8: Download And Install CasaOS
    print_output!("{} Downloading CasaOS...", style("[6/6]").bold().dim());
    match download_casaos(&download_domain.clone(), &arch.clone()).await {
        Ok(_) => {}
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // Step 9: Check Service Status

    // Step 10: Clear Term and Show Welcome Banner

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
        return Ok("https://casaos.oss-cn-shanghai.aliyuncs.com/IceWhaleTech/".to_string());
    }
    Ok("https://github.com/IceWhaleTech/".to_string())
}

/// Check architecture, only amd64, arm64 and arm-7 are supported.
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
fn check_distro() -> anyhow::Result<String> {
    Ok(linux_os_release().unwrap().id.unwrap())
}

/// Check memory
fn check_memory() -> anyhow::Result<(), anyhow::Error> {
    let memory = (sys_info::mem_info().unwrap().total as f64).div(1024.0);

    if memory < 400.0 {
        Err(anyhow::anyhow!("Requires atleast 400MB physical memory."))
    } else {
        Ok(())
    }
}

/// Check disk
fn check_disk() -> anyhow::Result<(), anyhow::Error> {
    let disk = (sys_info::disk_info().unwrap().free as f64)
        .div(1024.0)
        .div(1024.0);

    if disk < 5.0 {
        let _ = match confirm_default_no(format!("Recommended fress disk space is greater than 5 GB, Current free disk space is {:.2}GB\nContinue installation?", disk).as_str()).unwrap() {
            true => Ok(()),
            false => Err(anyhow::anyhow!("Installation cancelled.")),
        };
    }
    Ok(())
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

async fn download_casaos(download_domain: &str, arch: &str) -> anyhow::Result<()> {
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
        let mut sizes: Vec<String> = vec![];
        for p in packages.iter() {
            let response = client.head(p).send().await?;
            if response.status().is_success() {
                let size = response
                    .headers()
                    .get(CONTENT_LENGTH)
                    .and_then(|ct_len| ct_len.to_str().ok())
                    .and_then(|ct_len| ct_len.parse().ok())
                    .unwrap_or(0);
                sizes.push(size.to_string());
            }
        }
        sizes
    };

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    for size in sizes {
        let pb = ProgressBar::new(size.parse::<u64>().unwrap());
        pb.set_style(style.clone());

        for p in packages.iter() {
            let mut request = client.get(p);
            let file = Path::new("./dist").join(p.rsplit('/').next().unwrap());
            if file.exists() {
                let size = file.metadata().unwrap().len().saturating_sub(1);
                request = request.header(RANGE, format!("bytes={}-", size));
                pb.inc(size);
            }
            let mut source = request.send().await?;
            let mut dest = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file)
                .await?;
            while let Some(chunk) = source.chunk().await? {
                dest.write_all(&chunk).await?;
                pb.inc(chunk.len() as u64);
            }
        }
    }

    Ok(())
}
