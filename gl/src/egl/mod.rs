use std::ffi::{c_char, c_void, CStr};

use self::extensions::egl_khr_debug::{post_debug_msg, EGLLabelKHR, EGLDebugMessageTypeKHR};

pub mod extensions;

type EGLInteger = u32;

type EGLGeneralFn = unsafe extern "C" fn();

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
    /// # Panics
    /// Panics if `self.0` == null.
    pub unsafe fn as_display(&self) -> &'static mut EGLDisplay {
        let ptr = self.0 as *mut EGLDisplay;

        ptr.as_mut().unwrap()
    }

    /// # Safety
    /// `self.0` != null.
    pub unsafe fn into_display(self) -> Box<EGLDisplay> {
        let ptr = self.0 as *mut EGLDisplay;

        Box::from_raw(ptr)
    }

    pub fn from_display(display: EGLDisplay) -> Self {
        let reference = Box::leak(Box::new(display));
        let ptr = reference as *mut EGLDisplay;

        Self(ptr as _)
    }
}

pub struct EGLDisplay {
    initialized: bool,
}

impl EGLDisplay {
    pub fn new() -> EGLDisplay {
        EGLDisplay { initialized: false }
    }
}

impl Drop for EGLDisplay {
    fn drop(&mut self) {
        println!("Dropping display");
    }
}

#[repr(transparent)]
pub struct EGLFunctionName(*const c_char);

#[no_mangle]
pub unsafe extern "C" fn eglGetProcAddress(function_name: EGLFunctionName) -> EGLGeneralFn {
    if function_name.0.is_null() {
        std::mem::transmute::<*const (), EGLGeneralFn>(std::ptr::null())
    } else {
        let c_string = CStr::from_ptr(function_name.0);
        let rust_string = match c_string.to_str() {
            Ok(s) => s,
            Err(_) => return std::mem::transmute::<*const (), EGLGeneralFn>(std::ptr::null()),
        };

        let pointer = match rust_string {
            "eglGetPlatformDisplay" => eglGetPlatformDisplay as *const (),
            _ => std::ptr::null(),
        };

        std::mem::transmute::<*const (), EGLGeneralFn>(pointer)
    }
}

#[no_mangle]
pub extern "C" fn eglGetPlatformDisplay(
    _platform: i32,
    _native_display: *const c_void,
    _attribute_list: *const i32,
) -> EGLDisplayHandle {
    EGLDisplayHandle(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglGetDisplay(display_id: EGLDisplayID) -> EGLDisplayHandle {
    if display_id.0.is_null() {
        let display = EGLDisplay::new();

        EGLDisplayHandle::from_display(display)
    } else {
        EGLDisplayHandle(std::ptr::null_mut())
    }
}

#[no_mangle]
pub unsafe extern "C" fn eglInitialize(
    display_handle: EGLDisplayHandle,
    major_version: *mut EGLInteger,
    minor_version: *mut EGLInteger,
) -> EGLBoolean {
    if display_handle.0.is_null() {
        EGLBoolean::False
    } else {
        let display = display_handle.as_display();

        display.initialized = true;

        if major_version.is_null() == false {
            *major_version = 1;
        }

        if minor_version.is_null() == false {
            *minor_version = 0;
        }

        EGLBoolean::True
    }
}

#[no_mangle]
pub unsafe extern "C" fn eglTerminate(display_handle: EGLDisplayHandle) -> EGLBoolean {
    if display_handle.0.is_null() {
        EGLBoolean::False
    } else {
        display_handle.into_display();

        EGLBoolean::True
    }
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
