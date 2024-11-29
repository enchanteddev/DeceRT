class {NAME} {
private:
    static {NAME}* instance;
    int startAddress;
    int endAddress;
    int id;
    {NAME}() {
        startAddress = {ST};
        endAddress = {ET};
        id= {ID};
    }
public:
    static {NAME}* get_instance() {
        if (instance == nullptr) {
            instance = new {NAME}();
        }
        return instance;
    } 
    void read(char *buffer, int size){
        sensor_read(id, buffer, size);
    }
    void write(char *buffer, int size){
        sensor_write(id, buffer, size);
    }
};