mod lib;

use std::error::Error;
use std::path::PathBuf;
///✅测试成功
use crate::lib::test_super_exception::{test_super_exception, test_super_exception_result};
// use crate::lib::test_null_pointer::test_null_pointer;
use except_plugin::{SuperBuilder, SuperException, ExceptionFactory, Exceptions, SuperBuilderImpl, ExceptionLevel};


fn main() {
    test_super_exception();
    let e = test_super_exception_result();
    match e {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err.description());
        }
    }
}
