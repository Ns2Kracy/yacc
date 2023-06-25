use crate::{print_error, print_output, utils::confirm::confirm_default_no};
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use reqwest::{
    header::{CONTENT_LENGTH, RANGE},
    Client,
};
use std::{fmt::Write, ops::Div, path::Path, thread};
use sys_info::linux_os_release;
use tokio::{fs, io::AsyncWriteExt};

/// Install CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args {}

pub async fn run(_cmd: Args) -> anyhow::Result<(), anyhow::Error> {
    // clear screen
    console::Term::stdout().clear_screen()?;
    print_output!("Welcome to CasaOS installer.");

    // Step 0 : Get Download Url Domain
    // let download_domain = get_download_domain().unwrap();
    let arch = check_arch().unwrap();
    let distro = check_distro().unwrap();

    // Step 1: Check ARCH
    print_output!(
        "{} Your hardware architecture is: {}",
        style("[1/10]").bold().dim(),
        style(arch.clone()).bold()
    );

    // Step 2: Check OS
    if !cfg!(target_os = "linux") {
        print_error!("This is only for Linux.");
    }
    print_output!(
        "{} Your System is: {}",
        style("[2/10]").bold().dim(),
        style(distro.clone()).bold()
    );

    // Step 3: Check Distribution
    print_output!(
        "{} Your Linux Distribution is: {}",
        style("[3/10]").bold().dim(),
        style(distro).bold()
    );

    // Step 4: Check Memory, Disk
    match check_memory() {
        Ok(_) => print_output!(
            "{} Memory capacity check passed.",
            style("[4/10]").bold().dim()
        ),
        Err(e) => {
            print_error!("{}", e);
        }
    }
    // Step 5: Check Disk
    match check_disk() {
        Ok(_) => print_output!(
            "{} Disk capacity check passed.",
            style("[5/10]").bold().dim()
        ),
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // Step 6: Install Depends
    print_output!("{} Updating dependencies...", style("[6/10]").bold().dim());
    update_denpendencies().unwrap();

    // Step 7: Check And Install Docker
    print_output!("{} Checking Docker...", style("[7/10]").bold().dim());
    check_docker().unwrap();

    // Step 8: Configuration Addon
    print_output!(
        "{} Configuring CasaOS addon...",
        style("[8/10]").bold().dim()
    );
    configuraion_addon().unwrap();

    // Step 9: Download And Install CasaOS
    print_output!("{} Downloading CasaOS...", style("[9/10]").bold().dim());
    match download_casaos("https://github.com/IceWhaleTech/".to_string(), arch.clone()).await {
        Ok(_) => {}
        Err(e) => {
            print_error!("{}", e);
        }
    }

    // Step 10: Check Service Status
    print_output!(
        "{} Checking CasaOS service status...",
        style("[10/10]").bold().dim()
    );
    check_service_status().unwrap();

    // Step 11: Clear Term and Show Welcome Banner

    welcome_banner().unwrap();

    Ok(())
}

/// Get the download domain by region.
/// For China, use Aliyun OSS.
/// For other regions, use Github.
fn get_download_domain() -> anyhow::Result<String, anyhow::Error> {
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

async fn download_casaos(
    download_domain: String,
    arch: String,
) -> anyhow::Result<(), anyhow::Error> {
    let client = Client::new();
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

    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-");

    let m = MultiProgress::new();

    let handles: Vec<_> = (0..packages.len())
        .map(|i| {
            let package = packages[i].clone();
            let size = sizes[i].clone();
            let name = package.rsplit('/').next().unwrap().to_string();
            let pb = m.add(ProgressBar::new(size.parse::<u64>().unwrap()));
            pb.set_style(style.clone());
            pb.set_prefix(format!("[{}/{}]:", i + 1, packages.len(),));
            let mut request = client.get(package);

            tokio::spawn(async move {
                let file = Path::new("./dist").join(name.clone());
                if file.exists() {
                    let size = file.metadata().unwrap().len().saturating_sub(1);
                    request = request.header(RANGE, format!("bytes={}-", size));
                    pb.inc(size);
                }
                let mut source = request.send().await.unwrap();
                let mut dest = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file)
                    .await
                    .unwrap();
                while let Some(chunk) = source.chunk().await.unwrap() {
                    dest.write_all(&chunk).await.unwrap();
                    pb.set_message(format!("{} Downloading", name.clone()));
                    pb.inc(chunk.len() as u64);
                }
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.await;
    }

    m.clear().unwrap();

    Ok(())
}

fn check_service_status() -> anyhow::Result<(), anyhow::Error> {
    Ok(())
}

fn welcome_banner() -> anyhow::Result<(), anyhow::Error> {
    Ok(())
}
