use crate::rc;

pub const RESULT_SUBMODULE: u32 = 100;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultRelaSizeMismatch: 1,
    ResultInvalidModuleMagic: 2
});
