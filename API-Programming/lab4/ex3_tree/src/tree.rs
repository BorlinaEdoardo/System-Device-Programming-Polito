use std::collections::HashMap;

struct Node {
    on: bool,
    father: Option<String>
}

pub struct Tree{
    tree: HashMap<String,Node>,
}



impl Tree{

    pub fn new(root_name: &str) -> Tree{
        let mut tree = HashMap::new();
        tree.insert(root_name.to_string(),
                    Node{
                        on:true,
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
}

