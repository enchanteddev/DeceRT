class {NAME} {
private:
    {NAME}(){}
public:
    static void read(char *buffer, int size){
        port_read({ID}, buffer, size);
    }
    static void write(char *buffer, int size){
        port_write({ID}, buffer, size);
    }
};