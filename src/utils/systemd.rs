use std::{
    io::{Error, ErrorKind, Read},
    process::ExitStatus,
};

pub fn systemctl(args: Vec<&str>) -> std::io::Result<ExitStatus> {
    let mut child = std::process::Command::new("systemctl")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    child.wait()
}

/// Invokes `systemctl $args` and captures stdout stream
fn systemctl_capture(args: Vec<&str>) -> std::io::Result<String> {
    let mut child = std::process::Command::new("systemctl")
        .args(args.clone())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    let exitcode = child.wait()?;

    match exitcode.success() {
        true => {
            let mut stdout: Vec<u8> = Vec::new();
            if let Ok(size) = child.stdout.unwrap().read_to_end(&mut stdout) {
                if size > 0 {
                    if let Ok(s) = String::from_utf8(stdout) {
                        Ok(s)
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            "Invalid utf8 data in stdout",
                        ))
                    }
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
                }
            } else {
                Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
            }
        }
        false => Err(Error::new(
            ErrorKind::Other,
            format!("systemctl {:?} failed", args),
        )),
    }
}

pub fn status(unit: &str) -> std::io::Result<String> {
    systemctl_capture(vec!["status", unit])
}

pub fn is_active(unit: &str) -> std::io::Result<bool> {
    let status = systemctl_capture(vec!["is-active", unit])?;
    Ok(status.trim_end().eq("active"))
}

pub fn exists(unit: &str) -> std::io::Result<bool> {
    let status = status(unit);
    Ok(status.is_ok())
}

pub fn enable(unit: &str) -> std::io::Result<bool> {
    let enable = systemctl(vec!["enable", "--now", unit]);
    Ok(enable.is_ok())
}

pub fn disable(unit: &str) -> std::io::Result<bool> {
    let disable = systemctl(vec!["disable", "--now", unit]);
    Ok(disable.is_ok())
}

#[test]
fn test_check_exists() {
    assert!(exists("casaos-gateway.service").unwrap());
    assert!(exists("casaos-message-bus.service").unwrap());
    assert!(exists("casaos-user-service.service").unwrap());
    assert!(exists("casaos-local-storage.service").unwrap());
    assert!(exists("casaos-app-management.service").unwrap());
    assert!(exists("rclone.service").unwrap());
    assert!(exists("casaos.service").unwrap());
}
