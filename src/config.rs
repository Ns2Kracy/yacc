use clap::Parser;

pub enum Arch {
    Amd64,
    Arm64,
    Armv7,
}

impl std::str::FromStr for Arch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amd64" => Ok(Arch::Amd64),
            "arm64" => Ok(Arch::Arm64),
            "arm7" => Ok(Arch::Armv7),
            _ => Err(format!(
                "Unknown architecture, CasaOS only supports amd64, arm64 and arm-7",
            )),
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::Amd64 => write!(f, "amd64"),
            Arch::Arm64 => write!(f, "arm64"),
            Arch::Armv7 => write!(f, "arm-7"),
        }
    }
}

#[derive(Debug, Parser, Default)]
pub struct Config {}
