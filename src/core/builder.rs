//! # builders for exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::ops::Deref;
use std::path::PathBuf;
use crate::{builder_impl};
use crate::core::{CommonParamImpl, NullPointerException, TargetParam};
use super::{
    SuperException, Exceptions, ExceptionLevel, SUPPER_MSG, EASY_MSG,
    ExceptionCode, FromBuilder, SuperBuilderImpl, EasyException,
};

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


#[derive(Clone, Debug, PartialEq)]
pub struct EasyExceptionBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    line: u32,
    path: PathBuf,
    e_type: Exceptions,
}

impl Default for EasyExceptionBuilder {
    fn default() -> Self {
        EasyExceptionBuilder {
            code: ExceptionCode::COMMON,
            msg: String::from(EASY_MSG),
            level: ExceptionLevel::Info,
            line: 0,
            path: PathBuf::new(),
            e_type: Exceptions::Easy,
        }
    }
}

builder_impl!(EasyExceptionBuilder,EasyException);

impl CommonParamImpl for EasyExceptionBuilder {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
    fn line(&self) -> u32 {
        self.line
    }
    fn set_path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path;
        self
    }
    fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }
}

//----------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct NullPointerExceptionBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    line: u32,
    path: PathBuf,
    target: Option<String>,
    e_type: Exceptions,
}

impl Default for NullPointerExceptionBuilder {
    fn default() -> Self {
        NullPointerExceptionBuilder {
            code: ExceptionCode::COMMON,
            msg: String::from(EASY_MSG),
            level: ExceptionLevel::Info,
            line: 0,
            path: PathBuf::new(),
            target: None,
            e_type: Exceptions::Easy,
        }
    }
}

builder_impl!(NullPointerExceptionBuilder,NullPointerException);

impl CommonParamImpl for NullPointerExceptionBuilder {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }
    fn line(&self) -> u32 {
        self.line
    }
    fn set_path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path;
        self
    }
    fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }
}

impl TargetParam for NullPointerExceptionBuilder {
    fn target(&self) -> &str {
        match self.target {
            Some(ref s) => s.as_str(),
            None => ""
        }
    }
    fn set_target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }
}