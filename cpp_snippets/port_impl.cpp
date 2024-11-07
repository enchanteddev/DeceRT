class {NAME} {
private:
    {NAME}(){}
public:
    static void read(char *buffer, int size){
        r_read({ID}, buffer, size);
    }
    static void write(char *buffer, int size){
        r_write("{NAME}", buffer, size);
    }
};