use crate::rc;

pub const RESULT_SUBMODULE: u32 = 700;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultDeviceNotFound: 1,
    ResultInvalidPath: 2,
    ResultNotInSameFileSystem: 3
});
