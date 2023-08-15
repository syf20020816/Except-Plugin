//! # NullPointerException
//! 在Rust中平时不会出现空指针异常，当使用Option设定时存在None，这里就是指代None的情况
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::path::{PathBuf};
use std::fmt::{Debug, Display, Formatter};
use super::{SuperBuilderImpl, ExceptionLevel, NewFrom, FromBuilder, CommonParamImpl, TargetParamImpl, Exception, Exceptions, NullPointerExceptionBuilder, NULL_POINTER_MSG, ExceptionCode, DerefException};
use crate::{display_err_impl, exception_impl, common_param_impl, target_param_impl};

/// # NullPointerException
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
/// - target: null pointer target
#[derive(Debug, Clone, PartialEq)]
pub struct NullPointerException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
    target: Option<String>,
}

impl Default for NullPointerException {
    fn default() -> Self {
        NullPointerException {
            code: ExceptionCode::NULL_POINTER,
            msg: String::from(NULL_POINTER_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
            target: None,
        }
    }
}

impl NewFrom for NullPointerException {
    type Builder = NullPointerExceptionBuilder;
    fn new() -> Self::Builder {
        NullPointerExceptionBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        NullPointerException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            path: PathBuf::new(),
            line: 0,
            target: None,
        }
    }
}

impl FromBuilder for NullPointerException {
    type Input = NullPointerExceptionBuilder;
    type Output = NullPointerException;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            line: builder.line(),
            path: builder.path(),
            target: Some(builder.target().to_string()),
        }
    }
}

display_err_impl!(NullPointerException);

exception_impl!(NullPointerException,Exceptions::NullPointer);

common_param_impl!(NullPointerException);

target_param_impl!(NullPointerException);

impl DerefException for NullPointerException {
    fn deref_mut_exception(&mut self) -> Self {
        NullPointerException {
            code: self.code(),
            msg: String::from(self.msg()),
            line: self.line(),
            path: self.path(),
            level: self.level(),
            target: Some(self.target().to_string()),
        }
    }
}

