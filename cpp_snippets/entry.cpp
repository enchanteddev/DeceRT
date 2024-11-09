#include "ports.hpp"
#include "rtos.hpp"
#include "entry.hpp"

{PREFIX}

void scheduler() {
    {INITS}
    while (1) {
        {TASKS}
    }
}