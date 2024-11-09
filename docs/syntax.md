# Syntax and file formats
## Format for `sensor.json`
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

## Syntax `task.conf`

<> :: description
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