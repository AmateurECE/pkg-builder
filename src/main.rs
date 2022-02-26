///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
//
// CREATED:         02/26/2022
//
// LAST EDITED:     02/26/2022
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
use clap::Parser;
use serde_yaml;

mod package_manager;
mod configuration;

use package_manager::{PackageManagerName, PackageManager, pacman};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Configuration file
    #[clap(short, long)]
    config: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let configuration_file = args.config.unwrap_or(
        "pkg-builder.yaml".to_string());

    let configuration: configuration::BuilderConfiguration =
        serde_yaml::from_reader(io::BufReader::new(fs::File::open(
            configuration_file)?))?;

    let packager: Box<dyn PackageManager> = match configuration.package_manager
    {
        PackageManagerName::Pacman => Box::new(pacman::Pacman::new()),
    };

    for package in fs::read_dir(".")?.into_iter() {
        let package = package?;
        if !package.file_type()?.is_dir() {
            continue
        }

        let path = package.path();
        let base_name = path.file_name().unwrap().to_str().unwrap();
        if base_name == ".git" {
            continue
        }

        println!("Packaging {}", base_name);
        if let Err(error) = packager.as_ref().build(&base_name) {
            println!("{}", error.to_string())
        }
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
