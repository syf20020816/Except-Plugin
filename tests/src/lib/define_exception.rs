use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::time::Duration;
use except_plugin::{
    SuperBuilderImpl, Exception, ExceptionLevel, exception_impl, display_err_impl, Exceptions, builder_impl,
    NewFrom, FromBuilder, ExceptionFactory,
};

/// # step1 : create a new exception
///
/// mut contain 4 param!:
/// - code
/// - msg
/// - level
/// - timestamp
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MyException {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    timestamp: Duration,
    /// here I define a new param!
    define: String,
}

/// # step2 : use macro to impl Exception for MyException
/// if you wanna impl Exception trait , you must impl Error,Debug,Display,as followings:
/// ```rust
/// impl Error for MyException {}
/// impl Debug for MyException {}
/// impl Display for MyException {}
/// impl Exception for MyException { // fn ... }
/// ```
/// it is very complicated , so you can use exception_impl! and display_err_impl!
display_err_impl!(MyException);
exception_impl!(MyException,Exceptions::Define);

/// # step3 : impl the other param for define Exception
/// actually , now it is enough
/// but you need to continue!!!
impl MyException {
    pub fn get_define(&self) -> &str { &self.define }
    pub fn set_define(&mut self, value: &str) -> &mut Self {
        self.define = String::from(value);
        self
    }
}

/// # step4 : create a builder for define Exception
/// builder is quite simple and you should add a param : `e_type: Exceptions`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MyExceptionBuilder {
    code: u32,
    msg: String,
    level: ExceptionLevel,
    timestamp: Duration,
    /// here I define a new param!
    define: String,
    e_type: Exceptions,
}

impl MyExceptionBuilder {
    pub fn get_define(&self) -> &str { &self.define }
    pub fn set_define(&mut self, value: &str) -> &mut Self {
        self.define = String::from(value);
        self
    }
}

/// # step5 : add builder_impl! macro for Define Exception
builder_impl!(MyExceptionBuilder,MyException);

/// # step6 : impl NewFrom and FromBuilder
impl NewFrom for MyException {
    type Builder = MyExceptionBuilder;
    fn new() -> Self::Builder {
        MyExceptionBuilder::new()
    }
    fn from_super(e: Box<dyn Exception>) -> Self where Self: Sized {
        MyException {
            code: e.code(),
            msg: String::from(e.msg()),
            level: e.level(),
            timestamp: e.timestamp(),
            define: String::new(),
        }
    }
}

impl FromBuilder for MyException {
    type Output = MyException;
    type Input = MyExceptionBuilder;
    fn from_builder(builder: &Self::Input) -> Self::Output {
        Self::Output {
            code: builder.code(),
            msg: String::from(builder.msg()),
            level: builder.level(),
            timestamp: builder.timestamp(),
            define: String::from(builder.get_define()),
        }
    }
}

/// # step 7: test
pub fn test_my_exception() -> () {
    // [src\lib\define_exception.rs:110] my_e = MyException {
    //     code: 1000,
    //     msg: "",
    //     level: Info,
    //     timestamp: 0ns,
    //     define: "this is my exception",
    // }
    let my_e = ExceptionFactory::new::<MyException, MyExceptionBuilder>()
        .set_code(1000)
        .set_define("this is my exception")
        .build();
    dbg!(my_e);
}