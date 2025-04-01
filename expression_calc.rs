#![allow(dead_code)]
//! Evaluating mathematical expressions.
/// This algorithm reads mathematical expressions involving brackets from left to 
/// right and determines whether the brackets match
///
use std::collections::HashMap;

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T>{
    // Initializing the stack.
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    // IMPLEMENTING ITERATION.
    // into_iter: Modifying the stack to be an iterator.
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // iter: Stack unmodified, getting immutable iterator.
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    // iter_mut: Stack unmodified, gettin mutable iterator
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

///IMPLEMENTING THE THREE ITERATORS.
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

struct Iter<'a, T: 'a> { stack: Vec<&'a T> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> { stack: Vec<&'a mut T> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

/// Parenthesis checker.
fn par_match(open: char, close: char) -> bool {
    let opens = "({[";
    let closes = ")}]";
    opens.find(open) == closes.find(close)
}

/// Check if the parentheses are balanced.
fn par_checker(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            // Push opennig bracket.
            stack.push(c);
        }
        // Closing bracket to check for balance.
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) { balance = false; }
            }
        }
        // Skip if not balanced
        index += 1;
    }
    balance && stack.is_empty()
}

/// Converting infix expression to postfix.
fn infix_to_postfix(infix: &str) -> Option<String> {
    // Check if parentheses are balanced.
    if !par_checker(infix) {
        return None
    }
    // Set priority of all symbols.
    let mut prec = HashMap::new();
    prec.insert("(", 1); prec.insert(")", 1);
    prec.insert("+", 2); prec.insert("-", 2);
    prec.insert("*", 3); prec.insert("/", 3);

    // ops: svae operators, postfix: svae postfix expression
    let mut ops = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        // Characters 0-9 and A-Z onto stack.
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            ops.push(token);
        } else if ")" == token {
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            while (!ops.is_empty()) && (prec[ops.peek().unwrap()] >= prec[token]) {
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    // Push the remaining operators into stack.
    while !ops.is_empty() {
        postfix.push(ops.pop().unwrap());
    }

    // pop out operators and create the postfix expression.
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += "";
    }
    Some(postfix_str)
}

fn main() {
    println!("Testing our scintific calculator");
}
