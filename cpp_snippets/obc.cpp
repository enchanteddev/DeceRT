#include "ports.hpp"
#include "rtos.hpp"

void scheduler() {
    while (1) {
        __TASKS__
    }
}