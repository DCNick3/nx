pub const RESULT_MODULE: u32 = 10;

result_define_group!(RESULT_MODULE => {
    ResultInvalidHeaderSize: 202,
    ResultInvalidInputHeader: 211,
    ResultInvalidOutputHeader: 212,
    ResultInvalidCommandRequestId: 221,
    ResultInvalidInObjectCount: 235,
    ResultInvalidOutObjectCount: 236
});
