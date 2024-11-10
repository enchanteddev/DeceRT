# Syntax and file formats
## Format for `sensor.json`
```
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

## Syntax `task.conf`

Keywords: `IN:`, `OUT:`, `INIT_CONDITIONS:`, `Task`, `@requires:` , `@satisfies:`

### IN:
list of input ports name separated by comma. 

Example: `IN: port1, port3, port4`


### OUT:
list of output ports name separated by comma. 

Example: `OUT: port1, port3, port4`


### INIT_CONDITIONS: 
list of conditions that must be satisfied at start of loop. This feature could be used if the first tasks depends on something that is satisfied at end of loop, which In turn depends on prior tasks. Hence this helps in breaking deadlocks to start the infinite loop

### @requires: 
list of conditions which are prerequisite to run this task. This is used above `Task` declaration.


### Task 
Task is used to declare a task.

Syntax: `Task taskname(args): cycles`

**taskname** : name of the task
<br>
**args**: sensors used by this task. This info is used by scheduler to schedule 
<br>
**cycles**: number of cycles this task must run.


### @satisfies: 
List of conditions which are satisfied upon *completion of this task*.

It is important to note by *completion of the task*, we consider this task completed after it had ran it's alloted cycles. 

This is used below `Task` declaration.


**NOTE**: @requires and @satisfies are optional. It is required to omit them if they are not required.

### Example:
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