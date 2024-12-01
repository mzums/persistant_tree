use std::io;
use std::rc::Rc;

#[derive(Clone)]
struct Node<T> {
    value: T,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
}

#[derive(Clone)]
pub struct PersistentTree<T> {
    root: Option<Rc<Node<T>>>,
}

impl<T: Ord + Clone> PersistentTree<T> {
    pub fn new() -> Self {
        PersistentTree { root: None }
    }

    pub fn insert(&self, value: T) -> Self {
        PersistentTree {
            root: Self::insert_recursive(&self.root, value),
        }
    }

    fn insert_recursive(node: &Option<Rc<Node<T>>>, value: T) -> Option<Rc<Node<T>>> {
        match node {
            Some(n) if value < n.value => Some(Rc::new(Node {
                value: n.value.clone(),
                left: Self::insert_recursive(&n.left, value),
                right: n.right.clone(),
            })),
            Some(n) if value > n.value => Some(Rc::new(Node {
                value: n.value.clone(),
                left: n.left.clone(),
                right: Self::insert_recursive(&n.right, value),
            })),
            Some(n) => Some(n.clone()),
            None => Some(Rc::new(Node {
                value,
                left: None,
                right: None,
            })),
        }
    }

    pub fn inorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        Self::inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive(node: &Option<Rc<Node<T>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            Self::inorder_recursive(&n.left, result);
            result.push(n.value.clone());
            Self::inorder_recursive(&n.right, result);
        }
    }
}

fn main() {
    let mut versions: Vec<PersistentTree<i32>> = vec![PersistentTree::new()];
    let mut current_version = 0;

    loop {
        println!("\n--- Persistent Tree ---");
        println!("Current version: {}", current_version);
        println!("Tree (inorder): {:?}", versions[current_version].inorder());
        println!("Options:");
        println!("1. Add");
        println!("2. Go to previous version");
        println!("3. Quit");
        println!("Chose option:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Value to be added:");
                let mut value = String::new();
                io::stdin().read_line(&mut value).unwrap();

                if let Ok(value) = value.trim().parse::<i32>() {
                    let new_tree = versions[current_version].clone().insert(value);
                    versions.push(new_tree);
                    current_version = versions.len() - 1;
                } else {
                    println!("Incorrect value!");
                }
            }
            "2" => {
                println!("How many versions do you want to go back?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if let Ok(value) = input.trim().parse::<usize>() {
                    if value > 0 && current_version >= value {
                        current_version -= value;
                        println!("Back by {} versions. Current version: {}", value, current_version);
                    } else {
                        println!("You can't go back that many versions!");
                    }
                } else {
                    println!("Incorrect value!");
                }
            }
            "3" => {
                println!("Bye.");
                break;
            }
            _ => println!("Incorrect option!"),
        }
    }
}
