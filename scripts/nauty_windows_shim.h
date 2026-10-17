#ifndef NAUTY_WINDOWS_SHIM_H
#define NAUTY_WINDOWS_SHIM_H

#include <stdio.h>

#define AVOID_SYS_WAIT_H 1
#define putc_unlocked(c,f) putc(c,f)
#define getc_unlocked(f) getc(f)
#define flockfile(x) ((void)0)
#define funlockfile(x) ((void)0)

#endif
