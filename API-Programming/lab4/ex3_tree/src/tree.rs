use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
struct Node {
    on: bool,
    father: Option<String>
}

#[derive(Debug, Clone)]
pub struct Tree{
    tree: HashMap<String,Node>,
}

impl Display for Tree{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>{
        self.tree.fmt(f)
    }
}

impl Tree{

    pub fn new(root_name: &str) -> Tree{
        let mut tree = HashMap::new();
        tree.insert(root_name.to_string(),
                    Node{
                        on:false,
                        father: None
                    });
        Tree{
            tree,
        }
    }

    pub fn add(& mut self, father: &str, node: &str) {
        if self.tree.contains_key(father){
            let n = Node{
                on: false,
                father: Some(father.to_string())
            };
            self.tree.insert(node.to_string(), n);
        }
    }

    // remove the selected node and all its children
    pub fn remove(&mut self, node: &str) {
        if ! self.tree.contains_key(node) || self.tree.is_empty(){ return; }

        for (key, val) in self.tree.clone().iter(){
            if val.father == Some(node.to_string()){
                self.remove(key);
            }
        }

        self.tree.remove(node);
    }

    /// Toggle the switch (off if previously on and vice versa)
    /// and returns the new value
    pub fn toggle(&mut self, node: &str) -> bool {
        // If the node doesn't exist, bail out early.
        if !self.tree.contains_key(node) {
            return false;
        }

        // 1) Check all ancestors first (immutable borrows only)
        let mut current = Some(node.to_string());
        while let Some(ref name) = current {
            // Safe to unwrap since we know it exists
            let node_ref = &self.tree[name];
            if let Some(ref father) = node_ref.father {
                // If any ancestor is off, force this node off
                if !self.peek(father) {
                    // Now grab a mutable borrow just for setting `on = false`
                    let n = self.tree.get_mut(node).unwrap();
                    n.on = false;
                    return false;
                }
                current = Some(father.clone());
            } else {
                break;
            }
        }

        // 2) All ancestors are on, so toggle this node
        let n = self.tree.get_mut(node).unwrap();
        n.on = !n.on;
        n.on
    }

    // return if the light is on or off
    pub fn peek(&self, node: &str) -> bool {
        if ! self.tree.contains_key(node) || self.tree.is_empty(){ return false; }

        self.tree.get(node).unwrap().on
    }
}

