// use std::error::Error;
// use std::path::PathBuf;
// use except_plugin::{Exception, NullPointerException, NewFrom, ExceptionLevel};
//
// pub fn test_null_pointer() -> Result<(), Box<dyn Exception>> {
//     let mut e = NullPointerException::new()
//         .set_msg("null pointer")
//         .set_code(555)
//         .set_level(ExceptionLevel::Fatal)
//         .set_line(line!())
//         .set_path(PathBuf::from("E://err"))
//         .build();
//     e.set_msg("change null pointer!");
//     println!("{}", e.msg());
//     println!("{:?}", e);
//     let e = e.deref_mut_exception();
//     Err(Box::new(
//         e
//     ))
// }