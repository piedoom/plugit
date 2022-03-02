//! Definitions of a desired architecture
//! Options are:
//! - `windows`
//! - `windows_x86`
//! - `linux`
//! - `linux_x86`
//! - `mac`

use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone)]
pub enum Target {
    Windows(Arch),
    Mac,
    Linux(Arch),
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Windows(arch) => write!(f, "windows-{arch}"),
            Target::Mac => write!(f, "mac"),
            Target::Linux(arch) => write!(f, "linux-{arch}"),
        }
    }
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "windows" | "windows-x64" => Ok(Self::Windows(Arch::X64)),
            "windows-x86" | "windows-x86_64" => Ok(Self::Windows(Arch::X86_64)),
            "linux" | "linux-x64" => Ok(Self::Linux(Arch::X64)),
            "linux-x86" | "linux-x86_64" => Ok(Self::Linux(Arch::X86_64)),
            "mac" | "macos" | "apple" => Ok(Self::Mac),
            _ => Err(anyhow::anyhow!("no match found for {}", s)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Arch {
    X64,
    X86_64,
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::X64 => write!(f, "x86_64"),
            Arch::X86_64 => write!(f, "x86"),
        }
    }
}
