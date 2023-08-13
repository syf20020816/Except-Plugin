use except_plugin::{SupperBuilder, BuilderImpl, ExceptionLevel};

pub fn test_super_builder() {
    let mut builder = SupperBuilder::new();
    let builder = builder
        .set_code(10086)
        .set_msg("supper exp")
        .set_level(ExceptionLevel::Warn);
    println!("{:?}", builder);
}