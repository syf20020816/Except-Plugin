//! # Msg for exceptions
//! these messages are default , you should define msg when you new the exceptions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/13
//! @version:0.0.1
//! @description:
//! ```

pub const SUPER_MSG: &str = "SupperException: this exception cannot match any error , if you want to fix , you should use lower level exception!";
pub const EASY_MSG: &str = "EasyException: this is an easy exception!";
pub const NULL_POINTER_MSG: &str = "NullPointerException: if you see this exception , you may have a null pointer exception , please check!";
pub const OUT_OF_BOUNDS_MSG: &str = "ArrayOutOfBoundsException: target index out of bounds for length!";
pub const ILLEGAL_ARG_MSG: &str = "IllegalArgumentException: your current input parameter is illegal, please check!";
pub const ILLEGAL_STATE_MSG: &str = "IllegalStateException: current status is illegal , please check!";
pub const IO_MSG: &str = "IOException: if you see this exception , current read write operation exception , please check!";
pub const FILE_NOT_FOUND_MSG: &str = "FileNotFoundException: unable to find the target file currently , please check!";
pub const SQL_MSG: &str = "SQLException: sql error , please check!";
pub const INTERRUPTED_MSG: &str = "InterruptedException: the current thread is in a waiting or sleeping state , please check!";
pub const CLASSCAST_MSG: &str = "ClassCastException: class conversion exception, , please check!";
pub const UNSUPPORTED_OPERATION_MSG: &str = "UnSupportedOperationException: the current operation is not allowed , please check!";
pub const DEFINE_MSG: &str = "DefineException: this is the default exception , you should reset it!";


