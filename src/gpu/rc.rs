use crate::rc;

pub const RESULT_SUBMODULE: u32 = 500;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultNvErrorCodeInvalid: 1,
    ResultNvErrorCodeNotImplemented: 2,
    ResultNvErrorCodeNotSupported: 3,
    ResultNvErrorCodeNotInitialized: 4,
    ResultNvErrorCodeInvalidParameter: 5,
    ResultNvErrorCodeTimeOut: 6,
    ResultNvErrorCodeInsufficientMemory: 7,
    ResultNvErrorCodeReadOnlyAttribute: 8,
    ResultNvErrorCodeInvalidState: 9,
    ResultNvErrorCodeInvalidAddress: 10,
    ResultNvErrorCodeInvalidSize: 11,
    ResultNvErrorCodeInvalidValue: 12,
    ResultNvErrorCodeAlreadyAllocated: 13,
    ResultNvErrorCodeBusy: 14,
    ResultNvErrorCodeResourceError: 15,
    ResultNvErrorCodeCountMismatch: 16,
    ResultNvErrorCodeSharedMemoryTooSmall: 17,
    ResultNvErrorCodeFileOperationFailed: 18,
    ResultNvErrorCodeIoctlFailed: 19
});
