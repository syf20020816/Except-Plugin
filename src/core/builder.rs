//! # builders for exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::path::PathBuf;
use crate::core::Exceptions;
use crate::core::null_pointer::NullPointerException;
use crate::Exception;
use super::{SupperException, ExceptionLevel, SUPPER_MSG, ExceptionCode, FromBuilder};

/// # Supper Builder for Supper Exception
/// use super_pattern
#[derive(Clone, Debug, PartialEq)]
pub struct SupperBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
}

impl Default for SupperBuilder {
    fn default() -> Self {
        SupperBuilder {
            code: ExceptionCode::SUPPER,
            msg: String::from(SUPPER_MSG),
            level: ExceptionLevel::Info,
        }
    }
}

impl SupperBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn level(&self) -> ExceptionLevel {
        self.level.clone()
    }

    pub fn set_code(&mut self, code: u32) -> &mut Self {
        self.code = code;
        self
    }

    pub fn set_msg(&mut self, msg: &str) -> &mut Self {
        self.msg = String::from(msg);
        self
    }

    pub fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
        self.level = level;
        self
    }

    pub fn build(&mut self) -> SupperException {
        SupperException {
            code: self.code(),
            msg: String::from(self.msg()),
            level: self.level(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExceptionBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    line: u32,
    path: PathBuf,
    exception_type: Exceptions,
}

impl ExceptionBuilder {
    pub fn new(exception_type: Exceptions) -> Self {
        ExceptionBuilder {
            code: ExceptionCode::SUPPER,
            msg: String::from(SUPPER_MSG),
            level: ExceptionLevel::Info,
            line: 0,
            path: PathBuf::new(),
            exception_type,
        }
    }
    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn level(&self) -> ExceptionLevel {
        self.level.clone()
    }

    pub fn set_code(&mut self, code: u32) -> &mut Self {
        self.code = code;
        self
    }

    pub fn set_msg(&mut self, msg: &str) -> &mut Self {
        self.msg = String::from(msg);
        self
    }

    pub fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
        self.level = level;
        self
    }
    pub fn line(&self) -> u32 {
        self.line
    }
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }
    pub fn set_path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path;
        self
    }
    pub fn exception_type(&self) -> Exceptions {
        self.exception_type.clone()
    }
    pub fn build(&mut self) -> Box<dyn Exception> {
        match self.exception_type {
            Exceptions::Super => panic!("please use SuperBuilder"),
            Exceptions::NullPointer => {
                Box::new(NullPointerException::from_builder(self))
            }
            _ => panic!("no")
        }
    }
}

