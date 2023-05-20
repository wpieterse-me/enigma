#[repr(i32)]
pub enum ErrorCode {
    Success = 0x3000,
}

#[no_mangle]
pub extern "C" fn eglGetError() -> ErrorCode {
    ErrorCode::Success
}
