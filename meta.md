## TODO

### Imminent

* [ ] Update should fail on non modified file without force option

Branch: New features
* [ ] Add new features
	* [x] Add update message, 
	* [ ] Add update hook so that user can configure automatic behaviour on update.

Branch: Config
* [ ] Create rif config
    * [x] History Capacity
	* [ ] Update Hook
	    * [ ] Check hook status
	    * [ ] Check hook script 
	    * [ ] Argument options(Which argument to pass)
		e.g. Only get fresh vector, only get stale vector, get json struct
    * [ ] Set --check flag for update command as default

* [ ] Testing
* [ ] Consider using sanity check "after" every cli operation

### Done

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

Branch: better_status
* [x] Status to show untracked files list
* [x] Use gitignore like file to add files blacklist(rifignore)
This behaviour also affected remove and add logic

Branch: Ergonomics
* [x] Color print logs for better readability
* [x] Check after update option
* [x] Add set option for add subcommand

Branch: Error handling
* [x] Improve error handling ergonomics
	* [x] Make error print logging pretty to read 
	* [x] Make error result more distinguishable
	Make riferror more diverse and meaningful

Branch : File structure rearrangement
* [x] Files are getting bigger segregate them with multiple modules

Branch: directory
* [x] Make subcommand to get directory as argument
	* [x] Recursive option with directory
* [x] Make rifignore can ignore directory

Branch: Path sanity
* [x] Ensure path can be absolute
Currently all path operations assume that path is relative to rif file, which is ok in most cases however it is better to make it compatible with absolute path.
* [x] Make add subcommand to convert input file path into stripped path

Branch: documentation
* [x] Make good documentation in codes and rearrange for better reading
	* [x] Sort imports
	* [x] Add rust doc comments

* [x] Remove list subcommand's extra new lines
* [x] Add depth option for list subcommand
Currently only direct descendant is shown but it is not always helpful for users. Add depth option so that user can view all file relation tree in cost of performance.
* [x] Rename file method and command line option

Branch: rif-directory
* [x] Migrate from .rif file into .rif directory
This extension of functionality such as history, update hook or something like watch update relatively easy to implement without losing performance. Though current state of optimization is somewhat alreaydy mediocre though.

### Later

* [ ] Export rif as library
* [ ] Create vs code extension

### Known Issues

#### Path sanity
Currently path sanity and directory recursion logics are all based on the fact that command execution can be and only be done on the directory where .rif file dwells (Heavilty using std::env::current_dir method and return Err when there is no rif file in cwd). This is fine for now however it might be problematic when I enable rif to execute command on nested directories.

Thus, consider use std::fs::canonicalize for every path related operations.

### Known Bugs

* [x] Self reference panics on sanity check 
This was because child was self-referencing and i didn't made such diversion to check self-referencing in child node.
* [x] Add node method is checker struct doesn't add children to node map
* [x] .rifignore is not properly applied on directories
* [x] Sanity fix doesn't fix non existent path error
