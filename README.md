## Reference checking program

## Abstract

This is not usable by any means. This is merely a chunk of pseudocode at a current status.

Rif checks corelation between files and decide whether the files are stale or fresh. You can use this program/library when you need to make sure all files are up to date while such files refer multiple other files.

This is a project derived from my project called gesign. Gesign is a independent editor thus not so versatile and somewhat clunky. On the other side, rif aims to make file references check easily attachable and cross platform by default.

## TODO

Proceed with tests 

* [x] Create desired format of toml file
* [x] Enable creation of structure from .rif file  
* [ ] Enable multiple operations
	* [x] Add file to rifList
	* [x] Add references to a file
	* [x] Delete file from rifList
	* [x] Change status of a file manually
	* [ ] Update file's status according to references' corelation
		* [ ] Construct file corelation tree
* [ ] Make cli interface
	* [ ] Add file: Add file into .rif file
	* [ ] Sanity check: Whether file exists or not 
	* [ ] Update: Update file time check references
	* [ ] Discard: Discard change and only timestamp without affecting references.
	* [ ] List: List .rif file contents into standard out descriptor
	* [ ] Get as json: List but as json format
* [ ] Create vs code extension to integrate rif to any other projects.
