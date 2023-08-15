//! # SQL Exception
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/15
//! @version:0.0.1
//! @description:
//! ```

use std::collections::HashMap;
use std::error::Error;
use std::path::{PathBuf};
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use super::{SQL_MSG, SQLExceptionBuilder, ReasonParamImpl, SuperBuilderImpl, SQLParamImpl, ExceptionLevel, NewFrom, FromBuilder, CommonParamImpl, Exception, Exceptions, ExceptionCode, DerefException};
use crate::{display_err_impl, exception_impl, common_param_impl, reason_param_impl, sql_param_impl, Reasons, SQLReasons};

/// # SQLException
/// - code: exception code
/// - msg: exception msg
/// - line: error line
/// - path: error file path
/// - level: exception level
/// - reason: reason for error
/// - stmt: sql statement (error)
/// - tips: how to recover
#[derive(Debug, Clone, PartialEq)]
pub struct SQLException {
    code: u32,
    msg: String,
    line: u32,
    path: PathBuf,
    level: ExceptionLevel,
    reason: Reasons,
    stmt: Option<String>,
    tips: HashMap<String, String>,
    timestamp: Duration,
}

impl Default for SQLException {
    fn default() -> Self {
        SQLException {
            code: ExceptionCode::SQL,
            msg: String::from(SQL_MSG),
            line: 0,
            path: PathBuf::new(),
            level: ExceptionLevel::Info,
            reason: Reasons::SQL(SQLReasons::Query),
            stmt: None,
            tips: HashMap::new(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl NewFrom for SQLException {
    type Builder = SQLExceptionBuilder;
    fn new() -> Self::Builder {
        SQLExceptionBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        SQLException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            reason: Reasons::SQL(SQLReasons::Other),
            stmt: None,
            path: PathBuf::new(),
            line: 0,
            timestamp: e.timestamp(),
            tips: HashMap::new()
        }
    }
}

impl FromBuilder for SQLException {
    type Input = SQLExceptionBuilder;
    type Output = SQLException;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            reason: builder.reason(),
            stmt: Some(String::from(builder.stmt())),
            line: builder.line(),
            path: builder.path(),
            timestamp: builder.timestamp(),
            tips: builder.tips().clone()
        }
    }
}

display_err_impl!(SQLException);

exception_impl!(SQLException,Exceptions::SQL);

common_param_impl!(SQLException);

reason_param_impl!(SQLException);

sql_param_impl!(SQLException);

impl DerefException for SQLException {
    fn deref_mut_exception(&mut self) -> Self {
        SQLException {
            code: self.code(),
            msg: String::from(self.msg()),
            line: self.line(),
            path: self.path(),
            level: self.level(),
            reason: self.reason(),
            stmt: Some(String::from(self.stmt())),
            tips: self.tips().clone(),
            timestamp: self.timestamp(),
        }
    }
}


