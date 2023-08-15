//! # Exception Core
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```
mod macros;
mod e_msg;
mod flex_impl;
mod easy;
mod null_pointer;
mod builder;
mod index;
mod sql;
mod unsupported;

pub use easy::EasyException;
pub use null_pointer::NullPointerException;
pub use index::ArrayIndexOutOfBoundsException;
pub use unsupported::UnSupportedOpException;
pub use sql::SQLException;
pub use flex_impl::*;
pub use e_msg::*;
pub use builder::*;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::{display_err_impl, exception_impl};


/// # Exception trait
/// each exception should impl this trait
/// - can convert
/// - can change
pub trait Exception: Error {
    fn code(&self) -> u32;
    fn msg(&self) -> &str;
    fn level(&self) -> ExceptionLevel;
    fn set_code(&mut self, code: u32) -> ();
    fn set_msg(&mut self, msg: &str) -> ();
    fn set_level(&mut self, level: ExceptionLevel) -> ();
    fn get_type(&self) -> Exceptions;
    fn timestamp(&self) -> Duration;
}

/// # Exceptions enum
/// - Super（顶级异常）：顶级异常只能用于简单的处理，实际上他并不能显式的知道程序到底有什么问题，需要更加具体的异常。它虽然是顶级异常，但却是最低级的异常。
/// - NullPointerException（空指针异常）：当代码尝试访问一个空对象的方法或属性时抛出。
/// - ArrayIndexOutOfBoundsException（数组越界异常）：当试图访问数组中不存在的索引位置时抛出。
/// - IllegalArgStateException（非法参数|状态异常）：当传递给方法的参数不符合要求或无效时抛出或对象的状态与调用方法的前提条件不符时抛出。
/// - IOException（输入/输出异常）：当发生与输入/输出操作相关的错误时抛出，如文件读写错误、网络连接问题等。
/// - FileNotFoundException（文件未找到异常）：当尝试打开或访问不存在的文件时抛出。
/// - SQLException（SQL异常）：与数据库操作相关的异常，如连接失败、SQL语句错误等。
/// - InterruptedException（线程中断异常）：当线程在等待或睡眠状态被中断时抛出。
/// - ClassCastException（类转换异常）：当试图将一个对象强制转换为其不兼容的类型时抛出。
/// - UnsupportedOperationException（不支持的操作异常）：当调用对象不支持的方法或操作时抛出。
/// - Define（自定义异常）：当自己定义非上访的异常时。
#[derive(Debug, Clone, PartialEq)]
pub enum Exceptions {
    Super,
    Easy,
    NullPointer,
    ArrayIndexOutOfBounds,
    IllegalArgState,
    IO,
    FileNotFound,
    SQL,
    Interrupted,
    ClassCast,
    UnSupportedOperation,
    Define,
}

/// # Exception Code 异常码
/// - common exception or supper exception should use 101
/// - you may need to define exception code
pub struct ExceptionCode(u32);

impl ExceptionCode {
    pub const COMMON: u32 = 101;
    pub const SUPER: u32 = 102;
    pub const NULL_POINTER: u32 = 1000;
    pub const ARRAY_INDEX_OUT_OF: u32 = 1100;
    pub const ILLEGAL_ARGUMENT: u32 = 1200;
    pub const ILLEGAL_STATE: u32 = 1300;
    pub const IO: u32 = 1400;
    pub const FILE_NOT_FOUND: u32 = 1500;
    pub const SQL: u32 = 1600;
    pub const INTERRUPTED: u32 = 1700;
    pub const CLASS_CAST: u32 = 1800;
    pub const UNSUPPORTED_OPERATION: u32 = 1900;
    /// 自定义异常码
    pub fn define(&mut self, code: u32) -> ExceptionCode {
        Self(code)
    }
}

/// # Exception Error Level 异常等级
///
/// ExceptionLevel should match LoggerLevel to print or debug information
///
/// 异常等级可对应日志等级，对应进行输出
/// ## description
/// - Error == Fatal : panic or stop the project
/// - Warn : can fix or panic | stop the project
/// - Debug : for debugging
/// - Info : default level
/// - Trace : lowest level
#[derive(Debug, Clone, PartialEq)]
pub enum ExceptionLevel {
    Error,
    Fatal,
    Warn,
    Debug,
    Info,
    Trace,
}

/// # Reason for UnSupported
#[derive(Debug, PartialEq, Clone)]
pub enum Reasons {
    UnSupported(UnSupportedReasons),
    SQL(SQLReasons),
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnSupportedReasons {
    /// Illegal value
    /// such as : a param need u32 , but get value < 0 or None(Option) or bigger than u32::MAX_VALUE
    Value,
    /// Illegal type
    /// such as : a param need u32 , but get bool
    Type,
    /// thread block
    Block,
    /// cannot get lock
    Lock,
    /// cannot access
    UnAccessible,
    /// No permission to access
    Auth,
    /// IO
    IO,
    /// file not found
    FileNotFound,
    /// interrupt when thread waiting or sleeping
    Interrupted,
    /// conert error
    ClassCast,
    /// other reason
    /// specific reasons need to be set on msg param
    Other,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SQLReasons {
    /// SQL statement error
    Stmt,
    /// SQL Logic error
    Logic,
    /// no Database
    NO_DB,
    /// no table exist
    NO_Table,
    /// no column
    NO_Column,
    /// no rows
    NO_Rows,
    /// empty
    Empty,
    /// no authorization
    NO_Auth,
    /// no namespace
    NO_Namespace,
    /// Insert fail
    Insert,
    /// Update fail
    Update,
    /// Query fail
    Query,
    /// Delete | Drop fail
    Delete,
    Other,
}

/// # Supper Exception
/// It is the top-level implementation of all exceptions , you can get this from all more specific exceptions' recover param
/// > Although it is the parent of all exceptions, it is actually the lowest level exception
/// ## example
/// ```rust
/// use std::error::Error;
/// use except_plugin::{SuperBuilder, SuperException, ExceptionFactory, Exceptions, SuperBuilderImpl, ExceptionLevel, Exception, DerefException};
///
/// pub fn test_super_exception() {
///     // use ExceptionFactory -> get SuperBuilder -> build SuperException
///     let e = ExceptionFactory::new::<SuperException, SuperBuilder>()
///         .set_code(1006)
///         .set_msg("super builder")
///         .set_level(ExceptionLevel::Fatal)
///         .build();
///     dbg!(e);
/// }
///
/// pub fn test_super_exception_result() -> Result<(), Box<dyn Error>> {
///     // build a exception
///     let mut e = ExceptionFactory::new::<SuperException, SuperBuilder>()
///         .set_code(1006)
///         .set_msg("super builder")
///         .set_level(ExceptionLevel::Fatal)
///         .build();
///     e.set_msg("this is a super exception!");
///     let e =  e.deref_mut_exception();
///     Err(Box::new(e))
/// }
/// fn main() {
///     test_super_exception();
///     let e = test_super_exception_result();
///     match e {
///         Ok(_) => {}
///         Err(err) => {
///             println!("{:?}", err.description());
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SuperException {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    timestamp: Duration,
}


impl NewFrom for SuperException {
    type Builder = SuperBuilder;
    fn new() -> Self::Builder {
        SuperBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        SuperException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            timestamp: e.timestamp(),
        }
    }
}

impl FromBuilder for SuperException {
    type Output = SuperException;
    type Input = SuperBuilder;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            timestamp: builder.timestamp(),
        }
    }
}
//generate display and error for SuperException
display_err_impl!(SuperException);

exception_impl!(SuperException,Exceptions::Super);

impl DerefException for SuperException {
    fn deref_mut_exception(&mut self) -> Self {
        SuperException {
            code: self.code(),
            msg: String::from(self.msg()),
            level: self.level(),
            timestamp: self.timestamp(),
        }
    }
}

impl Default for SuperException {
    fn default() -> Self {
        SuperException {
            code: ExceptionCode::SUPER,
            msg: String::from(SUPER_MSG),
            level: ExceptionLevel::Info,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

