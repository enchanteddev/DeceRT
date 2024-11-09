# DeceRT
DeceRT is a framework for building decentralised real-time systems. It incorporates a static task scheduler, that ensures that a sensor is only used by one On-board Computer (OBC) at a time. The tasks can also have dependencies on other tasks.

# Usage
Ensure you have the rust toolchain installed.

Clone this repository and run `cargo r -r` to run the DeceRT CLI.

You can also run `cargo b -r` to build the DeceRT CLI, and use the executable in the `target/release` folder directly.

# Docs

1. [Quickstart Guide](./docs/quickstart.md)
2. [Syntax of .conf and format of sensors.json](./docs/syntax.md)
3. [CLI Usage](#CLI-Usage)

## CLI Usage

### create-project
- Creates a new directory with the name and adds sensor.json
```bash
decert create-project <name>
```

### add-obc
- Creates a new folder called `obc<id>`.
- Creates a new folder called `obc<id>/entry`
- Creates a new folder called `obc<id>/lib
- Creates a new file `obc<id>/ports.hpp`
- Adds `tasks.conf` inside of `obc<id>`

```bash
decert add-obc <id>
```

### update-tasks
- Parse sensor.json
- Parse tasks.conf

```bash
decert update-tasks
```

### compile
- runs update-tasks for each obc
- for each obc, add required header files to the obc<id> folder
- runs the scheduler
- creates the port .cpp which consist of implementation for the ports and the sensors mentioned in sensors.json
- compiles all the files to create obc<id>.o

```bash
decert compile
```

**Note**: The `decert` command can be replaced with `cargo r -r` to run the Decert CLI, when using directly from the repo. 

Example: `cargo r -r create-project <name>`