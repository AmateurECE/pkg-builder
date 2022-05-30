///////////////////////////////////////////////////////////////////////////////
// NAME:            pacman.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for building Pacman packages.
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

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use crate::package_manager::{PackageManager, PackageError};

pub struct Pacman;

impl Pacman {
    pub fn new() -> Self {
        Pacman {}
    }

    fn get_file(&self, dir: &Path, extension: &str) ->
        Result<Option<PathBuf>, PackageError>
    {
        let native_extension = OsStr::new(extension);
        Ok(fs::read_dir(dir)?
           .collect::<Result<Vec<fs::DirEntry>, _>>()?.into_iter()
           .map(|entry| entry.path())
           .find(|entry| entry.extension() == Some(native_extension)))
    }
}

impl PackageManager for Pacman {
    fn build(&self, name: &Path) -> Result<PathBuf, PackageError>
    {
        println!("makepkg");
        let makepkg = Command::new("makepkg")
            .args(["-sc"])
            .current_dir(name)
            .output()?;
        if !makepkg.status.success() {
            return Err(PackageError::from(str::from_utf8(&makepkg.stderr)?))
        }

        self.get_file(name, "zst")?.ok_or(PackageError::from(
            "makepkg succeeded, but no package file was found"))
    }

    fn deploy(&self, package: &Path, repository: &Path) ->
        Result<(), PackageError>
    {
        let repository_path = repository.parent().ok_or(
            PackageError::from(repository.to_string_lossy().to_string()
                               + " is not a valid repository path"))?;

        // Copy package file to repository
        let deployed_package_path = repository_path.to_owned().join(package);
        fs::copy(package, &deployed_package_path)?;

        // Add package to repository file
        let repo_add = Command::new("repo-add")
            .args([&repository, deployed_package_path.as_path()])
            .output()?;
        if !repo_add.status.success() {
            return Err(PackageError::from(str::from_utf8(&repo_add.stderr)?))
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////
