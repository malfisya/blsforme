// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! File utilities shared between the blsforme APIs

use std::{
    fs::{self, File},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use crate::Error;

/// Case-insensitive path joining for FAT, respecting existing entries on the filesystem
/// Note, this discards errors, so will require read permissions
pub trait PathExt<P: AsRef<Path>> {
    fn join_insensitive(&self, path: P) -> PathBuf;
}

impl<P: AsRef<Path>> PathExt<P> for PathBuf {
    fn join_insensitive(&self, path: P) -> PathBuf {
        let real_path: &Path = path.as_ref();
        if let Ok(dir) = fs::read_dir(self) {
            let entries = dir.filter_map(|e| e.ok()).filter_map(|p| {
                let n = p.file_name();
                n.into_string().ok()
            });
            for entry in entries {
                if entry.to_lowercase() == real_path.to_string_lossy().to_lowercase() {
                    return self.join(&entry);
                }
            }
        }
        self.join(path)
    }
}

/// Compare two files with blake3 to see if they differ
fn files_identical(hasher: &mut blake3::Hasher, a: &Path, b: &Path) -> Result<bool, Error> {
    let fi_a = File::open(a)?;
    let fi_b = File::open(b)?;
    let fi_a_m = fi_a.metadata()?;
    let fi_b_m = fi_b.metadata()?;
    if fi_a_m.size() != fi_b_m.size() || fi_a_m.file_type() != fi_b_m.file_type() {
        Ok(false)
    } else {
        hasher.update_mmap_rayon(a)?;
        let result_a = hasher.finalize();
        hasher.reset();

        hasher.update_mmap_rayon(b)?;
        let result_b = hasher.finalize();
        hasher.reset();

        Ok(result_a == result_b)
    }
}

/// Find out which files in the set changed
///
/// Given a slice containing tuples of pathbufs, return an
/// allocated set of cloned pathbuf tuples (pairs) known to
/// differ.
///
/// The first element in the tuple should be the source path, and the
/// right hand side should contain the destination path.
pub fn changed_files<'a, 'b: 'a>(files: &'a [(PathBuf, PathBuf)]) -> Vec<(&'a PathBuf, &'a PathBuf)> {
    let mut hasher = blake3::Hasher::new();

    files
        .iter()
        .filter_map(|(source, dest)| match files_identical(&mut hasher, source, dest) {
            Ok(same) => {
                if same {
                    None
                } else {
                    Some((source, dest))
                }
            }
            Err(_) => Some((source, dest)),
        })
        .collect::<Vec<_>>()
}
