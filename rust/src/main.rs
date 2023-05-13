#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }
    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                println!(" here -- << 1");
                self.root = Node::new(value).into();
            }
            Some(node) => {
                println!(" here -- << 2");
                Tree::insert_recursive(node, value);
            }
        }
    }
    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            println!(" rec -- << 1");
            match &mut node.right {
                None => {
                    println!(" rec -- << 1.1");
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            println!(" rec -- << 2");
            match &mut node.left {
                None => {
                    println!(" rec -- << 2.2");
                    node.left = Node::new(value).into();
                }
                Some(n) => Tree::insert_recursive(n, value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works_builds_tree() {
        let mut tree = Tree::new();
        tree.insert(6);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
        tree.insert(8);
        tree.insert(7);
        println!("tree ---- {:?}", tree)
    }
}

fn main() {
    println!("Hello, world!");
}
