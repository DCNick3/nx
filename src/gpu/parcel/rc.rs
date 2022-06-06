use crate::rc;

pub const RESULT_SUBMODULE: u32 = 1200;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultNotEnoughReadSpace: 1,
    ResultNotEnoughWriteSpace: 2,
    ResultFdsNotSupported: 3,
    ResultReadSizeMismatch: 4
});
