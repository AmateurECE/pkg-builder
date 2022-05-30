///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
//
// CREATED:         02/26/2022
//
// LAST EDITED:     05/30/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
////

use std::error::Error;
use std::io;
use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;
use clap::Parser;
use serde_yaml;

mod package_manager;
mod configuration;

use package_manager::{PackageManagerName, PackageManager, pacman, dpkg};

const CONF_FILE: &'static str = "pkg-builder.yaml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Configuration file
    #[clap(short, long, default_value_t = CONF_FILE.to_string())]
    config: String,

    /// Path to repository
    repository: String,

    /// A single package
    package: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config: configuration::BuilderConfiguration =
        serde_yaml::from_reader(io::BufReader::new(fs::File::open(
            args.config)?))?;

    let packager: Box<dyn PackageManager> = match config.package_manager {
        PackageManagerName::Pacman => Box::new(pacman::Pacman::new()),
        PackageManagerName::Dpkg => Box::new(dpkg::Dpkg::new()),
    };

    let git = OsStr::new(".git");
    let repository_path = PathBuf::from(&args.repository);
    if let Some(package) = args.package {
        // If we've been instructed just to build one package
        let package = PathBuf::from(package);
        let binary = packager.as_ref().build(&package)?;
        packager.as_ref().deploy(&binary, &repository_path)
            .map_err(|e| e.into())
    } else {
        // If we've been instructed to build all the packages
        fs::read_dir(".")?.into_iter()
            // Collect ReadDir into trappable Vec, then back to iter
            .collect::<io::Result<Vec<fs::DirEntry>>>()?.into_iter()

            // Get the path of each entry
            .map(|entry| entry.path())

            // Filter only directories not named ".git"
            .filter(|entry| entry.is_dir() && entry.file_name() != Some(&git))

            // Build packages and collect file paths of binary packages
            .filter_map(|entry| packager.as_ref().build(&entry).ok())

            // Deploy the packages that were built successfully
            .map(|package| packager.as_ref()
                 .deploy(&package, &repository_path))
            .collect::<Result<(), _>>()
            .map_err(|e| e.into())
    }
}

///////////////////////////////////////////////////////////////////////////////
