use std::error::Error;
use std::{line, file};
use std::path::PathBuf;
use except_plugin::{
    ExceptionLevel, ExceptionFactory, EasyException, EasyExceptionBuilder, NewFrom, SuperBuilderImpl,
    DerefException, Exception, TargetParamImpl, CommonParamImpl, easy_e,
};

pub fn test_easy() -> Result<(), Box<dyn Error>> {
    let e = ExceptionFactory::new::<EasyException, EasyExceptionBuilder>()
        .set_code(500)
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .build();
    dbg!(&e);
    Err(Box::new(
        e
    ))
}

pub fn test_easy_macro() -> () {
    // let e = easy_e!(666,"666");
    // dbg!(e);
}