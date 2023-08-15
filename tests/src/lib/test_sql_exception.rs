use std::error::Error;
use std::{line, file};
use std::path::PathBuf;
use std::collections::HashMap;
use except_plugin::{ExceptionLevel, ExceptionFactory, NewFrom, Reasons, SuperBuilderImpl, DerefException, Exception, TargetParamImpl, CommonParamImpl, ExceptionCode, SQLException, SQLExceptionBuilder, SQLReasons, ReasonParamImpl, SQLParamImpl, sql_e};

pub fn test_sql() -> Result<(), Box<dyn Error>> {
    let e = ExceptionFactory::new::<SQLException, SQLExceptionBuilder>()
        .set_code(6666)
        .set_msg("Lock!")
        .set_stmt("Drop table;")
        .set_level(ExceptionLevel::Warn)
        .set_line(line!())
        .set_path(PathBuf::from(file!()))
        .add_tip("name", "joker")
        .set_reason(Reasons::SQL(SQLReasons::Delete))
        .build();
    dbg!(&e);
    Err(Box::new(
        e
    ))
}

pub fn test_sql_macro() -> () {
    let mut map = HashMap::new();
    map.insert("table".to_string(), "user".to_string());
    let e = sql_e!(
        10,
        "test sql",
        ExceptionLevel::Error,
        line!(),
        PathBuf::from(file!()),
        Reasons::SQL(SQLReasons::Empty),
        "",
        map
    );
    dbg!(e);
}