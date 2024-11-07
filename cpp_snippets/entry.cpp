#include "ports.hpp"
#include "rtos.hpp"

{PREFIX}

void scheduler() {
    {INITS}
    while (1) {
        {TASKS}
    }
}