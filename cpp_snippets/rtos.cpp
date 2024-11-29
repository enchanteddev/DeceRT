#include "stdio.h"
#include <unistd.h>
#include <stdarg.h>


using namespace std;
char* sensor_names[] = {SENSOR_NAMES};
char* port_names[] = {PORT_NAMES};
// char* sensor_names[] = {};
// char* port_names[] = {};

void scheduler();

int main() {
    scheduler();
}

void log(const char* buf, ...) {
    va_list va_args;
    va_start(va_args,buf);
    vprintf(buf, va_args);
    // cout << buf;
}

void runTask(void (*f)(void*), void* args, int cycles) {
    printf( "Running for : %d cycles.\n", cycles);
    f(args);
}

void delay(int cycles) {
    printf( "Delaying for : %d cycles.\n", cycles);
    sleep(cycles);
}


void port_read(int port_id, char* buf, int size){
    printf("Reading from port:%s, size:%d.\n ", port_names[port_id], size); 
}

void port_write(int port_id, char* buf, int size){
    printf("Writing from port:%s size:%d.\n", port_names[port_id], size); 
}

void sensor_write(int sensor_id, char* buf, int size){
    printf("Writing from sensor :%s size:%d.\n", sensor_names[sensor_id], size); 
}

void sensor_read(int sensor_id, char* buf, int size){
    printf("Reading from sensor :%s size:%d.\n", sensor_names[sensor_id], size); 
}
