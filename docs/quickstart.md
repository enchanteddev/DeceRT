# Quickstart

### Step 1
```bash
cargo r -r create-project exampleapp
```
this creates an "exampleapp" project and sensor.json in it
sensor.json:
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
### Step 2
```bash
cargo r -r add-obc 1
cargo r -r add-obc 2
```
creates 2 folders for each obc which contains a task.conf

### Step 3

updated task.conf for obc1:
```
IN: port2
OUT: port1

INIT_CONDITIONS:

Task task1(TEMP): 2
@satisfies: temperature
```

updated task.conf for obc2:

```
IN: port1
OUT: port2

INIT_CONDITIONS:

Task task1(TEMP): 2
@satisfies: temperature

@requires: temperature
Task task2(RELAY): 3
@satisfies: sent

Task task3(): 4

```

### Step 4
```bash
cd obc1
cargo r -r update-tasks
cd ..
cd obc2
cargo r -r update-tasks
```

This should now create some cpp files inside the `/entry/` folder. Do not add any other file in this folder.

You can edit these files to add your own program logic.

Also you can optionally use the `/lib/` folder to add some functions or classes that you might want to use in your project.

### Step 5
This must run from the main directory of the project 
```bash
cargo r -r compile
```

This will compile all your files, schedules the tasks and generate an object file `obc<id>.o` in `/obc<id>/dist/` folder.

