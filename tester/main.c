#include <stdint.h>
#include <stdio.h>

#include <EGL/egl.h>
#include <GL/gl.h>

int32_t main(int32_t argument_count, char **arguments)
{
    printf("0x%x\n", eglGetError());
    printf("0x%x\n", glGetError());

    return 0;
}
