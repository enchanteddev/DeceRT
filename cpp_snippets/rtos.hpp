void runTask(void (*f)(void**), void* , int);

void delay(int);

void port_read(int, char*, int);

void port_write(int, char*, int);

void sensor_read(int, char*, int);

void sensor_write(int, char*, int);