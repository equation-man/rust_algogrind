//! Implementing stack data structure in rust.
/// # Rustack
/// This is a stack data structure implementation 
/// Operates on data on Last In First Out(LIFO) basis.

#[derive(Debug)]
struct Stack<T> {
    // Size of the stack.
    size: usize,
    // Storage
    data: Vec<T>,
}

impl<T> Stack<T> {
    // Initializing the stack
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // Check if the stack is empty.
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Length or number of items in the stack.
    fn len(&self) -> usize {
        self.size
    }

    // Clearing the stack
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // Adding an item on top of the stack
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // Removing the top item from the stack
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        }
        self.size -= 1;
        self.data.pop()
    }

    // Returning reference to the top value
    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    // Return a mutable reference to the top value.
    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    // IMPLEMETING ITERATION FOR THE STACK.

    // Modifying the stack and turning it into an iterator.
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Getting immutable stack iterator.
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    // Getting a mutable stack iterator
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

// Implemetantion of 3 iterations
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> { stack: Vec<&'a T>, }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T:'a> { stack: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn main(){
    println!("The stack implemetation")
}
