use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    val: u32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn makeTree() {
    let leaf = Rc::new(Node {
        val: 6,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    {
        let branch = Rc::new(Node {
            val: 5,
            children: RefCell::new(vec![leaf.clone()]),
            parent: RefCell::new(Weak::new())
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_test() {
        makeTree();
    }
}


// Box<T> has a known size and points to data allocated on the heap. Rc<T> keeps
// track of the number of references to data on the heap so that data can
// have multiple owners. RefCell<T> with its interior mutability gives us a
// type that can be used where we need an immutable type, and enforces
// the borrowing rules at runtime instead of at compile time.

// We've also discussed the Deref and Drop traits that enable a lot of
// smart pointers' functionality. We explored how it's possible to create
// a reference cycle that would cause a memory leak, and how to prevent
// reference cycles by using Weak<T>.
