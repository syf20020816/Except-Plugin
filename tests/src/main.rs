mod lib;

use std::error::Error;
use std::path::PathBuf;
///✅测试成功
use crate::lib::test_super_exception::{test_super_exception, test_super_exception_result};
// use crate::lib::test_null_pointer::test_null_pointer;
use except_plugin::{
    SuperBuilder, NewFrom, SuperException, ExceptionFactory, Exceptions, SuperBuilderImpl, ExceptionLevel, EasyException,
    EasyExceptionBuilder, CommonParamImpl, NullPointerException, NullPointerExceptionBuilder, TargetParam,
};


fn main() {
    // test_super_exception();
    // let e = test_super_exception_result();
    // match e {
    //     Ok(_) => {}
    //     Err(err) => {
    //         println!("{:?}", err.description());
    //     }
    // }
    let e1 = ExceptionFactory::new::<EasyException, EasyExceptionBuilder>()
        .set_code(500)
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .build();
    dbg!(e1);
    let e2 = ExceptionFactory::new::<NullPointerException, NullPointerExceptionBuilder>()
        .set_code(500)
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .set_target("e1")
        .build();
    dbg!(e2);
}
