use std::fmt::Display;

// Basic node implementation that stores
// a variable of generic type and has a reference
// to the next node after it. Using Optional to
// represent the next node as it may be None (null)
struct Node<'a, T: Display> {
    value: T,
    next: Option<&'a Node<'a, T>>
}

impl<'a, T: Display> Node<'a, T> {
    // Returns a new node storing value
    // with a None next node
    fn new_node(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }

    // Print the current value stored in node
    // Can use println! macro since type
    // of value implements Display trait
    fn print_value(&self) {
        println!("{}", self.value);
    }

    // Prints the next nodes value
    // Really only wrote this to mess around with
    // match statement
    fn print_next(&self) {
        match self.next {
            Some(node) => node.print_value(),
            None => println!("Current node has no next node!"),
        }
    }
}

fn main() {
    let mut i32_node_1 = Node::new_node(9);

    i32_node_1.print_value();
    i32_node_1.print_next(); // Fails

    let i32_node_2 = Node::new_node(10);

    i32_node_1.next = Some(&i32_node_2);

    i32_node_1.print_next();
}
