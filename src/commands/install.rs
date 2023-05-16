use anyhow::Error;
use std::io::Read;

/// Install CasaOS
#[derive(clap::Parser, Debug, Default)]
pub struct Args {}

pub fn run(cmd: Args) -> anyhow::Result<(), Error> {
    download_casaos_files()?;
    todo!()
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
    Ok("https://github.com/IceWhaleTech/".to_string())
}

/// Get the operating system of the system
fn get_distro() -> anyhow::Result<String> {
    let command = std::process::Command::new("uname").arg("-n").output()?;
    let distro = String::from_utf8(command.stdout)?;

    Ok(distro)
}

/// Get the architecture of the system
fn get_arch() -> anyhow::Result<String> {
    let command = std::process::Command::new("uname").arg("-m").output()?;
    let arch = String::from_utf8(command.stdout)?;

    Ok(arch)
}

/// Download the files of CasaOS and extract them to current directory
/// then remove the downloaded files
fn download_casaos_files() -> anyhow::Result<()> {
    let download_domain = get_download_domain()?;
    let distro = get_distro()?;
    let arch = get_arch()?;
    let version = String::new();

    Ok(())
}