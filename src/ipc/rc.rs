use crate::rc;

pub const RESULT_SUBMODULE: u32 = 600;

result_define_subgroup!(rc::RESULT_MODULE, RESULT_SUBMODULE => {
    ResultCopyHandlesFull: 1,
    ResultMoveHandlesFull: 2,
    ResultDomainObjectsFull: 3,
    ResultInvalidDomainObject: 4,
    ResultPointerSizesFull: 5,
    ResultSendStaticsFull: 6,
    ResultReceiveStaticsFull: 7,
    ResultSendBuffersFull: 8,
    ResultReceiveBuffersFull: 9,
    ResultExchangeBuffersFull: 10,
    ResultInvalidSendStaticCount: 11,
    ResultInvalidReceiveStaticCount: 12,
    ResultInvalidSendBufferCount: 13,
    ResultInvalidReceiveBufferCount: 14,
    ResultInvalidExchangeBufferCount: 15,
    ResultInvalidBufferAttributes: 16,
    ResultInvalidProtocol: 17
});
