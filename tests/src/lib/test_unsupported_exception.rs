use std::error::Error;
use std::{line, file};
use std::path::PathBuf;
use except_plugin::{
    ExceptionLevel, ExceptionFactory, NewFrom, Reasons, SuperBuilderImpl, DerefException, Exception, TargetParamImpl, CommonParamImpl,
    ExceptionCode, UnSupportedOpExceptionBuilder, UnSupportedOpException, ReasonParamImpl, unsupported_op_e,UnSupportedReasons
};

pub fn test_unsupport() -> Result<(), Box<dyn Error>> {
    let e = ExceptionFactory::new::<UnSupportedOpException, UnSupportedOpExceptionBuilder>()
        .set_code(6666)
        .set_msg("Lock!")
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .set_reason(Reasons::UnSupported(UnSupportedReasons::Block))
        .build();
    dbg!(&e);
    Err(Box::new(
        e
    ))
}

pub fn test_unsupport_macro() -> () {
    let e = unsupported_op_e!(45,"main thread sleep");
    dbg!(e);
}