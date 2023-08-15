//! # builders for exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::ops::Deref;
use std::path::PathBuf;
use crate::{builder_impl, common_param_impl, target_param_impl, out_of_bounds_impl};
use crate::core::{CommonParamImpl, NullPointerException, TargetParamImpl};
use super::{
    SuperException, Exceptions, ExceptionLevel, SUPER_MSG, EASY_MSG, NULL_POINTER_MSG, OUT_OF_BOUNDS_MSG,
    ExceptionCode, FromBuilder, SuperBuilderImpl, EasyException,
    ArrayIndexOutOfBoundsException, OutOfBoundsParamImpl,
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
            code: ExceptionCode::SUPER,
            msg: String::from(SUPER_MSG),
            level: ExceptionLevel::Info,
            e_type: Exceptions::Super,
        }
    }
}

//----------------------------------------------------------
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

common_param_impl!(EasyExceptionBuilder);

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
            msg: String::from(NULL_POINTER_MSG),
            level: ExceptionLevel::Info,
            line: 0,
            path: PathBuf::new(),
            target: None,
            e_type: Exceptions::Easy,
        }
    }
}

builder_impl!(NullPointerExceptionBuilder,NullPointerException);

common_param_impl!(NullPointerExceptionBuilder);

target_param_impl!(NullPointerExceptionBuilder);

//----------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayIndexOutOfBoundsBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    line: u32,
    path: PathBuf,
    target: Option<String>,
    len: usize,
    index: usize,
    e_type: Exceptions,
}

impl Default for ArrayIndexOutOfBoundsBuilder {
    fn default() -> Self {
        ArrayIndexOutOfBoundsBuilder {
            code: ExceptionCode::ARRAY_INDEX_OUT_OF,
            msg: String::from(OUT_OF_BOUNDS_MSG),
            level: ExceptionLevel::Info,
            line: 0,
            path: PathBuf::new(),
            target: None,
            len: 0,
            index: 0,
            e_type: Exceptions::ArrayIndexOutOfBounds,
        }
    }
}

builder_impl!(ArrayIndexOutOfBoundsBuilder,ArrayIndexOutOfBoundsException);

common_param_impl!(ArrayIndexOutOfBoundsBuilder);

target_param_impl!(ArrayIndexOutOfBoundsBuilder);

out_of_bounds_impl!(ArrayIndexOutOfBoundsBuilder);