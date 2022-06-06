use crate::rc;

pub const RESULT_SUBMODULE: u32 = 1300;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultObjectIdAlreadyAllocated: 1,
    ResultDomainNotFound: 2,
    ResultInvalidCommandType: 3,
    ResultInvalidDomainCommandType: 4,
    ResultSignaledServerNotFound: 5,
    ResultAlreadyDomain: 6
});
