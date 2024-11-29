#include "stdarg.h"

void syslog(const char* buf, ...) {
    va_list va_args;
    va_start(va_args,buf);
    log("OBCOBCID: ", va_args);
    log(buf, va_args);
}