#[repr(u32)]
pub enum ErrorCode {
    NoError = 0,
    InvalidEnumeration = 0x500,
    InvalidValue = 0x501,
    InvalidOperation = 0x502,
    StackOverflow = 0x503,
    StackUnderflow = 0x504,
    OutOfMemory = 0x505,
    // TODO(wpieterse): This is defined in the spec, but there is no definition
    // of it inside the header.
    // TableTooLarge,
}

#[no_mangle]
pub extern "C" fn glGetError() -> ErrorCode {
    ErrorCode::NoError
}
