mod tree;



#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn test_add(){
        let mut tree = Tree::new("test");
        tree.add("test", "son1");
        tree.add("test", "son2");
        tree.add("son2", "son3");

        println!("Test add: {:#?}", tree);
    }

    #[test]
    fn test_remove(){
        let mut tree = Tree::new("test");
        tree.add("test", "son1");
        tree.add("son1", "son2");
        tree.add("son2", "son3");
        tree.add("son2", "son4");
        tree.add("son1", "son5");

        tree.remove("son1");

        assert_eq!(tree.to_string(), "{\"test\": Node { on: false, father: None }}");
    }

    #[test]
    fn test_on_off(){
        let mut tree = Tree::new("test");
        tree.add("test", "son1");
        tree.add("son1", "son2");
        tree.add("son2", "son3");
        tree.add("son2", "son4");
        tree.add("son1", "son5");

        tree.toggle("son1");
        assert_eq!(false, tree.peek("son1"));

        tree.toggle("test");
        tree.toggle("son1");
        tree.toggle("son2");
        assert_eq!(true, tree.peek("son1"));
        assert_eq!(true, tree.peek("son2"));
    }

}
