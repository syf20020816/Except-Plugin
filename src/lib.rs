//! # Except-Plugin
//! - author：syf20020816@outlook.com
//! - docName：Except-Plugin README
//! - createDate：20230814
//! - updateDate：20230814
//! - version：0.0.1
//! - email：syf20020816@outlook.com
//!
//! ## LICEMSE
//!
//! MIT
//!
//! ## Except-Plugin Introduction
//!
//! exception-plugin is a common exception in Rust， which helps developers better control their programs
//!
//! ### Default Support Exception
//!
//! - Super (top level exception): top level exceptions can only be used for simple handling, in fact, they cannot explicitly know what the program is having, and more specific exceptions are needed. Although it is a top level exception, it is the lowest level exception.
//! - NullPointerException: thrown when code attempts to access a method or property of an empty object.
//! - ArrayIndexOutOfBoundsException: thrown when attempting to access an index position in an array that does not exist.
//! - IllegalArgumentException: Thrown when the parameter passed to the method does not meet the requirements or is invalid.
//! - IllegalStateException: Thrown when the state of the object does not match the prerequisite for calling the method.
//! - IOException: Thrown when an error related to input/output operations occurs, such as file read/write errors, network connectivity issues, etc.
//! - FileNotFoundException: thrown when attempting to open or access a file that does not exist.
//! - SQLException: An exception related to database operations, such as connection failure, SQL statement errors, etc.
//! - InterruptedException: Thrown when a thread is interrupted while waiting or sleeping.
//! - ClassCastException: Thrown when attempting to cast an object to its incompatible type.
//! - UnsupportedOperationException: Thrown when an unsupported method or operation is called on an object.
//! - Define (custom exception): When defining non visited exceptions by oneself.
//!
//! ## QuickStart
//!
//! ```rust
//!
//! ```
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/14
//! @version:0.0.1
//! @description:
//! ```
mod core;

pub use crate::core::{
    SuperBuilder, ExceptionLevel, SuperException, Exception, NewFrom, ExceptionFactory, Exceptions, SuperBuilderImpl,
    DerefException, EasyException, NullPointerException, TargetParamImpl, EasyExceptionBuilder, CommonParamImpl,
    NullPointerExceptionBuilder, ArrayIndexOutOfBoundsException, ArrayIndexOutOfBoundsBuilder,OutOfBoundsParamImpl,
    ExceptionCode
};
