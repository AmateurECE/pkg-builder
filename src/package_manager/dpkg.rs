///////////////////////////////////////////////////////////////////////////////
// NAME:            dpkg.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for handling dpkg packages.
//
// CREATED:         05/30/2022
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

use std::path::{Path, PathBuf};
use crate::package_manager::{PackageManager, PackageError};

pub struct Dpkg;

impl Dpkg {
    pub fn new() -> Self {
        Self
    }

    fn source(&self) -> Result<(), PackageError> {
        todo!()
    }
}

impl PackageManager for Dpkg {
    fn build(&self, name: &Path) -> Result<PathBuf, PackageError> {
        todo!()
    }

    fn deploy(&self, package: &Path, repository: &Path) ->
        Result<(), PackageError>
    {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
