## Reference checking program

## Abstract

~~This is not usable by any means. This is merely a chunk of pseudocode at a current status.~~
I completed all the features that was planned. However I haven't tested enough
to ensure this program works as intended. I'm currently dog fooding this
program and fixing known bugs. If you want to use stable version, then you may
have to wait.

Rif checks corelation between files and decide whether the files are stale or
fresh. You can use this program or library(yet to come) when you need
to make sure all files are up to date while the files refer multiple other
files.

This is a project derived from my project called gesign. Gesign is a
independent editor thus not so versatile and somewhat clunky to use with other programs. On the other
side, rif aims to make file references check easily attachable and cross
platform by default.

## Usage

**Basics**

```bash

# Create new .rif file in current working directory
rif new

# Show current status of the rif file
rif status 
	-i --ignore : Ignore untracked files
	-v --verbose : Alsy display list output

# Show item list of registered file
# Optionaly give file to show only the file into the list
rif list <FILE>(optional)

# Add file to rif
# <FILE> can be any bash glob pattern e.g "."(whole project), "*"(whole files in current directory)
rif add <FILE>
	-s --set <REFS> : Add references to the <FILE> after addition. <REFS> should already exists in rif
	-b --batch <REFS> : Same with set but enable batch setting to mutliple <FILE>. This explicitly enquire a user because unsetting is very trival process while setting references is instant.

# Remove file from rif
# remove doesn't process directory input because it is very prone to unintended operation
# you cannot revert a removal 
rif remove <FILE>

# Set references to a file 
# references should exist in rif
rif set <FILE> <REFS>

# Unset references to a file 
# references do not have to be valid path
# becuase unset operation is technically "set -"
rif unset <FILE> <REFS>

# Discard file modification
# modification of registered files are always tracked
# this command discard the tracking and pretend as if nothing has changed
rif discard <FILE>

# Update file modification
# update so that rif can use the modified status
# for file corelation checking
rif update <FILE>
	-f --force : force update even if file is not modified
	-c --check : auto check after update

# Check file corelation
rif check
	-u --update : update all modified files before checking

# Checking file format sanity of ".rif"
rif sanity
	-f --fix : Fix invalid format automatically

```

**General Usage**

```bash

# Initiate rif
rif new

# Add all files in current project directory
rif add . 

# Check file corelation after automatcially updating all modified files
rif check -u

# Update a file's tracking status without actually modifying file + 
# check after update is executed
rif update -fc

# Show file status of only the given file
rif list <File Name>

```

## How it works

To be updated

## Demo

To be updated

[Todos and Known Bugs](meta.md)
