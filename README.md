create-project <name>
    *   create-project.rs  
        -   creates a new directory with the name and adds sensor.json
    
add-obc <id>
    * add-obc.rs
        -   Creates a new folder called `obc<id>`.
        -   Creates a new folder called `obc<id>/entry`
        -   Creates a new folder called `obc<id>/lib
        -   Creates a new file `obc<id>/ports.hpp`
        -   Adds `tasks.conf` inside of `obc<id>`

update-tasks
    * update-task.rs
        - parse sensor.json
        - parse tasks.conf

