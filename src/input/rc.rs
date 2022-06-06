use crate::rc;

pub const RESULT_SUBMODULE: u32 = 800;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultInvalidControllerId: 1,
    ResultInvalidTouchIndex: 2
});
