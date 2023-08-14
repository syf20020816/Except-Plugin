use std::error::Error;
use except_plugin::{SuperBuilder, SuperException, ExceptionFactory, Exceptions, SuperBuilderImpl, ExceptionLevel, Exception, DerefException};

pub fn test_super_exception() {
    // use ExceptionFactory -> get SuperBuilder -> build SuperException
    let e = ExceptionFactory::new::<SuperException, SuperBuilder>()
        .set_code(1006)
        .set_msg("super builder")
        .set_level(ExceptionLevel::Fatal)
        .build();
    dbg!(e);
}

pub fn test_super_exception_result() -> Result<(), Box<dyn Error>> {
    // build a exception
    let mut e = ExceptionFactory::new::<SuperException, SuperBuilder>()
        .set_code(1006)
        .set_msg("super builder")
        .set_level(ExceptionLevel::Fatal)
        .build();
    e.set_msg("this is a super exception!");
    let e =  e.deref_mut_exception();
    Err(Box::new(e))
}