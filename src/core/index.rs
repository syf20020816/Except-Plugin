//! # Index Out of bounds Exception
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/14
//! @version:0.0.1
//! @description:
//! ```


use std::error::Error;
use std::path::{PathBuf};
use std::fmt::{Debug, Display, Formatter};
use super::{OUT_OF_BOUNDS_MSG, SuperBuilderImpl, ExceptionLevel, NewFrom, FromBuilder, CommonParamImpl, OutOfBoundsParamImpl, TargetParamImpl, Exception, Exceptions, ArrayIndexOutOfBoundsBuilder, ExceptionCode, DerefException};
use crate::{display_err_impl, exception_impl, common_param_impl, target_param_impl, out_of_bounds_impl};

/// # ArrayIndexOutofBoundsException
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
/// - target: null pointer target
/// - len: array length
/// - index: which index cause out of bounds
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndexOutOfBoundsException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
    target: Option<String>,
    len: usize,
    index: usize,
}

impl Default for ArrayIndexOutOfBoundsException {
    fn default() -> Self {
        ArrayIndexOutOfBoundsException {
            code: ExceptionCode::NULL_POINTER,
            msg: String::from(OUT_OF_BOUNDS_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
            target: None,
            len: 0,
            index: 0,
        }
    }
}

impl NewFrom for ArrayIndexOutOfBoundsException {
    type Builder = ArrayIndexOutOfBoundsBuilder;
    fn new() -> Self::Builder {
        ArrayIndexOutOfBoundsBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        ArrayIndexOutOfBoundsException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            path: PathBuf::new(),
            line: 0,
            target: None,
            len: 0,
            index: 0,
        }
    }
}

impl FromBuilder for ArrayIndexOutOfBoundsException {
    type Input = ArrayIndexOutOfBoundsBuilder;
    type Output = ArrayIndexOutOfBoundsException;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            line: builder.line(),
            path: builder.path(),
            target: Some(builder.target().to_string()),
            len: builder.len(),
            index: builder.index(),
        }
    }
}

display_err_impl!(ArrayIndexOutOfBoundsException);

exception_impl!(ArrayIndexOutOfBoundsException,Exceptions::ArrayIndexOutOfBounds);

common_param_impl!(ArrayIndexOutOfBoundsException);

target_param_impl!(ArrayIndexOutOfBoundsException);

out_of_bounds_impl!(ArrayIndexOutOfBoundsException);

impl DerefException for ArrayIndexOutOfBoundsException {
    fn deref_mut_exception(&mut self) -> Self {
        ArrayIndexOutOfBoundsException {
            code: self.code(),
            msg: String::from(self.msg()),
            line: self.line(),
            path: self.path(),
            level: self.level(),
            target: Some(self.target().to_string()),
            len: self.len(),
            index: self.index(),
        }
    }
}

