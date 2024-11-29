void syslog(char* buf, ...) {
    va_list va_args;
    va_start(va_args,buf);
    log("OBC{OBCID}", va_args);
    log(buf, va_args);
}