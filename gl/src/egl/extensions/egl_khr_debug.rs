use std::{
    ffi::{c_char, c_void},
    sync::Once,
};

use crate::egl::EGLErrorCode;

#[repr(transparent)]
pub struct EGLObjectKHR(pub *mut c_void);

#[repr(transparent)]
pub struct EGLLabelKHR(pub *mut c_void);

#[repr(i32)]
pub enum EGLDebugMessageTypeKHR {
    Critical = 0x33B9,
    Error = 0x33BA,
    Warning = 0x33BB,
    Information = 0x33BC,
}

type EGLDebugMessageCallbackKHRFn = extern "C" fn(
    error: EGLErrorCode,
    command: *const c_char,
    message_type: EGLDebugMessageTypeKHR,
    thread_label: EGLLabelKHR,
    object_label: EGLLabelKHR,
    message: *const c_char,
);

struct Debug {
    callback: Option<EGLDebugMessageCallbackKHRFn>,
    enable_critical_messages: bool,
    enable_error_messages: bool,
    enable_warning_messages: bool,
    enable_information_messages: bool,
}

static DEBUG_ONCE: Once = Once::new();
static mut DEBUG: Debug = Debug {
    callback: None,
    enable_critical_messages: true,
    enable_error_messages: true,
    enable_warning_messages: true,
    enable_information_messages: false,
};

impl Debug {
    fn init(&mut self) {}

    fn get() -> &'static Self {
        unsafe { &DEBUG }
    }

    fn get_mut() -> &'static mut Self {
        unsafe { &mut DEBUG }
    }

    fn ensure_init() {
        DEBUG_ONCE.call_once(|| unsafe { DEBUG.init() });
    }
}

pub fn post_debug_msg(
    error_code: EGLErrorCode,
    command: &str,
    message_type: EGLDebugMessageTypeKHR,
    thread_label: EGLLabelKHR,
    object_label: EGLLabelKHR,
    message: &str,
) {
    Debug::ensure_init();

    match message_type {
        EGLDebugMessageTypeKHR::Critical => {
            if Debug::get().enable_critical_messages == false {
                return;
            }
        }
        EGLDebugMessageTypeKHR::Error => {
            if Debug::get().enable_error_messages == false {
                return;
            }
        }
        EGLDebugMessageTypeKHR::Warning => {
            if Debug::get().enable_warning_messages == false {
                return;
            }
        }
        EGLDebugMessageTypeKHR::Information => {
            if Debug::get().enable_information_messages == false {
                return;
            }
        }
    }

    match Debug::get().callback {
        Some(callback) => callback(
            error_code,
            command.as_ptr() as *const c_char,
            message_type,
            thread_label,
            object_label,
            message.as_ptr() as *const c_char,
        ),
        None => {}
    };
}

#[no_mangle]
pub extern "C" fn eglDebugMessageControlKHR(
    callback: Option<EGLDebugMessageCallbackKHRFn>,
    _attribute_list: *const i32,
) -> EGLErrorCode {
    Debug::ensure_init();

    Debug::get_mut().callback = callback;

    EGLErrorCode::Success
}
