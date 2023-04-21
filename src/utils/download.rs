use std::process::Command;

fn get_download_url() -> anyhow::Result<String> {
    let command = Command::new("curl")
        .arg("-s")
        .arg("ipconfig.io/country")
        .output()?;
    let region = String::from_utf8(command.stdout)?;
    if region == "CN" || region == "China" {
        return Ok("https://casaos.oss-cn-shanghai.aliyuncs.com/".to_string());
    }
    Ok("https://github.com/IceWhaleTech/".to_string())
}

fn file_version(name: &str, version: &str) -> String {
    format!("{}-{}", name, version)
}