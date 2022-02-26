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

use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

pub mod pacman;

#[derive(Serialize, Deserialize)]
pub enum PackageManagerName {
    Pacman,
}

pub trait PackageManager {
    fn build(&self, name: &str) -> Result<PathBuf, PackageError>;
}

///////////////////////////////////////////////////////////////////////////////
// PackageError
////

#[derive(Debug)]
pub struct PackageError {
    details: String,
}

impl Error for PackageError {}
impl PackageError {
    pub fn new(details: String) -> Self {
        PackageError { details }
    }
}

impl fmt::Display for PackageError {
    fn fmt(&self, writer: &mut fmt::Formatter) -> fmt::Result {
        write!(writer, "{}", &self.details)
    }
}

///////////////////////////////////////////////////////////////////////////////
