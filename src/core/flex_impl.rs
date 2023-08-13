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

use super::{SupperException, Exception, ExceptionLevel, ExceptionCode, SUPPER_MSG};

/// # Builder impl
/// each exception should impl this trait
/// ## example
/// ```rust
///impl BuilderImpl for SupperBuilder {
///     type Output = SupperException;
///
///     fn new() -> Self {
///         Default::default()
///     }
///     fn code(&self) -> u32 {
///         self.code
///     }
///
///     fn msg(&self) -> &str {
///         &self.msg
///     }
///
///     fn level(&self) -> ExceptionLevel {
///         self.level.clone()
///     }
///
///     fn set_code(&mut self, code: u32) -> &mut Self {
///         self.code = code;
///         self
///     }
///
///     fn set_msg(&mut self, msg: &str) -> &mut Self {
///         self.msg = String::from(msg);
///         self
///     }
///
///     fn set_level(&mut self, level: ExceptionLevel) -> &mut Self {
///         self.level = level;
///         self
///     }
///
///     fn build(&mut self) -> Self::Output {
///         SupperException {
///             code: self.code(),
///             msg: String::from(self.msg()),
///             level: self.level(),
///         }
///     }
/// }
/// ```
pub trait BuilderImpl {
    /// # Output type
    /// you should set for build func
    /// ```rust
    /// type Output = SupperException;
    /// ```
    type Output;
    fn new() -> Self;
    fn code(&self) -> u32;
    fn msg(&self) -> &str;
    fn level(&self) -> ExceptionLevel;
    fn set_code(&mut self, code: u32) -> &mut Self;
    fn set_msg(&mut self, msg: &str) -> &mut Self;
    fn set_level(&mut self, level: ExceptionLevel) -> &mut Self;
    fn build(&mut self) -> Self::Output;
}


/// # New or From Exception
/// Use this trait to create an Exception
/// ## example
/// ```rust
/// impl NewFrom for SupperException {
///     type Builder = SupperBuilder;
///
///     fn new() -> Self::Builder {
///         SupperBuilder::new()
///     }
///     fn from(e: Box<dyn Exception>) -> Self where Self: Sized {
///         SupperException {
///             code: e.code(),
///             msg: String::from(e.msg()),
///             level: e.level(),
///         }
///     }
/// }
/// ```
pub trait NewFrom {
    type Builder:BuilderImpl;
    /// create a new Exception
    fn new() -> Self::Builder;
    /// create a new Exception from any Exception
    /// - can convert from : supper|lower
    fn from(e: Box<dyn Exception>) -> Self where Self: Sized;
}

// pub trait FromDyn {
//     /// dyn create a new Exception
//     fn from_dyn(f: impl Fn()) -> Self;
// }

