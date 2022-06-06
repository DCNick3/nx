use crate::rc;

pub const RESULT_SUBMODULE: u32 = 1100;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultErrorCodeInvalid: 1,
    ResultErrorCodePermissionDenied: 2,
    ResultErrorCodeNameNotFound: 3,
    ResultErrorCodeWouldBlock: 4,
    ResultErrorCodeNoMemory: 5,
    ResultErrorCodeAlreadyExists: 6,
    ResultErrorCodeNoInit: 7,
    ResultErrorCodeBadValue: 8,
    ResultErrorCodeDeadObject: 9,
    ResultErrorCodeInvalidOperation: 10,
    ResultErrorCodeNotEnoughData: 11,
    ResultErrorCodeUnknownTransaction: 12,
    ResultErrorCodeBadIndex: 13,
    ResultErrorCodeTimeOut: 14,
    ResultErrorCodeFdsNotAllowed: 15,
    ResultErrorCodeFailedTransaction: 16,
    ResultErrorCodeBadType: 17
});
