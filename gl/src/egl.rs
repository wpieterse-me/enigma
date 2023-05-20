use std::ffi::{c_char, c_void};

type EGLBoolean = u32;
type EGLInteger = u32;

#[repr(i32)]
pub enum EGLErrorCode {
    /// The operation has succeeded
    Success = 0x3000,

    /// EGL is not initialized, or could not be initialized, for the specified display
    NotInitialized = 0x3001,

    /// EGL cannot access a requested resource (for example, a context is bound in another thread)
    BadAccess = 0x3002,

    /// EGL failed to allocate resources for the requested operation
    BadAllocation = 0x3003,

    /// An unrecognized attribute or attribute value was passed in an attribute list
    BadAttribute = 0x3004,

    /// An EGLConfig argument does not name a valid EGLConfig
    BadConfiguration = 0x3005,

    /// An EGLContext argument does not name a valid EGLContext
    BadContext = 0x3006,

    /// The current surface of the calling thread is a window, pixel buffer, or pixel map that is no longer valid
    BadCurrentSurface = 0x3007,

    /// An EGLDisplay argument does not name a valid EGLDisplay; or, EGL is not initialized on the specified EGLDisplay
    BadDisplay = 0x3008,

    /// Arguments are inconsistent; for example, an otherwise valid context requires buffers (e.g. depth or stencil) not allocated by an otherwise valid surface
    BadMatch = 0x3009,

    /// A NativePixmapType argument does not refer to a valid native pixmap
    BadNativePixelMap = 0x300A,

    /// A NativeWindowType argument does not refer to a valid native window
    BadNativeWindow = 0x300B,

    /// One or more argument values are invalid
    BadParameter = 0x300C,

    /// An EGLSurface argument does not name a valid surface (window, pbuffer, or pixmap) configured for OpenGL ES rendering
    BadSurface = 0x300D,
}

#[repr(transparent)]
pub struct EGLDisplayID(*mut c_char);

#[repr(transparent)]
pub struct EGLQueryString(*const c_char);

#[repr(transparent)]
pub struct EGLDisplay(*mut c_void);

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
    EGLErrorCode::Success
}

#[no_mangle]
pub extern "C" fn eglGetDisplay(_display_id: EGLDisplayID) -> EGLDisplay {
    EGLDisplay(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglInitialize(
    _display: EGLDisplay,
    _major_version: *mut EGLInteger,
    _minor_version: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglTerminate(_display: EGLDisplay) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglQueryString(_display: EGLDisplay, _name: EGLInteger) -> EGLQueryString {
    EGLQueryString(std::ptr::null())
}

#[no_mangle]
pub extern "C" fn eglGetConfigs(
    _display: EGLDisplay,
    _configurations: *mut EGLConfiguration,
    _configuration_size: EGLInteger,
    _configuration_count: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglChooseConfig(
    _display: EGLDisplay,
    _attribute_list: *const EGLInteger,
    _configurations: *const EGLConfiguration,
    _configuration_size: EGLInteger,
    _configuration_count: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglGetConfigAttrib(
    _display: EGLDisplay,
    _configuration: EGLConfiguration,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglCreateWindowSurface(
    _display: EGLDisplay,
    _configuration: EGLConfiguration,
    _window: EGLNativeWindow,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferSurface(
    _display: EGLDisplay,
    _configuration: EGLConfiguration,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglCreatePixmapSurface(
    _display: EGLDisplay,
    _configuration: EGLConfiguration,
    _pixel_map: EGLNativePixelMap,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglDestroySurface(_display: EGLDisplay, _surface: EGLSurface) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglQuerySurface(
    _display: EGLDisplay,
    _surface: EGLSurface,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglBindAPI(_api: u32) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglQueryAPI() -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn eglWaitClient() -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglReleaseThread() -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglCreatePbufferFromClientBuffer(
    _display: EGLDisplay,
    _buffer_type: u32,
    _buffer: EGLClientBuffer,
    _configuration: EGLConfiguration,
    _attribute_list: *const EGLInteger,
) -> EGLSurface {
    EGLSurface(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglSurfaceAttrib(
    _display: EGLDisplay,
    _surface: EGLSurface,
    _attribute: EGLInteger,
    _value: EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglBindTexImage(
    _display: EGLDisplay,
    _surface: EGLSurface,
    _buffer: EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglReleaseTexImage(
    _display: EGLDisplay,
    _surface: EGLSurface,
    _buffer: EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglSwapInterval(_display: EGLDisplay, _interval: EGLInteger) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglCreateContext(
    _display: EGLDisplay,
    _configuration: EGLConfiguration,
    _share_context: EGLContext,
    _attribute_list: *const EGLInteger,
) -> EGLContext {
    EGLContext(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglDestroyContext(_display: EGLDisplay, _context: EGLContext) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglMakeCurrent(
    _display: EGLDisplay,
    _write_surface: EGLSurface,
    _read_surface: EGLSurface,
    _context: EGLContext,
) -> EGLBoolean {
    0
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
pub extern "C" fn eglGetCurrentDisplay() -> EGLDisplay {
    EGLDisplay(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn eglQueryContext(
    _display: EGLDisplay,
    _context: EGLContext,
    _attribute: EGLInteger,
    _value: *mut EGLInteger,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglWaitGL() -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglWaitNative(_engine: EGLInteger) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglSwapBuffers(_display: EGLDisplay, _surface: EGLSurface) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglCopyBuffers(
    _display: EGLDisplay,
    _surface: EGLSurface,
    _target: EGLNativePixelMap,
) -> EGLBoolean {
    0
}

#[no_mangle]
pub extern "C" fn eglGetProcAddress(_procedure: *const c_char) -> *const c_void {
    std::ptr::null()
}
