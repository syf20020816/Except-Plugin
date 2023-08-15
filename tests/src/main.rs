mod lib;

use std::error::Error;
use std::path::PathBuf;
///✅测试成功
use crate::lib::test_super_exception::{test_super_exception, test_super_exception_result};
///✅测试成功
use crate::lib::test_easy_exception::{test_easy, test_easy_macro};
///✅测试成功
use crate::lib::test_null_pointer_exception::{test_null_pointer, test_null_pointer_macro};
use crate::lib::test_array_out_of_bounds_exception::{test_out_of_bounds, test_out_of_bounds_macro};
use crate::lib::test_unsupported_exception::{test_unsupport, test_unsupport_macro};
use crate::lib::test_sql_exception::{test_sql, test_sql_macro};
use crate::lib::define_exception::test_my_exception;
use except_plugin::{
    super_e, SuperException, SuperBuilderImpl, NewFrom, easy_e, EasyException,
    CommonParamImpl, EasyExceptionBuilder, ExceptionFactory, ExceptionCode, ExceptionLevel,
    null_pointer_e, NullPointerException, NullPointerExceptionBuilder, TargetParamImpl,
};

fn main() {
    // let _ = test_super_exception();
    // let e = test_null_pointer();
    // match e {
    //     Ok(_) => {}
    //     Err(err) => {
    //         println!("{:?}", err.description());
    //     }
    // }
    // let _ = test_easy();
    // let _ = test_easy_macro();
    // let _ = test_null_pointer();
    // let _ = test_out_of_bounds();
    // let _ = test_null_pointer_macro();
    // let _ = test_out_of_bounds_macro();
    // let e = test_unsupport();
    // match e {
    //     Ok(_) => {}
    //     Err(e) => {
    //         println!("{:?}", e.description());
    //     }
    // }
    // let _ = test_unsupport_macro();
    // let _ = test_sql_macro();
    let _ = test_my_exception();
}
