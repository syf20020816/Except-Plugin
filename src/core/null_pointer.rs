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
use super::{ExceptionLevel};
use crate::{display_err_impl, exception_impl};

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

// impl NewFrom for NullPointerException {
//     type Builder = ExceptionBuilder;
//     fn new() -> Self::Builder {
//         ExceptionBuilder::new(Exceptions::NullPointer)
//     }
//     fn from(e: Box<dyn Exception>) -> Self where Self: Sized {
//         NullPointerException {
//             code: e.code(),
//             msg: String::from(e.msg()),
//             line: e.line(),
//             path: e.path(),
//             level: e.level(),
//         }
//     }
// }
//
// impl FromBuilder for NullPointerException {
//     type Input = ExceptionBuilder;
//     type Output = NullPointerException;
//     fn from_builder(builder: &mut Self::Input) -> Self::Output {
//         Self::Output {
//             code: builder.code(),
//             msg: String::from(builder.msg()),
//             line: builder.line(),
//             path: builder.path(),
//             level: builder.level(),
//         }
//     }
// }
//
// display_err_impl!(NullPointerException);
// exception_impl!(NullPointerException);
//
//
// impl Default for NullPointerException {
//     fn default() -> Self {
//         NullPointerException {
//             code: ExceptionCode::NULL_POINTER,
//             msg: String::from(NULL_POINTER_MSG),
//             line: 0,
//             path: PathBuf::new(),
//             level: ExceptionLevel::Info,
//         }
//     }
// }

// e_new_from_impl! {NullPointerException,SupperBuilder}