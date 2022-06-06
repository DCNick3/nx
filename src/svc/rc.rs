pub const RESULT_MODULE: u32 = 1;

result_define_group!(RESULT_MODULE => {
    ResultInvalidSize: 101,
    ResultInvalidAddress: 102,
    ResultInvalidHandle: 114,
    ResultTimedOut: 117,
    ResultCancelled: 118,
    ResultSessionClosed: 123,
    ResultNotHandled: 124,
    ResultDebug: 128
});
