// https://rustcurious.com/9

// Exercise: Stack Iterator
//
// See TODO at the very bottom.
//
// Implement the Iterator trait to pass the unit tests.
// Iterating a Stack should behave the same as `pop()`
// so a char is removed and returned at each step.
//
// SPOILER WARNING:
// Contains a solution to the Stack exercise.

/// A first-in-last-out container of up to 25 chars.
pub struct Stack {
    buf: [char; 25],
    len: usize,
}

impl Stack {
    /// Returns an empty Stack.
    pub fn new() -> Stack {
        Stack {
            buf: [' '; 25],
            len: 0,
        }
    }

    /// Returns the current number of items (chars) held.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Pushes the item `c` onto the stack.
    pub fn push(&mut self, c: char) {
        self.buf[self.len] = c;
        self.len += 1;
    }

    /// Removes the top item from the stack and returns it.
    ///
    /// Returns None if the stack is empty.
    pub fn pop(&mut self) -> Option<char> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.buf[self.len])
        } else {
            None
        }
    }
}

#[test]
fn test_for_consume() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    // consumes stack
    for _ in stack {}
}

#[test]
fn test_for_drain() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    for _ in &mut stack {}
    // stack still exists but is empty
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_next() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    assert_eq!(stack.next(), Some('c'));
    assert_eq!(stack.next(), Some('b'));
    assert_eq!(stack.next(), Some('a'));
    assert_eq!(stack.next(), None);
}

#[test]
fn test_zip() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    // The `zip` method comes for free on any type
    // that implements Iterator.
    for (x, y) in stack.zip(['c', 'b', 'a']) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_enumerate() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    // The `enumerate` method comes for free on any type
    // that implements Iterator.
    let expected = ['c', 'b', 'a'];
    for (i, x) in stack.enumerate() {
        assert_eq!(x, expected[i]);
    }
}

// No need to change anything above this line.
// -------------------------------------------------------

impl Iterator for Stack {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
