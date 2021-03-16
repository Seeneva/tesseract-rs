// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use thiserror::Error as DeriveError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(DeriveError, Debug)]
pub enum Error {
    #[error("IO error occurred: {0}")]
    IO(#[from] io::Error),
    #[error("Tesseract inner error. Reason: {0}. Comment: {1:?}")]
    Sys(SysErrorKind, Option<String>),
}

impl Error {
    /// Create new sys error with optional comment
    pub fn new_sys_error(kind: SysErrorKind, comment: Option<impl Into<String>>) -> Self {
        Self::Sys(kind, comment.map(Into::into))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SysErrorKind {
    Null,
    NotOk,
}

impl Display for SysErrorKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Null => writeln!(f, "Tesseract return null pointer"),
            Self::NotOk => writeln!(f, "Leptonica return not success result"),
        }
    }
}
