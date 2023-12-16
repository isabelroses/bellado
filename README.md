# Bellado

## About
A fast and simple to-do list manager written in rust

<details><summary>

## Install
</summary>

### Arch Linux

```bash
paru -S bellado-git
```

### Nix

#### Try it out

```bash
nix run github:isabelroses/bellado
```
#### Using the home-manager module, with flakes

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    bellado.url = "github:isabelroses/bellado";
  };

  outputs = { self, nixpkgs, bellado }: {
    homeConfigurations."user@hostname" = home-manager.lib.homeManagerConfiguration {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;

      modules = [
        bellado.homeManagerModules.default
        {
          programs.bellado = {
            enable = true;
            enableAliases = true;
          };
        }
        # ...
      ];
    };
  }
}
```

Don't forget you can use my cachix cache, see below for an example configuration
For more information see [here](https://app.cachix.org/cache/isabelroses)

```nix
{
  nix.settings = {
    substituters = [ "https://isabelroses.cachix.org" ];
    trusted-public-keys = [ "isabelroses.cachix.org-1:mXdV/CMcPDaiTmkQ7/4+MzChpOe6Cb97njKmBQQmLPM=" ];
  };
}
```


### Build from source

```bash
cargo build --release --bin=bellado
```

</details>

## Usage
`bellado [COMMAND]`

commands | args | description 
---------|------|-------------
`init` | `-g` `git` create with support for git version control | Creates the required files
`add`, `-a`, `--add` | description of the task wrapped in `"`, `-c` can be used to set categories | Create a new task
`list`, `-l`, `--list` | `-a` show all, `-c` show complete, `-s` search by categories, `-t` `--table` output as a table, `--header` print with table headers | List out tasks
`completed`, `-c`, `--completed` | ID of the task(s) | Mark task(s) as completed
`uncomplete`, `-u`, `--uncomplete` | ID of the task(s) | Mark task(s) as uncompleted
`edit`, `-e`, `--edit` | ID of the task, new description of the task wrapped in "" | Edit task
`delete`, `-d`, `--delete` | ID of the task(s) | Delete task(s)
`clear`, `-C`, `--clear` | | Clear all tasks
`git`, `-g`, `--git` | `-i` `--init` git init, `-p` `--push` push all changes, `-P` `--pull` pull all changes | Git version control
`export`, `-x`, `--export` | `-m` `--markdown` markdown, `-mc` `--markdown --categories` markdown with catagories, `-j` `--json` json, `-jp` `--json --pretty` json as a pretty output | Export the tasks to a stdout, in diffrent formats
`help`, `-h`. `--help` | | Print this message or the help of the given subcommand(s)

### Examples

`bellado -la` this example will list out all the tasks with full details

`bellado -c 1 2 3` this example will mark items with the ID with `1` `2` and `3`

`bellado -d 1 3` this example will delete items with the ID with `1` and `3`

`bellado -e 2 "test"` this example will edit items with the ID with `2` and change the description to `test`

`bellado -lcs "cat"` this example will list out all tasks that contain the category `cat`

`bellado -xj` this example will export all tasks as json to stdout 

`bellado -xmc` this example will export all tasks as markdown with categories to stdout (useful for github issues, and obsidan notes, etc)

### Tips

#### aliases

This consist of some of the main commands that I use, you can add them to your `.bashrc` or `.zshrc` file

```bash
bel = "bellado";
bell = "bellado -l";
bella = "bellado -la";
bellc = "bellado -lc";
```

#### quickly export tasks to a file

```bash
bellado -xmc > ~/vault/todo.md
```