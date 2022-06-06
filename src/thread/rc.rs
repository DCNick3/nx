use crate::rc;

pub const RESULT_SUBMODULE: u32 = 900;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultInvalidStack: 1,
    ResultInvalidState: 2
});
