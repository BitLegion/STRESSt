use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Lang {
    Python, Java, Cpp
}

impl Lang {
    fn valid_ext(&self) -> Vec<&str> {
        match self {
            Lang::Python => vec!["py"],
            Lang::Java => vec!["java"],
            Lang::Cpp => vec!["cpp", "cc", "cxx", "c++"]
        }
    }
}

/// https://stackoverflow.com/questions/42726095
pub fn file_ext(file: &str) -> Option<&str> {
    Path::new(file)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn path_ext(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}

pub fn path_str(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn file_lang(file: &PathBuf) -> Option<Lang> {
    let ext = path_ext(file);
    if ext.is_none() {
        return None;
    }
    let ext = ext.unwrap();
    for l in Lang::iter() {
        if l.valid_ext().contains(&ext) {
            return Some(l);
        }
    }
    None
}
