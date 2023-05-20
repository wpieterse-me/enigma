use std::ffi::{c_void, CStr, c_char};

type EGLDebugCallbackFn = extern "C" fn(
    error: u32,
    command: *const c_char,
    message_type: i32,
    thread_label: i32,
    object_label: i32,
    message: *const c_char,
);

#[repr(transparent)]
pub struct EGLDisplayID(*mut c_void);

#[repr(transparent)]
pub struct EGLDisplayHandle(*mut c_void);

#[link(name = "gl")]
extern "C" {
    fn eglDebugMessageControlKHR(callback: EGLDebugCallbackFn, attribute_list: *const i32) -> i32;
    fn eglGetDisplay(display_id: EGLDisplayID) -> EGLDisplayHandle;
}

#[no_mangle]
extern "C" fn test_callback(
    error: u32,
    command: *const i8,
    message_type: i32,
    thread_label: i32,
    object_label: i32,
    message: *const i8,
) {
    println!(
        "CALLBACK : {} - {}",
        unsafe { CStr::from_ptr(command).to_str().unwrap() },
        unsafe { CStr::from_ptr(message).to_str().unwrap() }
    );
}

fn main() {
    let empty: i32 = 0;

    unsafe {
        eglDebugMessageControlKHR(test_callback, &empty);
        eglGetDisplay(EGLDisplayID(std::ptr::null_mut()));
    }
}
