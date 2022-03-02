//! Audio plugin bundler CLI

use anyhow::Context;
use clap::Parser;
use plugit_lib::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "plugit")]
struct Cli {
    /// Optional absolute path of an input library to be bundled
    input_path: Option<PathBuf>,
    /// Optional bundle target override
    #[clap(short, long, parse(try_from_str))]
    target: Option<Target>,
    /// Optional output format override
    #[clap(short, long, parse(try_from_str))]
    format: Option<Format>,
    /// Optionally specify that the target compiled in debug mode
    #[clap(short, long)]
    debug: bool,
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Get the input file from args or try to get a cargo project
    let input = {
        let manifest_dir = std::env::current_dir()?;
        match &cli.input_path {
            Some(input_path) => input_path.clone(),
            // If no path is provided, search for a cargo project
            None => {
                let toml_path = manifest_dir.join("Cargo.toml");
                let toml = cargo_toml::Manifest::from_path(toml_path.clone()).context(format!(
                    "could not load Cargo.toml at {}",
                    toml_path.display()
                ))?;
                let package_name = toml
                    .package
                    .context("Cargo.toml contains no package name")?
                    .name;
                let package_lib_name = vst::Vst3::exported_lib_filename(package_name, None);
                let build_folder = if cli.debug { "debug" } else { "release" };
                manifest_dir
                    .join("target")
                    .join(build_folder)
                    .join(package_lib_name)
            }
        }
    };

    // Get the plugin format from args, or try and parse from symbols if it was
    // not provided
    let plugin_format = {
        match cli.format {
            Some(f) => f,
            None => Format::parse_from_symbols(&input)?,
        }
    };

    // Create the plugin bundle
    plugin_format.try_bundle(&input, cli.target)?;

    Ok(())
}
