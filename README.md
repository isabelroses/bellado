# Bellado

## About
A fast and simple to-do list manager written in rust

## Usage
`bellado [COMMAND]`

commands | args | description 
---------|------|-------------
`init` | | Creates the required files
`add`, `-a`, `--add` | description of the task wrapped in `"`, `-c` can be used to set categories | Create a new task
`list`, `-l`, `--list` | `-a` show all, `-c` show complete, `-s` search by categories | List out tasks
`completed`, `-c`, `--completed` | ID of the task(s) | Mark task(s) as completed
`uncomplete`, `-u`, `--uncomplete` | ID of the task(s) | Mark task(s) as uncompleted
`delete`, `-d`, `--delete` | ID of the task(s) | Delete task(s)
`edit`, `-e`, `--edit` | ID of the task, new description of the task wrapped in "" | Edit task
`clear`, `-C`, `--clear` | | Clear all tasks
`help`, `-h`. `--help` | | Print this message or the help of the given subcommand(s)

### Examples

`bellado -la` this example will list out all the tasks with full details

`bellado -c 1 2 3` this example will mark items with the ID with `1` `2` and `3`

`bellado -d 1 3` this example will delete items with the ID with `1` and `3`

`bellado -e 2 "test"` this example will edit items with the ID with `2` and change the description to `test`

`bellado -lcs "cat"` this example will list out all tasks that contain the category `cat`
