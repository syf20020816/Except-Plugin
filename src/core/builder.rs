//! # builders for exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

use super::flex_impl::{BuilderImpl};
use super::{SupperException, ExceptionLevel, SUPPER_MSG, ExceptionCode};

/// # Supper Builder for Supper Exception
/// use super_pattern
#[derive(Clone, Debug, PartialEq)]
pub struct SupperBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
}

impl Default for SupperBuilder {
    fn default() -> Self {
        SupperBuilder {
            code: ExceptionCode::SUPPER,
            msg: String::from(SUPPER_MSG),
            level: ExceptionLevel::Info,
        }
    }
}

impl BuilderImpl for SupperBuilder {
    type Output = SupperException;

    fn new() -> Self {
        Default::default()
    }
    fn code(&self) -> u32 {
        self.code
    }

    fn msg(&self) -> &str {
        &self.msg
    }

    fn level(&self) -> ExceptionLevel {
        self.level.clone()
    }

    fn set_code(&mut self, code: u32) -> &mut Self {
        self.code = code;
        self
    }

    fn set_msg(&mut self, msg: &str) -> &mut Self {
        self.msg = String::from(msg);
        self
    }

    fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
        self.level = level;
        self
    }

    fn build(&mut self) -> Self::Output {
        SupperException {
            code: self.code(),
            msg: String::from(self.msg()),
            level: self.level(),
        }
    }
}