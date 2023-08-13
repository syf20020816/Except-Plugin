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
    type Builder;
    /// create a new Exception
    fn new() -> Self::Builder;
    /// create a new Exception from any Exception
    /// - can convert from : supper|lower
    fn from(e: Box<dyn Exception>) -> Self where Self: Sized;
}

pub trait FromBuilder {
    type Input;
    type Output;
    fn from_builder(builder: &mut Self::Input) -> Self::Output;
}

// pub trait FromDyn {
//     /// dyn create a new Exception
//     fn from_dyn(f: impl Fn()) -> Self;
// }

