use std::fs;
use std::env;
use std::ops::Add;

#[derive(Debug)]
struct Node<T: PartialOrd + Add + Copy + std::fmt::Display> {
    val: T,
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Add + Copy + std::fmt::Display> Node<T>
    where T: Add,
        <T as std::ops::Add>::Output: PartialOrd<i32> {
    fn new(value: T) -> Node<T> {
        Node {
            val: value,
            l: None,
            r: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value <= self.val {
            match &mut self.l {
                None => {
                    let new_node: Node<T> = Node::new(value);
                    self.l = Some(Box::new(new_node));
                } Some(left_node) => {
                    left_node.insert(value);
                }
            }
        } else {
            match &mut self.r {
                None => {
                    let new_node: Node<T> = Node::new(value);
                    self.r = Some(Box::new(new_node));
                } Some(right_node) => {
                    right_node.insert(value);
                }
            }
        }
    }

    fn node_find_2020(&self, branch: Option<&Box<Node<T>>>) -> (T, T) {
        let hope: bool = true;
        let mut result: (T, T) = (self.val, branch.unwrap().val);
        let mut check_branch = branch;
        while hope {
            println!("{} + {}", self.val, check_branch.unwrap().val);
            if self.val + check_branch.unwrap().val > 2020 {
                match &check_branch.unwrap().l {
                    None => {
                        break;
                    } Some(new_branch) => {
                        check_branch = Some(new_branch);
                    }
                }
            } else if self.val + check_branch.unwrap().val < 2020 {
                match &check_branch.unwrap().r {
                    None => {
                        break;
                    } Some(new_branch) => {
                        check_branch = Some(new_branch);
                    }
                }
            } else {
                println!("FOUND");
                return (self.val, check_branch.unwrap().val);
            }
        }
        println!("SEARCHING LEFT");
        match &self.l {
            None => {}
            Some(left_node) => {
                result = left_node.node_find_2020(branch);
            }
        }

        if result.0 + result.1 == 2020 {
            return result;
        }

        println!("SEARCHING RIGHT");
        match &self.r {
            None => {
                return result;
            } Some(right_node) => {
                result = right_node.node_find_2020(branch);
            }
        }
        result
    }
}

#[derive(Debug)]
struct BST<T: PartialOrd + Add + Copy + std::fmt::Display> {
    root: Option<Box<Node<T>>>,
    size: i64,
}

impl<T: PartialOrd + Add + Copy + std::fmt::Display> BST<T> 
    where T: Add,
        <T as std::ops::Add>::Output: PartialOrd<i32> {
    fn new() -> BST<T> {
        BST {
            root: None,
            size: 0,
        }
    }

    fn insert(&mut self, element: T) {
        match &mut self.root {
            None => {
                let new_node: Node<T> = Node::new(element);
                self.root = Some(Box::new(new_node));
            } Some(root) => {
                root.insert(element);
            }
        }
        self.size += 1;
    }

    fn find_2020(&self) -> (T, T) {
        match &self.root {
            None => {
                panic!("OPORA")
            } Some(root) => {
                root.l.as_ref().unwrap().node_find_2020(root.r.as_ref())
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tree: BST<i32> = BST::new();
    tree.insert(1010);

    let filename: String = args[1].clone();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let temp: i32 = line.parse::<i32>().unwrap();
        tree.insert(temp);
    }

    let results: (i32, i32) = tree.find_2020();
    println!("{}+{} = {}", results.0, results.1, results.0+results.1);
    println!("{}*{} = {}", results.0, results.1, results.0*results.1);
}
