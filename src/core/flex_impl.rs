//! # flex impls for exceptions
//! There are many traits to implement here, and when we implement a custom Exception
//!
//! these traits can provide standardized and flexible extension functions for the custom Exception
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration};
use super::{Exception, ExceptionLevel, Exceptions, Reasons};


/// # New or From Exception
/// Use this trait to create an Exception
/// ## example
/// ```rust
/// impl NewFrom for SuperException {
///     type Builder = SuperBuilder;
///
///     fn new() -> Self::Builder {
///         SuperBuilder::new()
///     }
///     fn from(e: Box<dyn Exception>) -> Self where Self: Sized {
///         SuperException {
///             code: e.code(),
///             msg: String::from(e.msg()),
///             level: e.level(),
///         }
///     }
/// }
/// ```
pub trait NewFrom {
    type Builder;
    /// create a new Exception
    fn new() -> Self::Builder;
    /// create a new Exception from any Exception
    /// - can convert from : supper
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized;
}

/// # deref the exception
/// convert &mut self to Self
/// - impl each exception
/// ## example
/// ```rust
/// impl DerefException for SuperException {
///     fn deref_mut_exception(&mut self) -> Self {
///         SuperException {
///             code: self.code(),
///             msg: String::from(self.msg()),
///             level: self.level(),
///         }
///     }
/// }
pub trait DerefException {
    fn deref_mut_exception(&mut self) -> Self;
}

/// # convert builder to exception
/// ## example
/// ```rust
/// impl FromBuilder for SuperException {
///     type Output = SuperException;
///     type Input = SuperBuilder;
///     fn from_builder(builder: &Self::Input) -> Self::Output {
///         Self::Output {
///             code: builder.code(),
///             msg: String::from(builder.msg()),
///             level: builder.level(),
///         }
///     }
/// }
/// ```
pub trait FromBuilder {
    /// builder type
    type Input;
    /// exception type
    type Output;
    fn from_builder(builder: &Self::Input) -> Self::Output;
}

pub trait SuperBuilderImpl<T> {
    fn new() -> Self;
    fn code(&self) -> u32;
    fn msg(&self) -> &str;
    fn level(&self) -> ExceptionLevel;
    fn set_code(&mut self, code: u32) -> &mut Self;
    fn set_msg(&mut self, msg: &str) -> &mut Self;
    fn set_level(&mut self, level: ExceptionLevel) -> &mut Self;
    fn exception_type(&self) -> Exceptions;
    fn timestamp(&self) -> Duration;
    fn build(&mut self) -> T;
}

pub trait CommonParamImpl {
    fn line(&self) -> u32;
    fn path(&self) -> PathBuf;
    fn set_line(&mut self, line: u32) -> &mut Self;
    fn set_path(&mut self, path: PathBuf) -> &mut Self;
}

pub trait TargetParamImpl {
    fn target(&self) -> &str;
    fn set_target(&mut self, target: &str) -> &mut Self;
}

pub trait OutOfBoundsParamImpl {
    fn len(&self) -> usize;
    fn set_len(&mut self, len: usize) -> &mut Self;
    fn index(&self) -> usize;
    fn set_index(&mut self, index: usize) -> &mut Self;
}

pub trait ReasonParamImpl {
    fn reason(&self) -> Reasons;
    fn set_reason(&mut self, reason: Reasons) -> &mut Self;
}

pub trait SQLParamImpl {
    fn stmt(&self) -> &str;
    fn set_stmt(&mut self, stmt: &str) -> &mut Self;
    fn tips(&self) -> &HashMap<String, String>;
    fn add_tip(&mut self, k: &str, v: &str) -> &mut Self;
    fn set_tips(&mut self, tips: HashMap<String, String>) -> &mut Self;
}

//------------------------------------------------------------

