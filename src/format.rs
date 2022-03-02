//! Retrieve and write plugin format information

use crate::prelude::*;
use anyhow::{bail, Context, Result};
use std::{path::Path, str::FromStr};

const VST3_SYMBOL: &str = "GetPluginFactory";

pub enum Format {
    Vst3,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vst" | "vst3" => Ok(Self::Vst3),
            _ => Err(anyhow::anyhow!("no format known for input {}", s)),
        }
    }
}

impl Format {
    /// Attempt to determine the plugin type by inspecting its symbols
    pub fn parse_from_symbols(path: &Path) -> Result<Self> {
        if Self::contains_symbol(path, VST3_SYMBOL)? {
            Ok(Self::Vst3)
        }
        // TODO: match for other symbols to get other formats
        else {
            // No valid matching symbol
            Err(anyhow::anyhow!(
                "{} does not match any known plugin types",
                path.as_os_str().to_str().unwrap()
            ))
        }
    }

    /// Attempt to bundle the plugin with the current format
    pub fn try_bundle(&self, input: &Path, target: Option<Target>) -> Result<()> {
        match self {
            Format::Vst3 => vst::Vst3::try_bundle(input, target),
        }
    }

    /// Check whether a binary exports the specified symbol. Used to detect the
    /// plugin formats supported by a plugin library. Returns an error if the
    /// binary cuuld not be read. This function will also parse non-native
    /// binaries.
    pub fn contains_symbol<P: AsRef<Path>>(binary: P, symbol: impl AsRef<str>) -> Result<bool> {
        // Parsing the raw binary instead of relying on nm-like tools makes
        // cross compiling a bit easier
        let bytes = std::fs::read(&binary)
            .context(format!("path {} does not exist", binary.as_ref().display()))?;
        match goblin::Object::parse(&bytes)? {
            goblin::Object::Elf(obj) => Ok(obj.dynsyms.iter().any(|sym| {
                !sym.is_import()
                    && sym.is_function()
                    && obj.dynstrtab.get_at(sym.st_name) == Some(symbol.as_ref())
            })),
            goblin::Object::Mach(obj) => {
                let obj = match obj {
                    goblin::mach::Mach::Fat(arches) => arches.get(0)?,
                    goblin::mach::Mach::Binary(obj) => obj,
                };

                // XXX: Why are all exported symbols on macOS prefixed with an
                // underscore?
                let symbol = format!("_{}", symbol.as_ref());

                Ok(obj.exports()?.into_iter().any(|sym| sym.name == symbol))
            }
            goblin::Object::PE(obj) => Ok(obj
                .exports
                .iter()
                .any(|sym| sym.name == Some(symbol.as_ref()))),
            obj => bail!("Unsupported object type: {:?}", obj),
        }
    }
}
