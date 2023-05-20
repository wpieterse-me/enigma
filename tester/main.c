#include <stdint.h>
#include <stdio.h>

#include <EGL/egl.h>

int32_t main(int32_t argument_count, char **arguments)
{
    printf("0x%x\n", eglGetError());

    return 0;
}
