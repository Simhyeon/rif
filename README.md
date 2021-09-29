## Rif, impact control program

### Caution

I completed all the features that was planned. However I haven't tested enough
to ensure this program works as intended. I'm currently dog fooding this
program and fixing known bugs. If you want to use stable version, then you may
have to wait.

### About rif

Rif checks corelation between files and decide whether the files are stale or
fresh. You can use this program or library(yet to come) when you need
to make sure all files are up to date when files refer multiple other
files.

This is a project derived from my project called gesign. Gesign was a
independent editor thus not so versatile and somewhat clunky to use with other
programs. On the other side, rif aims to make a file references checking easily
attachable and cross platform by default.

### Some use cases

Rif aims to help designers to track document changes. Especially when the
documents are highly modular and interconnected. A generic usage is game design
documents.

Designer can add a file to rif project and set references(children) to the
file(parent). Whenever any child file changes, the parent file's status also
changes. This process is manually checked by rif binary(at least for now).

For example, 

1. Create a new file called levelmanager.md and added it into the rif project
1. Set "level.md" as a reference of "levelmanager.md"
1. Update level.md's content
1. Levelmanager.md's status gets updated to stale
1. Update levelmanager.md's content
1. Levelmanager.md's status gets updated to "up to date"

In this case levelmanager depends on the level because level's change can
affect a behaviour of level manager. Thus change of level's content makes level
manager's status to stale which informs a designer to manually reassure if level
manager's content should be updated or not. After designer applys proper
modification, levelmanager's status gets updated. In this way, designer can
minimize logical errors derived from unnoticed file relationships.

### General Usage

**Binary**
```bash
# Initiate rif project with default .rifginoe file
rif init

# Add all files in current project directory
rif add . 

# Update a file with update message
rif commit <FILE> -m "This is important update"

# Show status of rif directory
rif status

# Show whole rif tree 
rif ls
```

**Library**

```toml
rif = "0.1.0"
```

```rust
use rif::{Rif, LisType};

// Every operation saves file to .rif which is created with new method
let rif = Rif::new();
let wd = Some(path::new("dir")); // Or use "None" for cwd
rif.new(wd);
rif.add(Path::new("file_to_add.txt", false));
rif.commit(None);
rif.list(None,LisType::All,None);
```

Not yet

### Config

You can set several config options. I'm planning to add more config options. Config file is located inc "$PWD/.rif/config".

- hook 
	-trigger: Whether trigger hook process after check command
    -hook command : Process name to trigger
	-hook argument : Argument type that should be passed to process. It should be one of among "All, None, Fresh, Stale"

[Example](./docs/config_example)

### Install method

```bash
# Simple binary file
cargo install rif --features binary

# With color prompt
cargo install rif --features binary,color
```

### Build method

Make sure rust langauge is installed. [Link](https://www.rust-lang.org/tools/install)
```
# Clone the repo
git clone https://github.com/simhyeon/rif

# And build with cargo, compiled binary is located in target/release
cd rif && cargo build --release
```
