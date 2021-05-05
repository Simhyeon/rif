use std::path::PathBuf;
use std::collections::HashMap;
use crate::models::RifError;

pub const DEFAULT_LEVEL: i32 = 0;

#[derive(Clone)]
struct Node {
    path: PathBuf,
    level: i32,
    parent: Option<PathBuf>,
    children: Vec<PathBuf>,
}

impl Node {
    pub fn new(path: &PathBuf) -> Self {
        Self {  
            path: path.clone(),
            level: DEFAULT_LEVEL,
            parent: None,
            children: vec![]
        }
    }
}

pub struct Checker {
    node_map: HashMap<PathBuf, Node>,
    existing: Vec<PathBuf>,
    non_existing: Vec<PathBuf>,
    sorted: Vec<PathBuf>,
}

impl Checker {
    pub fn new() -> Self {
        Self {  
            node_map: HashMap::new(),
            existing: vec![],
            non_existing: vec![],
            sorted: vec![],
        }
    }
    // TODO : Incomplete
    // Add node 
    pub fn add_node(&mut self, path: &PathBuf, children: &Vec<PathBuf>) -> Result<(), RifError> {
        // Boolean whehter the node should be created or not.
        let is_node_new = self.node_map.contains_key(path);

        // Update existing vector and non-existing vector 
        for child in children.iter() {
            if self.node_map.contains_key(child) {
                self.existing.push(child.clone());
            } else {
                self.non_existing.push(child.clone());
            }
        }

        let highest_node_level = self.get_highest_node_level(&self.existing)?;

        // NOTE : Former design was to diverge by the boolean is_node_new
        // However I thought that it might not be necessary
        
        // Create new node and insert into node map
        let mut target_node = Node::new(path);
        target_node.level = highest_node_level + 1;
        self.node_map.insert(path.clone(), target_node);

        if is_node_new {
            // Diversion tree
            // All reference nodes exists
            if self.existing.len() == children.len() {
                // Do nothing. At least for now
            }
            // No reference node exits
            else if self.non_existing.len() == children.len() {
                for item in children.iter() {
                    // Create child node and set necessary variables
                    let mut child_node = Node::new(item);
                    child_node.parent = Some(path.clone());
                    child_node.level = highest_node_level;
                    // Insert child node into hashmap
                    self.node_map.insert(item.clone(), child_node);
                }
            }
            // Some reference node exists
            else {
                // Recursively increase a value
                for item in self.non_existing.iter() {
                    // Create child node and set necessary variables
                    let mut child_node = Node::new(item);
                    child_node.parent = Some(path.clone());
                    child_node.level = highest_node_level;
                    // Insert child node into hashmap
                    self.node_map.insert(item.clone(), child_node);
                }

                self.recursive_increase(path)?;
            }
        } else {

        }

        Ok(())
    }

    pub fn check(&mut self) -> Result<(), RifError> {
        // 1. Sort lists
        // 2. and compare children's references
        // 3. Also check filestamp 
        let sorted = self.get_sorted_vec();
        Ok(())
    }

    pub fn get_sorted_vec(&mut self) -> Vec<PathBuf> {
        vec![]
    }

    pub fn get_highest_node_level(&self, children : &Vec<PathBuf>) -> Result<i32, RifError> {

        // Early return if children's lenth is 0
        if children.len() == 0 {
            return Ok(DEFAULT_LEVEL);
        }

        // Set first children's level as a highest level for now.
        let mut highest = 
            if let Some(value) = self.node_map.get(&children[0]) {
                value.level
            } else {
                return Err(RifError::CheckerError(format!("Failed to get highest number from given children\n{:#?}", children)));
            };

        // Iterate through children and update a highest level
        // If higher number is found.
        for index in 1..children.len() {
            if let Some(value) = self.node_map.get(&children[index]) {
                if highest < value.level {
                    highest = value.level;
                }
            } else {
                return Err(RifError::CheckerError(format!("Failed to get highest number from given children\nFrom tree:\n{:#?}\nItem:\n{:#?}", children, children[index])));
            };
        }

        Ok(highest)
    }

    pub fn recursive_increase(&mut self, path: &PathBuf) -> Result<(), RifError> {
        // Recursively increase the level from path to top level
        Ok(())
    }
}
