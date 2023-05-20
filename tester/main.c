#include <stdint.h>
#include <stdio.h>

#include <EGL/egl.h>
#include <GL/gl.h>

static const EGLint configAttribs[] = {
    EGL_SURFACE_TYPE, EGL_PBUFFER_BIT,
    EGL_BLUE_SIZE, 8,
    EGL_GREEN_SIZE, 8,
    EGL_RED_SIZE, 8,
    EGL_DEPTH_SIZE, 8,
    EGL_RENDERABLE_TYPE, EGL_OPENGL_BIT,
    EGL_NONE};

static const int pbufferWidth = 9;
static const int pbufferHeight = 9;

static const EGLint pbufferAttribs[] = {
    EGL_WIDTH,
    pbufferWidth,
    EGL_HEIGHT,
    pbufferHeight,
    EGL_NONE,
};

int32_t main(int32_t argument_count, char **arguments)
{
    // 1. Initialize EGL
    EGLDisplay eglDpy = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    if (eglDpy == NULL)
    {
        printf("Failed to get display\n");
        return 1;
    }

    EGLint major = 0;
    EGLint minor = 0;

    eglInitialize(eglDpy, &major, &minor);

    printf("Version %d.%d\n", major, minor);

    // 2. Select an appropriate configuration
    EGLint numConfigs;
    EGLConfig eglCfg;

    eglChooseConfig(eglDpy, configAttribs, &eglCfg, 1, &numConfigs);

    // 3. Create a surface
    EGLSurface eglSurf = eglCreatePbufferSurface(eglDpy, eglCfg,
                                                 pbufferAttribs);

    // 4. Bind the API
    eglBindAPI(EGL_OPENGL_API);

    // 5. Create a context and make it current
    EGLContext eglCtx = eglCreateContext(eglDpy, eglCfg, EGL_NO_CONTEXT, NULL);

    eglMakeCurrent(eglDpy, eglSurf, eglSurf, eglCtx);

    // from now on use your OpenGL context

    // 6. Terminate EGL when finished
    eglTerminate(eglDpy);

    printf("\n!!! DONE !!!\n");

    return 0;
}
