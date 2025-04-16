//! PALINDROME CHECKER
#![allow(warnings)]
/// Palindromes are strings in which characters at the same position
/// from both the ends are the same.

#[derive(Debug)]
struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(size: usize) -> Self {
        Self { cap: size, data: Vec::with_capacity(size) }
    }

    fn len(&self) -> usize { self.data.len() }
    fn is_full(&self) -> bool { self.len() == self.cap }
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn clear(&mut self) { self.data = Vec::with_capacity(self.cap) }

    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space left".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space left".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn remove_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.data.pop()
        } else {
            None
        }
    }

    fn remove_rear(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

fn palindrome_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        // Add rear so they retain their order
        let r_ = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.len() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

// Testing the program
fn main() {
    deque_ops();
    pal_ops();

    fn deque_ops() {
        println!("TESTING DEQUE OPERATIONS");
        let mut deque = Deque::new(4);
        deque.add_front("Leonhard"); deque.add_rear("Euler");
        deque.add_front("Simon"); deque.add_rear("Laplace");
        println!("The deque data is {:?}", deque);
        deque.remove_front(); deque.remove_rear();
        println!("The deque data is {:?}", deque);
        println!("=======================================");
    }

    fn pal_ops() {
        println!("CHECKING FOR PALINDROME IN A STRING");
        let my_str = "Greek";
        let check_pal = palindrome_checker(my_str);
        println!("The string '{my_str}' is palindrome: {check_pal}");
        println!("==============================================");
    }
}
