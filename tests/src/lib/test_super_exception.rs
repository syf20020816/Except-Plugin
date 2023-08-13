use except_plugin::{SupperBuilder, BuilderImpl, ExceptionLevel, SupperException, Exception, NewFrom};

pub fn test_super_exception() {
    let mut builder = SupperBuilder::new();
    let builder = builder
        .set_code(10086)
        .set_msg("supper exp")
        .set_level(ExceptionLevel::Warn)
        .build();
    let exception = SupperException::new()
        .set_code(10086)
        .set_msg("supper exp")
        .set_level(ExceptionLevel::Warn)
        .build();
    dbg!(builder);
    dbg!(exception);
}
