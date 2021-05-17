## Reference checking program

## Abstract

~~This is not usable by any means. This is merely a chunk of pseudocode at a current status.~~
I completed all the features that was planned. However I haven't tested enought to ensure this program works as intended. I'm currently dog fooding this program and fixing known bugs. If you want to use stable version, then you may have to wait.

Rif checks corelation between files and decide whether the files are stale or fresh. You can use this program or library( to be implemented ) when you need to make sure all files are up to date while the files refer multiple other files.

This is a project derived from my project called gesign. Gesign is a independent editor thus not so versatile and somewhat clunky. On the other side, rif aims to make file references check easily attachable and cross platform by default.

## Usage

```bash

rif new

rif status

rif add <FILE>

rif remove <FILE>

rif set <FILE> <REFS>

rif unset <FILE> <REFS>

rif discard <FILE>

rif update <FILE>

rif check

rif sanity

```

## How it works

To be updated

## Demo

To be updated

[Todos and Known Bugs](meta.md)
