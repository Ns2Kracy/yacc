use clap::Parser;
use std::{collections::HashMap, fmt::Display};
use thiserror::Error;

use crate::print_error;

#[derive(Parser, Debug, Default)]
pub struct Status {
    #[clap(short, long = "all")]
    pub all: bool,

    #[clap(short, long = "name")]
    pub name: Option<String>,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status")
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get service status")]
    GetServiceStatusFailed,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ActiveState {
    Active,
    Inactive,
    Failed,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum SubState {
    Running,
    Dead,
}

impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveState::Active => console::style("Active").green().bold().fmt(f),
            ActiveState::Inactive => console::style("Inactive").cyan().bold().fmt(f),
            ActiveState::Failed => console::style("Failed").red().bold().fmt(f),
        }
    }
}

impl Display for SubState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubState::Running => console::style("running").green().bold().fmt(f),
            SubState::Dead => console::style("dead").red().bold().fmt(f),
        }
    }
}

impl crate::cli::Command for Status {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        if let Some(name) = &self.name {
            let (active_state, sub_state) = Status::get_service_status(name).unwrap();
            print!(
                "{}: {}({})",
                console::style(name).cyan().bold(),
                active_state,
                sub_state
            );
        } else {
            Status::query_service_status().unwrap();
        }

        Ok(())
    }
}

impl Status {
    /// 查询systemd服务状态
    fn query_service_status() -> anyhow::Result<(), Error> {
        let services: HashMap<&str, &str> = [
            ("CasaOS", "casaos"),
            ("CasaOS-Gateway", "casaos-gateway"),
            ("CasaOS-Message-Bus", "casaos-message-bus"),
            ("CasaOS-User-Service", "casaos-user-service"),
            ("CasaOS-Local-Storage", "casaos-local-storage"),
            ("CasaOS-App-Management", "casaos-app-management"),
            ("Rclone", "rclone"),
        ]
        .into();

        let mut status_list: HashMap<&str, (ActiveState, SubState)> = HashMap::new();

        for (service_name, service) in services.iter() {
            let (active_state, sub_state) = Status::get_service_status(service).unwrap();
            status_list.insert(service_name, (active_state, sub_state));
        }

        for (service_name, (active_state, sub_state)) in status_list.iter() {
            let service_name = format!("{: <23}", service_name);
            print!(
                "{}: {}({})\n",
                console::style(service_name).cyan().bold(),
                active_state,
                sub_state
            );
        }

        Ok(())
    }

    fn get_service_status(service: &str) -> anyhow::Result<(ActiveState, SubState), Error> {
        // 先检查服务是否存在
        let output = std::process::Command::new("systemctl")
            .arg("status")
            .arg(service)
            .output()
            .map_err(|_| Error::GetServiceStatusFailed)?;

        // 如果服务不存在，返回错误
        if output.status.code() != Some(0) {
            print_error!("Service {} does not exist", service);
        }
        let active_output = std::process::Command::new("systemctl")
            .args(&["show", "-p", "ActiveState", service])
            .output()
            .map_err(|_| Error::GetServiceStatusFailed)?;

        let sub_output = std::process::Command::new("systemctl")
            .args(&["show", "-p", "SubState", service])
            .output()
            .map_err(|_| Error::GetServiceStatusFailed)?;

        let binding =
            String::from_utf8(active_output.stdout).map_err(|_| Error::GetServiceStatusFailed)?;
        let active_state = binding.trim().split('=').collect::<Vec<&str>>()[1];

        let binding =
            String::from_utf8(sub_output.stdout).map_err(|_| Error::GetServiceStatusFailed)?;
        let sub_state = binding.trim().split('=').collect::<Vec<&str>>()[1];

        let active_state = match active_state {
            "active" => ActiveState::Active,
            "inactive" => ActiveState::Inactive,
            "failed" => ActiveState::Failed,
            _ => ActiveState::Failed,
        };

        let sub_state = match sub_state {
            "running" => SubState::Running,
            "dead" => SubState::Dead,
            _ => SubState::Dead,
        };

        Ok((active_state, sub_state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Command;

    #[test]
    fn test_command() {
        let status = Status::default();
        status.call(&crate::config::Config::default()).unwrap();
    }
}
