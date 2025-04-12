#![allow(warnings)]
//! HOT METAL GAME.
///
/// DESCRIPTION  
/// This game involves a group of people from 1 to n in a circle.  
/// Starting from a designated persion, they are passing hot metal to each 
/// other. When the action stops, the person holding the metal is eliminated from 
/// the circle. We assume he/she didn't survive the heat hence lost.  
/// This continues until only one person remains. This is the survivor hence the winner.  
///
/// Applications.  
/// Solving problems involving circular data structures.  
/// Example; Round-robin scheduling, Token parsing i.e Token Ring Network controling 
/// which system gets to send data.  
/// Cryptography and security; Secure message transmission. The choice of surviving 
/// node or system can represent secure channels or keys.  
///
///


#[derive(Debug)]
struct Queue<T> {
    cap: usize, // Capacity.
    data: Vec<T>, // Stored elements.
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self { cap: size, data: Vec::with_capacity(size) }
    }

    fn len(&self) -> usize { self.data.len() }
    fn is_empty(&self) -> bool { Self::len(&self) == 0 }
    fn is_full(&self) -> bool { Self::len(&self) == self.cap }
    fn clear(&mut self) { self.data = Vec::with_capacity(self.cap); }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.data.pop()
        } else {
            None
        }
    }
}

fn hot_metal(names: Vec<&str>, num: usize) -> &str {
    // Adding players to a queue.
    // name: is a collection of the name of the players.
    // num: is the number of players to pass the hot metal to 
    // before elimination.
    //
    let mut q = Queue::new(names.len());
    for name in names {
        q.enqueue(name);
    }

    while q.len() > 1 {
        // Simulating circular movement
        // to imitatie moving the hot metal.
        for _ in 0..num {
            // Remove the name at the front.
            let name = q.dequeue().unwrap();
            // Add the name at the back of the queue.
            let add_to_back = q.enqueue(name);
        }

        // Person who didn't survive the heat is 
        // eliminated. Achieved by removing them from 
        // the queue
        let rm = q.dequeue();
    }

    // Obtain the winner. The only person remaining.
    q.dequeue().unwrap()
}

fn main() {
    // TESTING OUR PROGRAM
    basic_queue_test();
    hot_metal_test();

    fn basic_queue_test() {
        // Testing our queue container
        println!("============ QUEUE TEST =========");
        let mut q = Queue::new(4);
        println!("ADDING TO QUEUE");
        q.enqueue("Leonhard"); q.enqueue("Euler");
        q.enqueue("Simon"); q.enqueue("Laplace");
        println!("The queue is {:?}", q);
        println!("Empty: {}, Full: {}, Length: {}", q.is_empty(), q.is_full(), q.len());
        println!("REMOVING FROM THE QUEUE");
        if let Some(data) = q.dequeue() {
            println!("The dequeue data is: {data}");
        } else {
            println!("Empty queue");
        }
        println!("Empty: {}, Full: {}, Length: {}", q.is_empty(), q.is_full(), q.len());
        q.clear();
        println!("Empty is empty: {:?}", q);
        println!("=================================");
    }

    fn hot_metal_test() {
        // TESTING OUR HOT METAL GAME.
        println!("============ TESTING THE HOT METAL GAME ===========");
        let name = vec!["Jerry", "Tom", "Jim", "Carter", "Rose", "Xavier"];
        let winner = hot_metal(name, 10);
        println!("The winner/survivor is {winner}");
        println!("===================================================");
    }
}
