## Syntax

### sensor.json
Syntax:
```json
{
    "sensors": [
        {
            "name": <name>,
            "from": <starting memory address where this sensor is mapped>,
            "to": <ending memory address where this sensor is mapped>,
        }
    ],
    "ports": [
        <portName1>, <portName2>
    ]
}
```
Example:
```json
{
    "sensors": [
        {
            "name": "TEMP",
            "from": "1234",
            "to": "1234"
        },
        {
            "name": "RELAY",
            "from": "1234",
            "to": "1234"
        }
    ],
    "ports": [
        "port1", "port2"
    ]
}
```

### task.conf
Syntax for task.conf

<> :: actual description
```sql
IN: <list of input ports name separated by comma. Eg: IN: port1, port3, port4>
OUT: <list of output ports name separated by comma. Eg: OUT: port1, port3, port4>

INIT_CONDITIONS: <list of conditions that must be satisfied at start of loop. This feature could be used if the first tasks depends on something that is satisfied at end of loop, which In turn depends on prior tasks. Hence this helps in breaking deadlocks to start the infinite loop>

@requires: <list of conditions which are prerequisite to run this task>
Task <taskName>(<sensors used by this task. This info is used by scheduler to schedule>): <number of cycles this task must run>
@satisfies: <list of conditions which are satisfied upon *completion of this task*. It is important to note by *completion of the task*, we consider this task completed after it had ran it's alloted cycles >
```
NOTE: @requires and @satisfies are ONLY OPTIONAL. Means if there are no requirement then user should not write `@requires:` before `Task`

```bash
IN: port2
OUT: port1

INIT_CONDITIONS: relayed

@requires: relayed
Task task1(TEMP): 2
@satisfies: temperature

@requires: temperature
Task task2(RELAY): 7
@satisfies: relayed

Task task1(RELAY): 10
```


## APIS

```bash
create-project <name>
    -   creates a new directory with the name and adds sensor.json
```

```bash
add-obc <id>
    -   Creates a new folder called `obc<id>`.
    -   Creates a new folder called `obc<id>/entry`
    -   Creates a new folder called `obc<id>/lib
    -   Creates a new file `obc<id>/ports.hpp`
    -   Adds `tasks.conf` inside of `obc<id>`
```

```bash
update-tasks
    -   parse sensor.json
    -   parse tasks.conf
```

```bash
compile
    -   runs update-tasks for each obc
    -   for each obc, add required header files to the obc<id> folder
    -   runs the scheduler
    -   creates the port .cpp which consist of implementation for the ports and the sensors mentioned in sensors.json
    -   compiles all the files to create obc<id>.o
