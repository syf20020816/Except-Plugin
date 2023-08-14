//! # builders for exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::ops::Deref;
use std::path::PathBuf;
use crate::core::Exceptions;
use crate::core::null_pointer::NullPointerException;
use crate::{Exception, builder_impl};
use super::{SuperException, ExceptionLevel, SUPPER_MSG, ExceptionCode, FromBuilder, SuperBuilderImpl};

/// # 异常工厂
///
pub struct ExceptionFactory;

impl ExceptionFactory {
    pub fn new<E, B>() -> B where B: SuperBuilderImpl<E> {
        B::new()
    }
}

/// # Supper Builder for Supper Exception
/// use super_pattern
#[derive(Clone, Debug, PartialEq)]
pub struct SuperBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    e_type: Exceptions,
}

builder_impl!(SuperBuilder,SuperException);


// impl SuperBuilderImpl for SuperBuilder {
//     type Output = SuperException;
//
//     fn new() -> Self {
//         Default::default()
//     }
//
//     fn code(&self) -> u32 {
//         self.code
//     }
//
//     fn msg(&self) -> &str {
//         &self.msg
//     }
//
//     fn level(&self) -> ExceptionLevel {
//         self.level.clone()
//     }
//
//     fn set_code(&mut self, code: u32) -> &mut Self {
//         self.code = code;
//         self
//     }
//
//     fn set_msg(&mut self, msg: &str) -> &mut Self {
//         self.msg = String::from(msg);
//         self
//     }
//
//     fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
//         self.level = level;
//         self
//     }
//     fn exception_type(&self) -> Exceptions {
//         self.e_type.clone()
//     }
//     fn build(&mut self) -> Self::Output {
//         Self::Output::from_builder(self.deref())
//     }
// }

impl Default for SuperBuilder {
    fn default() -> Self {
        SuperBuilder {
            code: ExceptionCode::SUPPER,
            msg: String::from(SUPPER_MSG),
            level: ExceptionLevel::Info,
            e_type: Exceptions::Super,
        }
    }
}


//
// #[derive(Clone, Debug, PartialEq)]
// pub struct ExceptionBuilder {
//     code: u32,
//     msg: String,
//     level: ExceptionLevel,
//     line: u32,
//     path: PathBuf,
//     exception_type: Exceptions,
// }
//
// impl BuilderImpl for ExceptionBuilder {
//     fn new(exception_type: Exceptions) -> Self {
//         ExceptionBuilder {
//             code: ExceptionCode::SUPPER,
//             msg: String::from(SUPPER_MSG),
//             level: ExceptionLevel::Info,
//             line: 0,
//             path: PathBuf::new(),
//             exception_type,
//         }
//     }
//     fn code(&self) -> u32 {
//         self.code
//     }
//
//     fn msg(&self) -> &str {
//         &self.msg
//     }
//
//     fn level(&self) -> ExceptionLevel {
//         self.level.clone()
//     }
//
//     fn set_code(&mut self, code: u32) -> &mut Self {
//         self.code = code;
//         self
//     }
//
//     fn set_msg(&mut self, msg: &str) -> &mut Self {
//         self.msg = String::from(msg);
//         self
//     }
//
//     fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
//         self.level = level;
//         self
//     }
//     fn line(&self) -> u32 {
//         self.line
//     }
//     fn path(&self) -> PathBuf {
//         self.path.clone()
//     }
//     fn set_line(&mut self, line: u32) -> &mut Self {
//         self.line = line;
//         self
//     }
//     fn set_path(&mut self, path: PathBuf) -> &mut Self {
//         self.path = path;
//         self
//     }
//     fn exception_type(&self) -> Exceptions {
//         self.exception_type.clone()
//     }
//     fn build(&mut self) -> Box<dyn Exception> {
//         match self.exception_type {
//             Exceptions::Super => panic!("please use SuperBuilder"),
//             Exceptions::NullPointer => {
//                 Box::new(
//                     NullPointerException::from_builder(self)
//                 )
//             }
//             _ => panic!("no")
//         }
//     }
// }

