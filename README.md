## Reference checking program

## Abstract

This is not usable by any means. This is merely a chunk of pseudocode at a current status.

Rif checks corelation between files and decide whether the files are stale or fresh. You can use this program/library when you need to make sure all files are up to date while such files refer multiple other files.

This is a project derived from my project called gesign. Gesign is a independent editor thus not so versatile and somewhat clunky. On the other side, rif aims to make file references check easily attachable and cross platform by default.

## How corelation tree works

Let's say a file is a node. When a single node is added to corelatio tree it checks whether it already exists. There are multiple diversions according to an existence of the file. 

### Node already exists

1. All referencing nodes exist

Do nothing and continue iteration. -> While this was different in gesign... I'm not sure why I did so. Well time will tell.

2. No referencing nodes exist

Create all referencing nodes and and set the nodes' level to parent nodes' level - 1.

3. Some referencing node exists

Create missing reference nodes and set the nodes' level to parent nodes' level - 1. Also recursively increase nodes' level by 1 starting from parent node until reaching top node.

### Node doesn't exist

1. All referencing nodes exist

Crate a node and set the node's level to child node's level + 1 where the node has the highest level.

2. No referencing nodes exist

Create a node and set the node's level to default. Create all referencing nodes and set the nodes' level to parent node's level - 1 .

3. Some referencing node exists

Follow the first case but also creates non-existing nodes and set the nodes' level to parent node's level - 1.

#### How checks work

Corelation tree's keys are sorted by node levels in ascending order and converted int to single vector which is called level sorted vector.

Checker checks currently selected file's timestamp and references and set a file status. Final result is applied to an original struct's data.

## TODO

Proceed with tests 

* [x] Create desired format of toml file
* [x] Enable creation of structure from .rif file  
* [ ] Enable multiple operations
	* [x] Add file to rifList
	* [x] Add references to a file
	* [x] Delete file from rifList
	* [x] Change status of a file manually
	* [x] Update file's status according to references' corelation
		* [x] Construct file corelation tree
		Currently this looks good but not so sure 
		* [ ] Should check sanity on every opeartion.
		No direct self referencing
		No indirect self referencing
	* [ ] Save updated rif list into file
* [ ] Make cli interface
	* [ ] Add file: Add file into .rif file
	* [ ] Sanity check: Whether file exists or not 
	* [ ] Update: Update file time check references
	* [ ] Discard: Discard change and only timestamp without affecting references.
	* [ ] List: List .rif file contents into standard out descriptor
	* [ ] Get as json: List but as json format
* [ ] Create vs code extension to integrate rif to any other projects.
