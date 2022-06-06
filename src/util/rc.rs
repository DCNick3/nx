use crate::rc;

pub const RESULT_SUBMODULE: u32 = 300;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultInvalidPointer: 1,
    ResultInvalidSize: 2,
    ResultInvalidUtf8Conversion: 3
});
