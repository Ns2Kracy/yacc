use std::process::ExitStatus;

pub fn systemctl(args: Vec<&str>) -> anyhow::Result<ExitStatus> {
    let mut command = std::process::Command::new("systemctl")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    Ok(command.wait()?)
}

pub fn systemctl_capture(args: Vec<&str>) -> anyhow::Result<String> {
    let output = std::process::Command::new("systemctl")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn start(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["start", service])
}

pub fn stop(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["stop", service])
}

pub fn restart(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["restart", service])
}

pub fn enable(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["enable", service])
}

pub fn enable_now(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["enable", "--now", service])
}

pub fn disable(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["disable", service])
}

pub fn disable_now(service: &str) -> anyhow::Result<ExitStatus> {
    systemctl(vec!["disable", "--now", service])
}

pub fn is_enabled(service: &str) -> anyhow::Result<bool> {
    let output = systemctl(vec!["is-enabled", service])?;
    Ok(output.success())
}

pub fn is_active(service: &str) -> anyhow::Result<bool> {
    let output = systemctl(vec!["is-active", service])?;
    Ok(output.success())
}

pub fn is_failed(service: &str) -> anyhow::Result<bool> {
    let output = systemctl(vec!["is-failed", service])?;
    Ok(output.success())
}

pub fn is_exist(service: &str) -> anyhow::Result<bool> {
    let output = systemctl(vec!["status", service])?;
    Ok(output.success())
}

pub fn active_state(service: &str) -> anyhow::Result<String> {
    let output = systemctl_capture(vec!["show", "-p", "ActiveState", "--value", service])?;
    Ok(output.trim().to_string())
}

pub fn sub_state(service: &str) -> anyhow::Result<String> {
    let output = systemctl_capture(vec!["show", "-p", "SubState", "--value", service])?;
    Ok(output.trim().to_string())
}

pub fn state(service: &str) -> anyhow::Result<String> {
    let active_state = active_state(service)?;
    let sub_state = sub_state(service)?;
    let output = format!("{}({})", active_state, sub_state);
    Ok(output)
}
