pub const RESULT_MODULE: u32 = 2;

result_define_group!(RESULT_MODULE => {
    ResultPathNotFound: 1,
    ResultPathAlreadyExists: 2
});
