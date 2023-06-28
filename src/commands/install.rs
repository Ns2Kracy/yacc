use crate::{
    consts::CASA_SERVICES,
    print_error, print_info, print_ok, print_output, print_warn,
    utils::{
        confirm::confirm_default_no,
        systemd::{self},
    },
};
use console::style;
use flate2::read::GzDecoder;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{
    header::{CONTENT_LENGTH, RANGE},
    Client,
};
use std::{
    fs::File, io::Write, ops::Div, os::unix::prelude::PermissionsExt, path::Path, process::Command,
};
use sys_info::linux_os_release;
use tar::Archive;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use walkdir::WalkDir;

/// Install CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args {}

pub async fn run(_cmd: Args) -> anyhow::Result<(), anyhow::Error> {
    // clear screen
    console::Term::stdout().clear_screen()?;
    print_output!(
        r#"
   _____                 ____   _____
  / ____|               / __ \ / ____|
 | |     __ _ ___  __ _| |  | | (___
 | |    / _` / __|/ _` | |  | |\___ \
 | |___| (_| \__ \ (_| | |__| |____) |
  \_____\__,_|___/\__,_|\____/|_____/

   --- Made by IceWhale with YOU ---
    "#
    );

    // Step 0 : Get Download Url Domain
    let download_domain = get_download_domain().unwrap();
    let arch = check_arch().unwrap();
    let distro = check_distro().unwrap();

    // Step 1: Check ARCH
    print_info!(
        "Your hardware architecture is: {}",
        style(arch.clone()).bold()
    );

    // Step 2: Check OS
    if !cfg!(target_os = "linux") {
        print_error!("This is only for Linux.");
    }
    print_info!("Your System is: {}", style(distro.clone()).bold());

    // Step 3: Check Distribution
    print_info!("Your Linux Distribution is: {}", style(distro).bold());

    // Step 4: Check Memory, Disk
    match check_memory() {
        Ok(_) => print_info!("Memory capacity check passed.",),
        Err(e) => {
            print_error!("{}", e);
        }
    }
    // Step 5: Check Disk
    match check_disk() {
        Ok(_) => print_info!("Disk capacity check passed.",),
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // TODO: dependency update and install
    // Step 6: Install Depends
    print_info!("Updating dependencies...");
    update_denpendencies().unwrap();

    // TODO: docker check
    // Step 7: Check And Install Docker
    print_info!("Checking Docker...");
    check_docker().unwrap();

    // TODO: addon configuraion
    // Step 8: Configuration Addon
    print_info!("Configuring CasaOS addon...",);
    configuraion_addon().unwrap();

    // Step 9: Download And Install CasaOS
    print_info!("Downloading CasaOS...");
    match download_and_install_casaos(download_domain, arch).await {
        Ok(_) => {}
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // Step 10: Check Service Status
    check_service_status().unwrap();

    // TODO: welcome banner
    // Step 11: Clear Term and Show Welcome Banner
    welcome_banner().unwrap();

    Ok(())
}

/// Get the download domain by region.
/// For China, use Aliyun OSS.
/// For other regions, use Github.
fn get_download_domain() -> anyhow::Result<String, anyhow::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://ipapi.co/json").send().unwrap();

    let response = res.json::<serde_json::Value>().unwrap();

    if let Some(country_code) = response["country_code"].as_str() {
        if country_code == "CN" {
            return Ok("https://casaos.oss-cn-shanghai.aliyuncs.com/IceWhaleTech/".to_string());
        }
    }

    Ok("https://github.com/IceWhaleTech/".to_string())
}

/// Check architecture, only amd64, arm64 and arm-7 are supported.
fn check_arch() -> anyhow::Result<String, anyhow::Error> {
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
fn check_distro() -> anyhow::Result<String, anyhow::Error> {
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
fn update_denpendencies() -> anyhow::Result<(), anyhow::Error> {
    Ok(())
}

fn check_docker() -> anyhow::Result<(), anyhow::Error> {
    Ok(())
}

fn configuraion_addon() -> anyhow::Result<(), anyhow::Error> {
    Ok(())
}

async fn download_and_install_casaos(
    download_domain: String,
    arch: String,
) -> anyhow::Result<(), anyhow::Error> {
    let client = Client::new();
    // 在/tmp目录下创建casaos目录
    let tmp_dir = tempfile::tempdir()?.path().join("casaos");
    // create tmp dir
    std::fs::create_dir_all(&tmp_dir)?;
    let packages = vec![
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-Gateway/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-gateway-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-MessageBus/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-message-bus-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-UserService/releases/download/v0.4.2/linux-${TARGET_ARCH}-casaos-user-service-v0.4.2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-LocalStorage/releases/download/v0.4.3/linux-${TARGET_ARCH}-casaos-local-storage-v0.4.3.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-AppManagement/releases/download/v0.4.3/linux-${TARGET_ARCH}-casaos-app-management-v0.4.3.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS/releases/download/v0.4.3-1/linux-${TARGET_ARCH}-casaos-v0.4.3-1.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-CLI/releases/download/v0.4.3-alpha2/linux-${TARGET_ARCH}-casaos-cli-v0.4.3-alpha2.tar.gz",
        "${CASA_DOWNLOAD_DOMAIN}CasaOS-UI/releases/download/v0.4.3/linux-all-casaos-v0.4.3.tar.gz"
    ].into_iter()
    .map(|x| x.replace("${CASA_DOWNLOAD_DOMAIN}", &download_domain))
    .map(|x| x.replace("${TARGET_ARCH}", &arch))
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

    let spinner_style = ProgressStyle::with_template(
        "{prefix:.bold} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg}",
    )
    .unwrap()
    .progress_chars("#>-");

    let m = MultiProgress::new();

    let handles: Vec<_> = (0..packages.len())
        .map(|i| {
            let package = packages[i].clone();
            let size = sizes[i].clone();
            let name = package.rsplit('/').next().unwrap().to_string();
            let pb = m.add(ProgressBar::new(size.parse::<u64>().unwrap()));
            pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("Downloading {}\n", name));
            let mut request = client.get(package);
            let file = tmp_dir.as_path().join(name);
            // Download file
            tokio::spawn(async move {
                if file.exists() {
                    let size = file.metadata().unwrap().len().saturating_sub(1);
                    request = request.header(RANGE, format!("bytes={}-", size));
                    pb.inc(size);
                }
                let mut source = request.send().await.unwrap();
                let mut dest = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file)
                    .await
                    .unwrap();
                while let Some(chunk) = source.chunk().await.unwrap() {
                    dest.write_all(&chunk).await.unwrap();
                    pb.inc(chunk.len() as u64);
                }
                pb.finish_with_message("Downloaded");
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.await;
    }

    for p in packages.iter() {
        print_info!("Extracting {}...", style(format!("{}", p)).bold());
        let name = p.rsplit('/').next().unwrap().to_string();
        let file = tmp_dir.as_path().join(name.clone());
        let mut archive = Archive::new(GzDecoder::new(File::open(&file).unwrap()));
        if archive.unpack(tmp_dir.as_path()).is_err() {
            print_error!("Failed to extract {}", name);
        } else {
            print_ok!("{} Extracted", name);
        }
    }
    let services = CASA_SERVICES.clone();

    let build_dir = tmp_dir.as_path().join("build");
    if !build_dir.exists() {
        print_error!("Failed to find build directory");
    }

    // stop services
    // for service in services {
    //     print_info!("Stopping {}...", style(format!("{}", service)).bold());
    //     if let Ok(true) = systemd::exists(service) {
    //         if let Ok(true) = systemd::disable(service) {
    //             print_ok!("{} Stopped", service);
    //         } else {
    //             print_warn!("Failed to stop {}", service);
    //         }
    //     } else {
    //         print_warn!("Service {} does not exist.", service);
    //     }
    // }

    // check if migration script directory exists
    let migration_script_dir = build_dir.join("scripts/migration/script.d");
    if !migration_script_dir.exists() {
        print_error!("Failed to find migration script directory");
    }

    // execute migration scripts
    for entry in WalkDir::new(migration_script_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "sh" {
            print_info!(
                "Running migration script {}...",
                style(format!("{}", path.display())).bold()
            );
            let status = Command::new("bash").arg(path).status().unwrap();
            if !status.success() {
                print_error!("Failed to run migration script");
            }
        }
    }

    print_info!("Installing CasaOS...");
    let sysroot_dir = build_dir.join("sysroot");
    if !sysroot_dir.exists() {
        print_error!("Failed to find sysroot directory");
    }

    // Generate manifest for uninstallation
    let manifest_file = build_dir.join("sysroot/var/lib/casaos/manifest");
    let mut manifest = File::create(manifest_file)?;
    for entry in WalkDir::new(sysroot_dir.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            // write file path to manifest
            manifest.write_all(format!("{}\n", path.to_str().unwrap()).as_bytes())?;
        }
    }

    let options = fs_extra::dir::CopyOptions::new().overwrite(true);
    for entry in WalkDir::new(sysroot_dir.clone())
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let dest = Path::new("/").join(path.strip_prefix(sysroot_dir.clone()).unwrap());
        if path.is_dir() {
            if let Err(e) = fs_extra::dir::copy(path, "/", &options) {
                print_error!("Failed to copy directory: {}", e);
            } else {
                print_ok!("Copied {} to {}", path.display(), dest.display());
            }
        }
    }

    // check if setup script directory exists
    let setup_script_dir = build_dir.join("scripts/setup/script.d");
    if !setup_script_dir.exists() {
        print_error!("Failed to find setup script directory");
    }
    // execute migration scripts
    print_output!("Running setup scripts...");
    for entry in WalkDir::new(setup_script_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "sh" {
            let status = Command::new("bash").arg(path).status().unwrap();
            if !status.success() {
                print_error!("Failed to run setup script");
            }
        }
    }

    let ui_events_reg_script = Path::new("/etc/casaos/start.d/register-ui-events.sh");

    // chmod +x /etc/casaos/start.d/register-ui-events.sh
    let mut perms = std::fs::metadata(ui_events_reg_script)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(ui_events_reg_script, perms)?;

    // TODO: Make app store configurable

    // Start and enable casaos services
    for service in services {
        print_info!("Starting {}...", style(format!("{}", service)).bold());
        if let Ok(true) = systemd::exists(service) {
            if let Ok(true) = systemd::enable(service) {
                print_ok!("{}", style(format!("{} is enabled", service)));
            } else {
                print_error!("{} is not running, Please reinstall", service);
            }
        } else {
            print_error!("{} could not be found, Please reinstall", service);
        }
    }
    drop(tmp_dir);

    Ok(())
}

fn check_service_status() -> anyhow::Result<(), anyhow::Error> {
    let services = CASA_SERVICES.clone();

    for service in services {
        print_info!("Checking {}...", style(format!("{}", service)).bold());
        if let Ok(true) = systemd::exists(service) {
            if let Ok(true) = systemd::is_active(service) {
                print_ok!("{}", style(format!("{} is running", service)));
            } else {
                print_error!("{} is not running, Please reinstall", service);
            }
        } else {
            print_error!("Service {} does not exist.", service);
        }
    }
    Ok(())
}

fn welcome_banner() -> anyhow::Result<(), anyhow::Error> {
    print_output!(
        "{}",
        style("─────────────────────────────────────────────────────")
            .green()
            .bold()
    );
    print_output!(
        "{}",
        style("─────────────────────────────────────────────────────")
            .green()
            .bold()
    );
    print_output!("");
    print_output!(
        "{}",
        style("CasaOS Project  : https://github.com/IceWhaleTech/CasaOS".to_string()).dim()
    );
    print_output!(
        "{}",
        style("CasaOS Team     : https://github.com/IceWhaleTech/CasaOS#maintainers").dim()
    );
    print_output!(
        "{}",
        style("CasaOS Discord  : https://discord.gg/knqAbbBbeX").dim()
    );
    print_output!("{}", style("Website         : https://www.casaos.io").dim());
    print_output!("{}", style("Online Demo     : http://demo.casaos.io").dim());
    print_output!("");
    print_output!(
        "{}       {}",
        style("Uninstall").bold(),
        style(": casaos-uninstall"),
    );
    Ok(())
}

#[test]
fn test_get_download_domain() {
    let domain = get_download_domain();

    print!("{}", domain.unwrap());
}
