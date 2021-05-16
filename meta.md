## TODO

### Imminent

* [x] Create desired format of toml file
* [x] Enable creation of structure from .rif file  
* [x] Enable multiple operations
	* [x] Add file to rifList
	* [x] Add references to a file
	* [x] Delete file from rifList
	* [x] Change status of a file manually
	* [x] Update file's status according to references' corelation
		* [x] Construct file corelation tree
		Currently this looks good but not so sure 
		* [x] Should check sanity on every opeartion.
		* [x] Sanity check should also check file existences
	* [x] Save updated rif list into file
	* [x] Remove must delete all occurences not only just a singlefile
		* [x] Refactor SingleFile structure to use set for references so that addition and subtraction are easy to use.
	Because finding all file references and fixing is stupid I guess?
* [x] Enable tracking of files
	* [x] Draft
	* [x] Move tracker logics into rif_list models rather than separate file
	* [x] Test 
* [x] Enable clap integration for cli
	* [x] Parse sub commands
     <!-- Add, Check, Discard, List, New, Remove, SanityCheck, Update, Set, Unset, Status -->
	* [x] Add file: Add file into .rif file
	* [x] Check: check file references with rif file in cwd.
	* [x] Discard: Discard change and only timestamp without affecting references.
	This technically update filestamp stored in rif_time
	* [x] List: List .rif file contents into standard out descriptor
	* [x] New: create new rif and rif_time files in current working directory(cwd).
	* [x] Remove : remove file from .rif file
	* [x] Sanity check: Whether file exists or not 
	* [x] Update: Update file time check references
	* [x] Set: Set file's references
		* [x] Set can either be plus or minus
	* [x] Status : Show tracking position
	* [x] Test 
	Tested -> Add, remove, set, unset, list, new, update, status, discard,
	Fails -> Sanity: panics on self referencing
* [x] Improve functionality
	* [x] Sanity check with auto fix option
	* [x] Check should yield changed files list
	* [x] Make force update method which doesn't change file contents but only update filestamp 
	* [x] Check with auto update all files
	* [x] Refactor status and list operation
		* [x] Status to show tracked files and their statuses
		* [x] Show which file has affected the status change
		* [x] Convert list to all information saved in rif in json

### Later

Branch: new_status
* [x] Status to show untracked files list
* [ ] Use gitignore to add files blacklist

Branch: Ergonomics
* [x] Color print logs for better readability
* [x] Check after update option
* [x] Add set option for add subcommand

Branch: Regex
* [ ] Enable regex file arguments and multiple arguments

Branch: Error handling
* [ ] Improve error handling ergonomics
	* [ ] Make error result more understandable
	* [ ] Make error print logging pretty to read 

Whole new othe projects
* [ ] Create vs code extension.

### Bugs

* [x] Self reference panics on sanity check 
This was because child was self-referencing and i didn't made such diversion to check self-referencing in child node.
* [x] Add node method is checker struct doesn't add children to node map