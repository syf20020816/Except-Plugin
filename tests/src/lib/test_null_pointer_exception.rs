use std::error::Error;
use std::{line, file};
use std::path::PathBuf;
use except_plugin::{
    ExceptionLevel, ExceptionFactory, NullPointerException, NullPointerExceptionBuilder, NewFrom, SuperBuilderImpl,
    DerefException, Exception, TargetParamImpl, CommonParamImpl,null_pointer_e,ExceptionCode
};

pub fn test_null_pointer() -> Result<(), Box<dyn Error>> {
    let e = ExceptionFactory::new::<NullPointerException, NullPointerExceptionBuilder>()
        .set_code(500)
        .set_msg("Null Pointer")
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .set_target("e1")
        .build();
    dbg!(&e);
    dbg!(&e.timestamp());
    Err(Box::new(
        e
    ))
}

pub fn test_null_pointer_macro() -> () {
    let e = null_pointer_e!();
    dbg!(e);
}