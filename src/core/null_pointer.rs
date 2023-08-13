//! # NullPointerException
//! 在Rust中平时不会出现空指针异常，当使用Option设定时存在None，这里就是指代None的情况
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{PathBuf};
use crate::core::Exceptions;
use super::{ExceptionCode, ExceptionLevel, SupperException, NULL_POINTER_MSG, Exception, NewFrom, ExceptionBuilder, FromBuilder};

/// # NullPointerException
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
#[derive(Debug, Clone, PartialEq)]
pub struct NullPointerException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
}

impl NewFrom for NullPointerException {
    type Builder = ExceptionBuilder;
    fn new() -> Self::Builder {
        ExceptionBuilder::new(Exceptions::NullPointer)
    }
    fn from(e: Box<dyn Exception>) -> Self where Self: Sized {
        NullPointerException {
            code: e.code(),
            msg: String::from(e.msg()),
            line: e.line(),
            path: e.path(),
            level: e.level(),
        }
    }
}

impl FromBuilder for NullPointerException {
    type Input =  ExceptionBuilder;
    type Output = NullPointerException;
    fn from_builder(builder: &mut Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            line: builder.line(),
            path: builder.path(),
            level: builder.level(),
        }
    }
}

impl Error for NullPointerException {
    fn description(&self) -> &str {
        self.msg()
    }
}

impl Display for NullPointerException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Exception for NullPointerException {
    fn code(&self) -> u32 {
        self.code
    }

    fn msg(&self) -> &str {
        &self.msg
    }

    fn level(&self) -> ExceptionLevel {
        self.level.clone()
    }

    fn set_code(&mut self, code: u32) -> () {
        self.code = code;
    }
    fn set_level(&mut self, level: ExceptionLevel) -> () {
        self.level = level;
    }
    fn set_msg(&mut self, msg: &str) -> () {
        self.msg = String::from(msg);
    }

    fn get_type(&self) -> Exceptions {
        Exceptions::NullPointer
    }
    fn line(&self) -> u32 {
        self.line
    }
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
    fn set_line(&mut self, line: u32) -> () {
        self.line = line;
    }

    fn set_path(&mut self, path: PathBuf) -> () {
        self.path = path;
    }
}


impl Default for NullPointerException {
    fn default() -> Self {
        NullPointerException {
            code: ExceptionCode::NULL_POINTER,
            msg: String::from(NULL_POINTER_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
        }
    }
}

// e_new_from_impl! {NullPointerException,SupperBuilder}