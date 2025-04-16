//! Deque  
/// Allows items to be added and removed from both sides.
/// Behaves both as a stack and a queue.
#![allow(warnings)]

#[derive(Debug)]
struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Self {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    fn len(&self) -> usize { self.data.len() }
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn is_full(&self) -> bool { self.len() == self.cap }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    // Adding data at the front or start of the deque
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    // Adding data at the rear or end of the deque
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // Remove data from the start or front of the deque
    fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // Remove data from the rear or end of the deque
    fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    // IMPLEMENTING ITERATION.
    //
    // IntoIter: Iterator modified and iterator is returned.
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Iter: Return an immutable iterator.
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { deque: Vec::new() };
        for item in self.data.iter() {
            iterator.deque.push(item);
        }
        iterator
    }

    // IterMut: Return the mutable version of the iterator.
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { deque: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.deque.push(item);
        }
        iterator
    }
}


// IntoIter
struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

// Iter
struct Iter<'a, T: 'a> { deque: Vec<&'a T> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.deque.len() != 0 {
            Some(self.deque.remove(0))
        } else {
            None
        }
    }
}

// IterMut
struct IterMut<'a, T: 'a> { deque: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.deque.len() != 0 {
            Some(self.deque.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    basic_ops();
    iterative_ops();

    fn basic_ops() {
        println!("BASIC OPERATIONS");
        let mut deck = Deque::new(4);
        deck.add_front("Galileo"); deck.add_rear("Galilei");
        deck.add_front("Leonhard"); deck.add_rear("Euler");
        println!("The deque is {:?}", deck);
        println!("Empty: {}, Full: {}, Len: {}", deck.is_empty(), deck.is_full(), deck.len());
        deck.remove_front(); deck.remove_rear();
        println!("The deque is {:?}", deck);
        deck.clear();
        println!("Empty: {}, Full: {}, Len: {}", deck.is_empty(), deck.is_full(), deck.len());
        println!("================================================");
    }

    fn iterative_ops() {
        println!("ITERATIVE OPERATIONS");
        let mut deck = Deque::new(4);
        deck.add_front("Joseph"); deck.add_rear("Fourier");
        deck.add_front("Simon"); deck.add_rear("Laplace");
        println!("The immutable iterator is: \n");
        for item in deck.iter() {
            print!("{}, ",item);
        }
        println!("\nThe mutable iterator is: \n");
        for item in deck.iter_mut() {
            print!("{}, ", item);
        }
        println!("\n================================================");
    }
}
