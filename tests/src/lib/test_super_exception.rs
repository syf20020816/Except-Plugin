use std::error::Error;
use except_plugin::{SupperBuilder, ExceptionLevel, SupperException, Exception, NewFrom};

pub fn test_super_exception() {
    let mut builder = SupperBuilder::new();
    let builder = builder
        .set_code(10086)
        .set_msg("supper exp")
        .set_level(ExceptionLevel::Warn)
        .build();
    let exception = SupperException::new()
        .set_code(10086)
        .set_msg("supper exp")
        .set_level(ExceptionLevel::Warn)
        .build();
    dbg!(builder);
    dbg!(exception);
}

pub fn test_super_exception_result() -> Result<(), Box<dyn Error>> {
    let mut e = SupperException::new()
        .set_code(101)
        .set_msg("this is a super exception")
        .set_level(ExceptionLevel::Error)
        .build();
    e.set_msg("change super exception");
    let e =  e.deref_mut();
    Err(Box::new(e))
}