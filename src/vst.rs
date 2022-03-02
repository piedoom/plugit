//! Structs and functions related to the creation of VST3 plugins

use crate::prelude::*;
use anyhow::{Context, Result};
use std::{fmt::Display, path::Path};

pub struct Vst3;

impl Vst3 {
    /// Gets the local path string of where the bundle's library should be
    /// placed, determined by the current system. Returns a `String`.
    ///
    /// # Parameters
    ///
    /// * `package` - the name of the package to build
    fn lib_local_path(
        package: impl AsRef<str> + Display,
        target: Option<Target>,
    ) -> Result<String> {
        let linux = format!("{package}.vst3/Contents/x86_64-linux/{package}.so");
        let linux_x86 = format!("{package}.vst3/Contents/i386-linux/{package}.so");
        let mac = format!("{package}.vst3/Contents/MacOS/{package}");
        let windows = format!("{package}.vst3/Contents/x86_64-win/{package}.vst3");
        let windows_x86 = format!("{package}.vst3/Contents/x86-win/{package}.vst3");

        match target {
            Some(target) => match target {
                Target::Windows(arch) => match arch {
                    Arch::X64 => Ok(windows),
                    Arch::X86_64 => Ok(windows_x86),
                },
                Target::Mac => Ok(mac),
                Target::Linux(arch) => match arch {
                    Arch::X64 => Ok(linux),
                    Arch::X86_64 => Ok(linux_x86),
                },
            },
            None => {
                #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                return Ok(linux);
                #[cfg(all(target_os = "linux", target_arch = "x86"))]
                return Ok(linux_x86);
                #[cfg(target_os = "macos")]
                return Ok(mac);
                #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
                return Ok(windows);
                #[cfg(all(target_os = "windows", target_arch = "x86"))]
                return Ok(windows_x86);
            }
        }
    }

    /// Gets the platform-dependent filename of the compiled library
    ///
    /// # Parameters
    ///
    /// * `package` - the name of the compiled package
    /// * `target` - if not provided, will default to the current system's target
    pub fn exported_lib_filename(
        package: impl AsRef<str> + Display,
        target: Option<Target>,
    ) -> String {
        let linux = format!("lib{package}.so");
        let mac = format!("lib{package}.dylib");
        let windows = format!("{package}.dll");

        // Check to see if a target was provided
        match target {
            Some(target) => match target {
                // We don't care about arch here
                Target::Windows(_) => windows,
                Target::Mac => mac,
                Target::Linux(_) => linux,
            },
            // No target provided, use current system target
            None => {
                #[cfg(target_os = "linux")]
                return linux;
                #[cfg(target_os = "macos")]
                return mac;
                #[cfg(target_os = "windows")]
                return windows;
            }
        }
    }

    /// Try to create a VST3 bundle from an input file
    ///
    /// # Parameters
    ///
    /// * `path` - Absolute path to the input library that will be copied to our
    ///   VST bundle
    /// * `target` - Optionally provide what target to bundle. Defaults to current system.
    pub fn try_bundle(path: impl AsRef<Path>, target: Option<Target>) -> Result<()> {
        let parent_directory = path
            .as_ref()
            .parent()
            .context("could not get parent directory")?;
        let package_name = path
            .as_ref()
            .file_stem()
            .context("could not get file stem")?
            .to_str()
            .context("invalid path")?;

        let bundle_name = &format!("{}.vst3", package_name);
        let bundle_path = parent_directory.join(bundle_name);
        let bundle_lib_path = parent_directory.join(Self::lib_local_path(package_name, target)?);

        // Actually write the bundle directories
        std::fs::create_dir_all(bundle_lib_path.parent().unwrap())
            .context("Could not create bundle directory")?;
        reflink::reflink_or_copy(&path, &bundle_lib_path)
            .context("Could not copy library to bundle")?;

        #[cfg(target_os = "macos")]
        {
            std::fs::write(
                format!("{}/Contents/PkgInfo", bundle_path.display(),),
                "BNDL????",
            )
            .context("Could not create PkgInfo file")?;
            std::fs::write(
        format!("{}/Contents/Info.plist",  bundle_path.display()),
        format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist>
  <dict>
    <key>CFBundleExecutable</key>
    <string>{package_name}</string>
    <key>CFBundleIconFile</key>
    <string></string>
    <key>CFBundleIdentifier</key>
    <string>com.nih-plug.{package_name}</string>
    <key>CFBundleName</key>
    <string>{package_name}</string>
    <key>CFBundleDisplayName</key>
    <string>{package_name}</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>NSHumanReadableCopyright</key>
    <string></string>
    <key>NSHighResolutionCapable</key>
    <true/>
  </dict>
</plist>
"#)).context("Could not create Info.plist file")?;
        }

        println!("Created a VST3 bundle at '{}'", bundle_path.display());
        Ok(())
    }
}
