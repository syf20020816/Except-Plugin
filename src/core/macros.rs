//! # Macros Output
//!
//! These macros have zero overhead and can be used directly. 
//! If you find it difficult to adapt to factory built exceptions, you can use these macros, and their names are very straightforward
//!
//! ## features
//!
//! if you wanna use these macros , you must add `features=["macros"]`
//!
//! ```toml
//! [dependencies]
//! except-plugin = { version="0.0.1", features = ["macros"] }
//! ```
//! ----------------------------------------------------------------------
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/14
//! @version:0.0.1
//! @description:
//! ```


/// # macro for SuperException
/// ## example
/// ```rust
/// use except_plugin::{super_e};
///
/// let e3 = super_e!(103,"hello",ExceptionLevel::Debug);
///
/// /**--------------------------------------
/// [src\main.rs:39] e3 = SuperException {
///     code: 1006,
///     msg: "super builder",
///     level: Fatal,
/// }
/// ----------------------------------------*/
/// let e4 = super_e!(55);
/// let e5 = super_e!(10,"error");
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! super_e {
    ()=>{SuperException::new()};
    ($Code:expr)=>{
        super_e!($Code,"",ExceptionLevel::Info)
    };
    ($Code:expr,$Msg:expr)=>{
        super_e!($Code,$Msg,ExceptionLevel::Info)
    };
    ($Code:expr,$Level:expr)=>{
        super_e!($Code,"",$Level)
    };
    ($Code:expr,$Msg:expr,$Level:expr) => {
        ExceptionFactory::new::<SuperException, SuperBuilder>()
        .set_code($Code)
        .set_msg($Msg)
        .set_level($Level)
        .build();
    };
}

/// # macro for EasyException
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! easy_e {
    () =>{easy_e!(ExceptionCode::COMMON,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()))};
    ($Code:expr) =>{easy_e!($Code,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()))};
    ($Code:expr,$Msg:expr) =>{easy_e!($Code,$Msg,ExceptionLevel::Info,line!(),PathBuf::from(file!()))};
    ($Code:expr,$Msg:expr,$Level:expr) =>{easy_e!($Code,$Msg,$Level,line!(),PathBuf::from(file!()))};
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr) =>{easy_e!($Code,$Msg,$Level,$Line,PathBuf::from(file!()))};
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr) => {
        ExceptionFactory::new::<EasyException, EasyExceptionBuilder>()
        .set_code($Code)
        .set_msg($Msg)
        .set_level($Level)
        .set_line($Line)
        .set_path($Path)
        .build();
    };
}

#[cfg(feature = "macros")]
#[macro_export]
macro_rules! null_pointer_e {
    ()=>{
        null_pointer_e!(ExceptionCode::NULL_POINTER,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()),"")
    };
    ($Code:expr) =>{null_pointer_e!($Code,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()),"")};
    ($Code:expr,$Msg:expr) =>{null_pointer_e!($Code,$Msg,ExceptionLevel::Info,line!(),PathBuf::from(file!()),"")};
    ($Code:expr,$Msg:expr,$Level:expr) =>{null_pointer_e!($Code,$Msg,$Level,line!(),PathBuf::from(file!()),"")};
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr) =>{null_pointer_e!($Code,$Msg,$Level,$Line,PathBuf::from(file!()),"")};
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr)=>{null_pointer_e!($Code,$Msg,$Level,$Line,$Path,"")};
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr,$Target:expr) => {
        ExceptionFactory::new::<NullPointerException, NullPointerExceptionBuilder>()
        .set_code($Code)
        .set_msg($Msg)
        .set_level($Level)
        .set_line($Line)
        .set_path($Path)
        .set_target($Target)
        .build();
    };
}

#[cfg(feature = "macros")]
#[macro_export]
macro_rules! array_out_of_bounds_e {
    ()=>{array_out_of_bounds_e!(ExceptionCode::ARRAY_INDEX_OUT_OF,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()),"",0,0)};
    ($Code:expr) =>{
        array_out_of_bounds_e!($Code,"",ExceptionLevel::Info,line!(),PathBuf::from(file!()),"",0,0)
    };
    ($Code:expr,$Msg:expr) =>{
        array_out_of_bounds_e!($Code,$Msg,ExceptionLevel::Info,line!(),PathBuf::from(file!()),"",0,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr) =>{
        array_out_of_bounds_e!($Code,$Msg,$Level,line!(),PathBuf::from(file!()),"",0,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr) =>{
        array_out_of_bounds_e!($Code,$Msg,$Level,$Line,PathBuf::from(file!()),"",0,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr)=>{
        array_out_of_bounds_e!($Code,$Msg,$Level,$Line,$Path,"",0,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr,$Target:expr)=>{
        array_out_of_bounds_e!($Code,$Msg,$Level,$Line,$Path,$Target,0,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr,$Target:expr,$Len:expr)=>{
        array_out_of_bounds_e!($Code,$Msg,$Level,$Line,$Path,$Target,$Len,0)
    };
    ($Code:expr,$Msg:expr,$Level:expr,$Line:expr,$Path:expr,$Target:expr,$Len:expr,$Index:expr) => {
        ExceptionFactory::new::<ArrayIndexOutOfBoundsException, ArrayIndexOutOfBoundsBuilder>()
        .set_code($Code)
        .set_msg($Msg)
        .set_level($Level)
        .set_line($Line)
        .set_path($Path)
        .set_target($Target)
        .set_index($Index)
        .set_len($Len)
        .build();
    };
}

