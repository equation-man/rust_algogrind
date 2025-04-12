//! Queue data structure.
#![allow(warnings)] // Supress all warnings for the entire crate.

#[derive(Debug)]
struct Queue<T> {
    cap: usize, // Capacity
    data: Vec<T>, // Storing elements
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self { cap: size, data: Vec::with_capacity(size) }
    }

    fn is_empty(&self) -> bool { Self::len(&self) == 0 }
    fn is_full(&self) -> bool { self.len() == self.cap }
    fn len(&self) -> usize { self.data.len() }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    // enqueue or add data to the queue
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // pop out values.
    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // IMPLEMENTATION OF ITERATION FOR THE QUEUE.

    // Queue modified and turned into an iterator
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Returning an immutable iterator
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { queue: Vec::new() };
        for item in self.data.iter() {
            iterator.queue.push(item);
        }
        iterator
    }

    // Returning a mutable iterator
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { queue: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.queue.push(item);
        }
        iterator
    }
}

// INTOITER 
struct IntoIter<T>(Queue<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

// ITER
struct Iter<'a, T: 'a> { queue: Vec<&'a T> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() != 0 {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

// ITERMUT
struct IterMut<'a, T: 'a> { queue: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() !=0 {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}


fn main() {
    basic_queue_ops();
    queue_iter_ops();

    // Testing the queue operations.
    fn basic_queue_ops() {
        // The queue instance
        let mut q = Queue::new(4);
        println!("ADDING DATA TO QUEUE");
        q.enqueue("Huxley"); q.enqueue("Adonijah");
        q.enqueue("Harriet"); q.enqueue("Merit");
        println!("The queue data is {:?}", q);
        println!("Empty: {}, full: {}, len:{}", q.is_empty(), q.is_full(), q.len());
        if let Err(error) = q.enqueue("Herine") {
            println!("Enqueue error is: {error}");
        }
        println!("=========================");
        println!("REMOVING THE DATA FROM THE QUEUE");
        if let Some(data) = q.dequeue() {
            println!("The dequeue data is: {data}");
        } else {
            println!("Empty queue");
        }
        println!("Empty: {}, full: {}, len: {}", q.is_empty(), q.is_full(), q.len());
        q.clear();
        println!("The queue is cleared \n{:?}", q);
        println!("=========================");
    }

    // Testing the queue iteration operation.
    fn queue_iter_ops() {
        // The queue instance and adding items.
        let mut q = Queue::new(4);
        q.enqueue("Joseph"); q.enqueue("Fourier");
        q.enqueue("Leonhard"); q.enqueue("Euler");
        println!("TESTING ITERATIONS");
        for item in q.iter() {
            print!("{item}, ");
        }
        println!("\nTesting iter mut");
        for item in q.iter_mut() {
            print!("{item}, ");
        }
        println!("\n============test done===============");
    }
}
