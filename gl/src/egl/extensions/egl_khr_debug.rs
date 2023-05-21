use std::{ffi::c_char, sync::Once};

type EGLDebugCallbackFn = extern "C" fn(
    error: u32,
    command: *const c_char,
    message_type: i32,
    thread_label: i32,
    object_label: i32,
    message: *const c_char,
);

struct Debug {
    debug_callback: Option<EGLDebugCallbackFn>,
}

static DEBUG_ONCE: Once = Once::new();
static mut DEBUG: Debug = Debug {
    debug_callback: None,
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

pub fn post_debug_msg(command: &str, message: &str) {
    Debug::ensure_init();

    match Debug::get().debug_callback {
        Some(callback) => callback(
            0,
            command.as_ptr() as *const c_char,
            0,
            0,
            0,
            message.as_ptr() as *const c_char,
        ),
        None => {}
    };
}

#[no_mangle]
pub extern "C" fn eglDebugMessageControlKHR(
    callback: EGLDebugCallbackFn,
    _attribute_list: *const i32,
) -> i32 {
    Debug::ensure_init();

    Debug::get_mut().debug_callback = Some(callback);

    0
}
