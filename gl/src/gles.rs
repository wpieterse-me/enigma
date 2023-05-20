#[repr(u32)]
pub enum ErrorCode {
    /// The operation has completed successfully
    Success = 0,

    /// Enumeration argument is out of range
    InvalidEnumeration = 0x500,

    /// Numeric argument is out of range
    InvalidValue = 0x501,

    /// The requested operation is illegal in the current state
    InvalidOperation = 0x502,

    /// The requested operation would cause a stack overflow
    StackOverflow = 0x503,

    /// The requested operation would cause a stack underflow
    StackUnderflow = 0x504,

    /// Not enough memory is available to execute the requested operation
    OutOfMemory = 0x505,
}

#[no_mangle]
pub extern "C" fn glGetError() -> ErrorCode {
    ErrorCode::Success
}
