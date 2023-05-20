#[repr(u32)]
pub enum ErrorCode {
    Success = 0,
    InvalidEnumeration = 0x500,
    InvalidValue = 0x501,
    InvalidOperation = 0x502,
    StackOverflow = 0x503,
    StackUnderflow = 0x504,
    OutOfMemory = 0x505,
}

#[no_mangle]
pub extern "C" fn glGetError() -> ErrorCode {
    ErrorCode::Success
}
