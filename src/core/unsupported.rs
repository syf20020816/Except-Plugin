//!
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/15
//! @version:0.0.1
//! @description:
//! ```

use std::error::Error;
use std::path::{PathBuf};
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use super::{UNSUPPORTED_OPERATION_MSG, UnSupportedParamImpl, SuperBuilderImpl, ExceptionLevel, NewFrom, FromBuilder, CommonParamImpl, Exception, Exceptions, UnSupportedOpExceptionBuilder, ExceptionCode, DerefException};
use crate::{display_err_impl, exception_impl, common_param_impl,  unsupported_param_impl};

/// # Reason for UnSupported
#[derive(Debug, PartialEq, Clone)]
pub enum Reasons {
    /// Illegal value
    /// such as : a param need u32 , but get value < 0 or None(Option) or bigger than u32::MAX_VALUE
    Value,
    /// Illegal type
    /// such as : a param need u32 , but get bool
    Type,
    /// thread block
    Block,
    /// cannot get lock
    Lock,
    /// cannot access
    UnAccessible,
    /// No permission to access
    Auth,
    /// IO
    IO,
    /// file not found
    FileNotFound,
    /// interrupt when thread waiting or sleeping
    Interrupted,
    /// conert error
    ClassCast,
    /// other reason
    /// specific reasons need to be set on msg param
    Other,
}

/// # UnSupportedOperationException
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
/// - target: illagal target
/// - illegal_reason: reason for error
#[derive(Debug, Clone, PartialEq)]
pub struct UnSupportedOpException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
    reason: Reasons,
    timestamp: Duration,
}

impl Default for UnSupportedOpException {
    fn default() -> Self {
        UnSupportedOpException {
            code: ExceptionCode::UNSUPPORTED_OPERATION,
            msg: String::from(UNSUPPORTED_OPERATION_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
            reason: Reasons::Other,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl NewFrom for UnSupportedOpException {
    type Builder = UnSupportedOpExceptionBuilder;
    fn new() -> Self::Builder {
        UnSupportedOpExceptionBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        UnSupportedOpException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            reason: Reasons::Other,
            path: PathBuf::new(),
            line: 0,
            timestamp: e.timestamp(),
        }
    }
}

impl FromBuilder for UnSupportedOpException {
    type Input = UnSupportedOpExceptionBuilder;
    type Output = UnSupportedOpException;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            reason: builder.reason(),
            line: builder.line(),
            path: builder.path(),
            timestamp: builder.timestamp(),
        }
    }
}

display_err_impl!(UnSupportedOpException);

exception_impl!(UnSupportedOpException,Exceptions::UnSupportedOperation);

common_param_impl!(UnSupportedOpException);

unsupported_param_impl!(UnSupportedOpException);

impl DerefException for UnSupportedOpException {
    fn deref_mut_exception(&mut self) -> Self {
        UnSupportedOpException {
            code: self.code(),
            msg: String::from(self.msg()),
            line: self.line(),
            path: self.path(),
            level: self.level(),
            reason: self.reason(),
            timestamp: self.timestamp(),
        }
    }
}


