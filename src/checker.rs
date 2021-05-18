use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use colored::*;
use crate::models::{
    enums::FileStatus, 
    rif_error::RifError, 
    rif_list::RifList
};

const DEFAULT_LEVEL: i32 = 0;

#[derive(Clone, Debug)]
struct Node {
    path: PathBuf,
    level: i32,
    parent: Option<PathBuf>,
    children: HashSet<PathBuf>,
}

impl Node {
    fn new(path: &PathBuf) -> Self {
        Self {  
            path: path.clone(),
            level: DEFAULT_LEVEL,
            parent: None,
            children: HashSet::new()
        }
    }
}

pub struct Checker {
    node_map: HashMap<PathBuf, Node>,
    existing: HashSet<PathBuf>,
    non_existing: HashSet<PathBuf>,
}

impl Checker {
    pub fn new() -> Self {
        Self {  
            node_map: HashMap::new(),
            existing: HashSet::new(),
            non_existing: HashSet::new(),
        }
    }

    pub fn add_rif_list(&mut self, rif_list: &RifList) -> Result<(), RifError> {
        for tuple in rif_list.files.iter() {
            self.add_node(tuple.0, &tuple.1.references)?;
            // Clear is necessary because add_node utilizes internal cache for calculation
            // This may be an optimal algorithm, yet it works.
            self.existing.clear();
            self.non_existing.clear();
        }
        Ok(())
    }

    // Add node 
    fn add_node(&mut self, path: &PathBuf, children: &HashSet<PathBuf>) -> Result<(), RifError> {
        // Update existing vector and non-existing vector 
        for child in children.iter() {
            if self.node_map.contains_key(child) {
                self.existing.insert(child.clone());
            } else {
                self.non_existing.insert(child.clone());
            }
        }

        let highest_node_level = self.get_highest_node_level(&self.existing)?;

        // Create new node and insert into node map
        let mut target_node = Node::new(path);
        target_node.level = highest_node_level + 1;
        target_node.children = children.clone();
        self.node_map.insert(path.clone(), target_node);

        // If no reference node exits
        // else, some reference node exists
        if self.non_existing.len() == children.len() {
            for child in children.iter() {
                // Create child node and set necessary variables
                let mut child_node = Node::new(child);
                child_node.parent = Some(path.clone());
                child_node.level = highest_node_level;
                // Insert child node into hashmap
                self.node_map.insert(child.clone(), child_node);
            }
        } else {
            // Create non-existing nodes
            for child in self.non_existing.iter() {
                // Create child node and set necessary variables
                let mut child_node = Node::new(child);
                child_node.parent = Some(path.clone());
                child_node.level = highest_node_level;
                // Insert child node into hashmap
                self.node_map.insert(child.clone(), child_node);
            }

            // Recursively increase a value by 1
            self.recursive_increase(path)?;
        } // if else end

        Ok(())
    } // function end

    // Check methods to modify rif list and yield result
    pub fn check(&mut self, rif_list: &mut RifList) -> Result<(), RifError> {
        // 1. Sort lists
        // 2. and compare children's references
        // 3. Also check filestamp 
        let sorted = self.get_sorted_vec();

        for target_key in sorted.iter() {
            // New file status that will be set to the 'item'
            // Default status is fresh so that file is automatically fresh
            // when there are no references.
            let mut status = FileStatus::Fresh;

            // Item is a node retrieved with item_key
            if let Some(target_node) = self.node_map.get(target_key) {
                // item_ref_keys are vector of keys which parent is the 'item'
                let target_ref_keys = &self.node_map.get(&target_node.path).unwrap().children;

                for key in target_ref_keys.iter() {
                    // Child single_File that is the child of node 'item'
                    if let Some(child_file) = rif_list.files.get(key) {
                        // Made status public for debugging
                        // If child is stale, then parent is automatically stale
                        if let FileStatus::Stale = child_file.status {
                            status = FileStatus::Stale;
                            break;
                        }

                        // If child is fresh but fresher than parent, then parent is stale
                        if child_file.timestamp > rif_list.files.get(target_key).unwrap().timestamp {
                            status = FileStatus::Stale;
                            break;
                        }
                    }
                }
            } else {
                // No node found from item_key
                return Err(RifError::CheckerError(String::from("Failed to find item from key")));
            }

            // Set new status into rif_list
            if let Some(file) = rif_list.files.get_mut(target_key) {
                // Print status changes into stdout
                if file.status != status {
                    println!("Status update \"{}\" {} -> {}", target_key.display().to_string().green(), file.status, status);
                }
                file.status = status;
            } else {
                return Err(RifError::CheckerError(String::from("Failed to find item from rif list")));
            }
        } // for loop end

        Ok(())
    }

    // This vector returns keys array
    fn get_sorted_vec(&self) -> Vec<PathBuf> {
        let mut return_vec = vec![];
        let mut node_vec: Vec<(PathBuf, Node)> 
            = self.node_map.clone().into_iter().collect();

        // Sort nodes by levels
        node_vec.sort_by(|a, b| b.1.level.cmp(&a.1.level));
        // get only keys from node vector
        for tuple in node_vec { return_vec.push(tuple.0); }

        return_vec
    }

    fn get_highest_node_level(&self, children : &HashSet<PathBuf>) -> Result<i32, RifError> {
        let children: Vec<&PathBuf> = children.iter().collect();

        // Early return if children's lenth is 0
        if children.len() == 0 {
            return Ok(DEFAULT_LEVEL);
        }

        // Set first children's level as a highest level for now.
        let mut highest = 
            if let Some(value) = self.node_map.get(children[0]) {
                value.level
            } else {
                return Err(RifError::CheckerError(format!("Failed to get highest number from given children\n{:#?}", children)));
            };

        // Iterate through children and update a highest level
        // If higher number is found.
        for index in 1..children.len() {
            if let Some(value) = self.node_map.get(children[index]) {
                if highest < value.level {
                    highest = value.level;
                }
            } else {
                return Err(RifError::CheckerError(format!("Failed to get highest number from given children\nFrom tree:\n{:#?}\nItem:\n{}", children, children[index].display())));
            };
        }

        Ok(highest)
    } // function end

    fn recursive_increase(&mut self, path: &PathBuf) -> Result<(), RifError> {
        // Recursively increase the level from path to top level
        // Base case
        self.node_map.get_mut(path).unwrap().level += 1;

        // Current node position
        let mut target_path = path.clone();

        loop {
            // Get parent if possible
            // else, there is no parent, break from loop
            if let Some(parent_path) = self.node_map.get(&target_path).unwrap().parent.clone() {
                self.node_map.get_mut(&parent_path).unwrap().level += 1;
                target_path = parent_path;
            } else {
                break;
            }
        }

        Ok(())
    } // function end
}
