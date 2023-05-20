use std::ffi::{c_char, c_void};

type EGLInteger = u32;

#[repr(u32)]
pub enum EGLBoolean {
    False = 0,
    True = 1,
}

#[repr(i32)]
pub enum EGLErrorCode {
    Success = 0x3000,
    NotInitialized = 0x3001,
    BadAccess = 0x3002,
    BadAllocation = 0x3003,
    BadAttribute = 0x3004,
    BadConfiguration = 0x3005,
    BadContext = 0x3006,
    BadCurrentSurface = 0x3007,
    BadDisplay = 0x3008,
    BadMatch = 0x3009,
    BadNativePixelMap = 0x300A,
    BadNativeWindow = 0x300B,
    BadParameter = 0x300C,
    BadSurface = 0x300D,
}

#[repr(transparent)]
pub struct EGLDisplayID(*mut c_void);

#[repr(transparent)]
pub struct EGLQueryStringResponse(*const c_char);

#[repr(i32)]
pub enum EGLQueryStringRequest {
    Vendor = 0x3053,
    Version = 0x3054,
    Extensions = 0x3055,
}

#[repr(transparent)]
pub struct EGLDisplayHandle(*mut c_void);

#[repr(transparent)]
pub struct EGLConfiguration(*mut c_void);

#[repr(transparent)]
pub struct EGLContext(*mut c_void);

#[repr(transparent)]
pub struct EGLSurface(*mut c_void);

#[repr(transparent)]
pub struct EGLClientBuffer(*mut c_void);

#[repr(transparent)]
pub struct EGLNativeWindow(*mut c_void);

#[repr(transparent)]
pub struct EGLNativePixelMap(*mut c_void);

#[no_mangle]
pub extern "C" fn eglGetError() -> EGLErrorCode {
    println!("eglGetError");

    EGLErrorCode::Success
}

impl EGLDisplayHandle {
    pub unsafe fn as_display(&self) -> &'static mut EGLDisplay {
        let ptr = self.0 as *mut EGLDisplay;

        ptr.as_mut().unwrap()
    }

    pub fn from_display(display: EGLDisplay) -> Self {
        let reference = Box::leak(Box::new(display));
        let ptr = reference as *mut EGLDisplay;

        Self(ptr as _)
    }
}

pub struct EGLDisplay {
    bar: String,
}

impl EGLDisplay {
    pub fn new() -> EGLDisplay {
        EGLDisplay {
            bar: "dooda".to_string(),
        }
    }
}

#[no_mangle]
pub extern "C" fn eglGetDisplay(display_id: EGLDisplayID) -> EGLDisplayHandle {
    // This driver only supports the value of EGL_DEFAULT_DISPLAY being passed
    // to in at the moment
    if display_id.0.is_null() {
        // Create a new, blank display object
        let display = EGLDisplay::new();

        // Convert the display into a handle object
        EGLDisplayHandle::from_display(display)
    } else {
        // Return null for any other display identifier, as this is not supported
        // at the moment
        EGLDisplayHandle(std::ptr::null_mut())
    }
}

#[no_mangle]
pub unsafe extern "C" fn eglInitialize(
    display_handle: EGLDisplayHandle,
    _major_version: *mut EGLInteger,
    _minor_version: *mut EGLInteger,
) -> EGLBoolean {
    println!("eglInitialize");

    let display = display_handle.as_display();

    println!("{}", display.bar);

    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglTerminate(_display: EGLDisplayHandle) -> EGLBoolean {
    println!("eglTerminate");

    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglQueryString(
    _display: EGLDisplayHandle,
    _name: EGLQueryStringRequest,
) -> EGLQueryStringResponse {
    EGLQueryStringResponse(std::ptr::null())
}

#[no_mangle]
pub extern "C" fn eglGetConfigs(
    _display: EGLDisplayHandle,
    _configurations: *mut EGLConfiguration,
    _configuration_size: EGLInteger,
    _configuration_count: *mut EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglChooseConfig(
    _display: EGLDisplayHandle,
    _attribute_list: *const EGLInteger,
    _configurations: *const EGLConfiguration,
    _configuration_size: EGLInteger,
    _configuration_count: *mut EGLInteger,
) -> EGLBoolean {
    println!("eglChooseConfig");

    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglGetConfigAttrib(
    _display: EGLDisplayHandle,
    _configuration: EGLConfiguration,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglCreateWindowSurface(
    _display: EGLDisplayHandle,
    _configuration: EGLConfiguration,
    _window: EGLNativeWindow,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferSurface(
    _display: EGLDisplayHandle,
    _configuration: EGLConfiguration,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    println!("eglCreatePbufferSurface");

    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglCreatePixmapSurface(
    _display: EGLDisplayHandle,
    _configuration: EGLConfiguration,
    _pixel_map: EGLNativePixelMap,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglDestroySurface(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglQuerySurface(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglBindAPI(_api: u32) -> EGLBoolean {
    println!("eglBindAPI");

    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglQueryAPI() -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn eglWaitClient() -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglReleaseThread() -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferFromClientBuffer(
    _display: EGLDisplayHandle,
    _buffer_type: u32,
    _buffer: EGLClientBuffer,
    _configuration: EGLConfiguration,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglSurfaceAttrib(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
    _attribute: EGLInteger,
    _value: EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglBindTexImage(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
    _buffer: EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglReleaseTexImage(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
    _buffer: EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglSwapInterval(_display: EGLDisplayHandle, _interval: EGLInteger) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglCreateContext(
    _display: EGLDisplayHandle,
    _configuration: EGLConfiguration,
    _share_context: EGLContext,
    _attribute_list: *const EGLInteger,
) -> EGLContext {
    println!("eglCreateContext");

    EGLContext(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglDestroyContext(
    _display: EGLDisplayHandle,
    _context: EGLContext,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglMakeCurrent(
    _display: EGLDisplayHandle,
    _write_surface: EGLSurface,
    _read_surface: EGLSurface,
    _context: EGLContext,
) -> EGLBoolean {
    println!("eglMakeCurrent");

    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglGetCurrentContext() -> EGLContext {
    EGLContext(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglGetCurrentSurface(_read_draw: EGLInteger) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglGetCurrentDisplay() -> EGLDisplayHandle {
    EGLDisplayHandle(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglQueryContext(
    _display: EGLDisplayHandle,
    _context: EGLContext,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglWaitGL() -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglWaitNative(_engine: EGLInteger) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglSwapBuffers(_display: EGLDisplayHandle, _surface: EGLSurface) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglCopyBuffers(
    _display: EGLDisplayHandle,
    _surface: EGLSurface,
    _target: EGLNativePixelMap,
) -> EGLBoolean {
    EGLBoolean::False
}

#[no_mangle]
pub extern "C" fn eglGetProcAddress(_procedure: *const c_char) -> *const c_void {
    std::ptr::null()
}
