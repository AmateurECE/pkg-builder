///////////////////////////////////////////////////////////////////////////////
// NAME:            package_manager.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for uniquely handling different package managers.
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

use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::str;

use serde::{Serialize, Deserialize};

pub mod pacman;

#[derive(Serialize, Deserialize)]
pub enum PackageManagerName {
    Pacman,
}

pub trait PackageManager {
    fn build(&self, name: &Path) -> Result<PathBuf, PackageError>;
    fn deploy(&self, package: &Path, repository: &Path) ->
        Result<(), PackageError>;
}

///////////////////////////////////////////////////////////////////////////////
// PackageError
////

#[derive(Debug)]
pub struct PackageError {
    details: String,
}

impl Error for PackageError {}
impl From<&str> for PackageError {
    fn from(value: &str) -> Self {
        PackageError { details: value.to_string() }
    }
}

impl From<String> for PackageError {
    fn from(details: String) -> Self {
        PackageError { details }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
        write!(writer, "{}", &self.details)
    }
}

impl From<io::Error> for PackageError {
    fn from(value: io::Error) -> Self {
        PackageError { details: value.to_string() }
    }
}

impl From<str::Utf8Error> for PackageError {
    fn from(value: str::Utf8Error) -> Self {
        PackageError { details: value.to_string() }
    }
}

///////////////////////////////////////////////////////////////////////////////
