# bellado

## About
This is a CLI todo tool thats simple and fast.

## Usage
`bellado [COMMAND]`

Commands:
  init                          Create the reqired files
  new, -n, --new                Create a new task
  list, -l, --list              List out tasks
  completed, -c, --completed    Mark task(s) as completed
  uncomplete, -u, --uncomplete  Mark task(s) as uncompleted
  help                          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

### Examples

`bellado -la` this example will list out all the tasks with full details

`bellado -c 1 2 3` this example will mark items with the id with `1` `2` and `3`

