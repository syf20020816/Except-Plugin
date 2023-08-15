use std::error::Error;
use std::{line, file};
use std::path::PathBuf;
use except_plugin::{
    ExceptionLevel, ExceptionFactory, ArrayIndexOutOfBoundsException, ArrayIndexOutOfBoundsBuilder, NewFrom, SuperBuilderImpl,
    DerefException, Exception, TargetParamImpl, CommonParamImpl, OutOfBoundsParamImpl, array_out_of_bounds_e, ExceptionCode,
};

pub fn test_out_of_bounds() -> Result<(), Box<dyn Error>> {
    let e = ExceptionFactory::new::<ArrayIndexOutOfBoundsException, ArrayIndexOutOfBoundsBuilder>()
        .set_code(500)
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .set_target("arr1")
        .set_index(10)
        .set_len(4)
        .build();
    dbg!(&e);
    Err(Box::new(
        e
    ))
}

pub fn test_out_of_bounds_macro() -> () {
    let e = array_out_of_bounds_e!();
    dbg!(e);
}