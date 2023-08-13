//! # NullPointerException
//! 在Rust中平时不会出现空指针异常，当使用Option设定时存在None，这里就是指代None的情况
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use std::path::{PathBuf};
use crate::core::Exception;
use super::{ExceptionCode, ExceptionLevel, SupperException,  NULL_POINTER_MSG};

/// # NullPointerException
/// - code:
/// - msg:
/// - line:
/// - path:
/// - level:
#[derive(Debug)]
pub struct NullPointerException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
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