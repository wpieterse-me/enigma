#[repr(i32)]
pub enum ErrorCode {
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

#[no_mangle]
pub extern "C" fn eglGetError() -> ErrorCode {
    ErrorCode::Success
}
