// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use crate::Kernel;

/// An entry corresponds to a single kernel, and may have a supplemental
/// cmdline
#[derive(Debug)]
pub struct Entry<'a> {
    kernel: &'a Kernel,

    // Additional cmdline
    cmdline: Option<String>,
}

impl<'a> Entry<'a> {
    /// New entry for the given kernel
    pub fn new(kernel: &'a Kernel) -> Self {
        Self { kernel, cmdline: None }
    }

    /// With the following cmdline
    pub fn with_cmdline(self, cmdline: impl AsRef<str>) -> Self {
        Self {
            cmdline: Some(cmdline.as_ref().to_string()),
            ..self
        }
    }
}