//------------------------------------------------------------
/// # generate SuperBuilderImpl for each Builder
/// it will generate implementations for each builder
/// ## example
/// ```rust
/// use crate::builder_impl;
///
/// builder_impl!(SuperBuilder,SuperException);
/// ```
#[macro_export]
macro_rules! builder_impl {
    ($Builder:tt,$Output:tt) => {
        impl SuperBuilderImpl<$Output> for $Builder {

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
            fn exception_type(&self) -> Exceptions {
                self.e_type.clone()
            }
            fn timestamp(&self)->Duration{
                self.timestamp
            }
            fn build(&mut self) -> $Output {
                $Output::from_builder(self.deref())
            }
        }
    };
}

/// generate display and error for all exceptions
#[macro_export]
macro_rules! display_err_impl {
    ($E:tt) => {
        impl Error for $E {
            fn description(&self) -> &str {
                self.msg()
            }
        }

        impl Display for $E {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self, f)
            }
        }
    };
}

/// generate impl Exception for each Exception
#[macro_export]
macro_rules! exception_impl {
    ($E:tt,$EType:expr) => {
        impl Exception for $E {
            fn code(&self) -> u32 {
                self.code
            }
            fn msg(&self) -> &str {
                &self.msg
            }
            fn level(&self) -> ExceptionLevel {
                self.level.clone()
            }
            fn set_code(&mut self, code: u32) -> () {
                self.code = code;
            }
            fn set_level(&mut self, level: ExceptionLevel) -> () {
                self.level = level;
            }
            fn set_msg(&mut self, msg: &str) -> () {
                self.msg = String::from(msg);
            }
            fn get_type(&self) -> Exceptions {
                $EType
            }
            fn timestamp(&self)->Duration{
                self.timestamp
            }
        }
    };
}

/// # Generate NewFrom impl for specific exception
#[macro_export]
macro_rules! e_new_from_impl {
    ($E:tt,$Builder:tt) => {
        impl NewFrom for $E {
            type Builder = $Builder;

            fn new() -> Self::Builder {
                $Builder::new()
            }
            fn from(e: Box<dyn Exception>) -> Self where Self: Sized {
                $E {
                    code: e.code(),
                    msg: String::from(e.msg()),
                    level: e.level(),
                    line: e.line(),
                    path: e.path(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! common_param_impl {
    ($E:tt) => {
        impl CommonParamImpl for $E {
            fn line(&self) -> u32 {
                self.line
            }
            fn path(&self) -> PathBuf {
                self.path.clone()
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
    };
}

/// # macro for TargetParamImpl
#[macro_export]
macro_rules! target_param_impl {
    ($E:tt) => {
        impl TargetParamImpl for $E {
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
    };
}

/// # macro for OutOfBoundsImpl
#[macro_export]
macro_rules! out_of_bounds_impl {
    ($E:tt) => {
        impl OutOfBoundsParamImpl for $E{
            fn len(&self) -> usize {
                self.len
            }
            fn set_len(&mut self, len: usize) -> &mut Self {
                self.len = len;
                self
            }
            fn index(&self) -> usize {
                self.index
            }
            fn set_index(&mut self, index: usize) -> &mut Self {
                self.index = index;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! reason_param_impl {
    ($E:tt) => {
        impl ReasonParamImpl for $E {
            fn reason(&self) -> Reasons {
                self.reason.clone()
            }
            fn set_reason(&mut self, reason: Reasons) -> &mut Self {
                self.reason = reason;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! sql_param_impl {
    ($E:tt) => {
        impl SQLParamImpl for $E {
            fn stmt(&self) -> &str {
                match self.stmt {
                    Some(ref s) => s.as_str(),
                    None => ""
                }
            }
            fn set_stmt(&mut self, stmt: &str) -> &mut Self {
                self.stmt = Some(stmt.to_string());
                self
            }
            fn tips(&self) -> &HashMap<String, String> {
                &self.tips
            }
            fn set_tips(&mut self,tips: HashMap<String, String>) -> &mut Self {
                self.tips = tips;
                self
            }
            fn add_tip(&mut self, k: &str, v: &str) -> &mut Self {
                self.tips.insert(String::from(k), String::from(v));
                self
            }
        }
    };
}