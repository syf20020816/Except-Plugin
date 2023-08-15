//! # Easy Exception
//! Easy Exception is the simplest exception, which has no special properties and is also the most universal
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/14
//! @version:0.0.1
//! @description:
//! ```

use std::path::PathBuf;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use super::{NewFrom, FromBuilder, EasyExceptionBuilder, CommonParamImpl, EASY_MSG, ExceptionCode, SuperBuilderImpl};
use crate::{DerefException, display_err_impl, Exception, exception_impl, common_param_impl, ExceptionLevel, Exceptions};

/// # Easy Exception
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
#[derive(Debug, Clone, PartialEq)]
pub struct EasyException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
}

impl Default for EasyException {
    fn default() -> Self {
        EasyException {
            code: ExceptionCode::COMMON,
            msg: String::from(EASY_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
        }
    }
}

impl NewFrom for EasyException {
    type Builder = EasyExceptionBuilder;
    fn new() -> Self::Builder {
        EasyExceptionBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        EasyException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            path: PathBuf::new(),
            line: 0,
        }
    }
}

impl FromBuilder for EasyException {
    type Input = EasyExceptionBuilder;
    type Output = EasyException;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            line: builder.line(),
            path: builder.path(),
        }
    }
}

display_err_impl!(EasyException);

exception_impl!(EasyException,Exceptions::Easy);

common_param_impl!(EasyException);

impl DerefException for EasyException {
    fn deref_mut_exception(&mut self) -> Self {
        EasyException {
            code: self.code(),
            msg: String::from(self.msg()),
            line: self.line(),
            path: self.path(),
            level: self.level(),
        }
    }
}