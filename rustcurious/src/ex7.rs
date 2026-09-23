// https://rustcurious.com/7

// Exercise: Stack
//
// Implement the methods to pass the unit tests.
//
// Yes you could use a Vec as a stack, but that
// would be cheating. In this exercise we use a
// fixed-length array as a buffer, with a separate
// integer field to track the number of items.

/// A first-in-last-out container of up to 25 chars.
#[derive(Default)]
pub struct Stack {
    buf: [char; 25],
    len: usize,
}

impl Stack {
    /// Returns an empty Stack.
    pub fn new() -> Stack {
        Self::default()
    }

    /// Returns the current number of items (chars) held.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Pushes the item `c` onto the stack.
    ///
    /// Panics if already at full capacity.
    /// (You don't need to write code to explicitly panic,
    /// the bounds check on array access will do it for you.)
    pub fn push(&mut self, c: char) {
        self.buf[self.len] = c;
        self.len += 1;
    }

    /// Removes the top item from the stack and returns it.
    ///
    /// Returns None if the stack is empty.
    pub fn pop(&mut self) -> Option<char> {
        self.len = self.len.checked_sub(1)?;
        Option::Some(self.buf[self.len])
    }

    /// Returns a shared reference to the top item.
    ///
    /// Returns None if the stack is empty.
    pub fn peek(&self) -> Option<&char> {
        self.as_slice().last()
    }

    /// Returns a mutable reference to the top item.
    ///
    /// Returns None if the stack is empty.
    pub fn peek_mut(&mut self) -> Option<&mut char> {
        self.as_mut_slice().last_mut()
    }

    /// Returns the content of the stack as a shared
    /// array slice. The most recently pushed item
    /// is the last element in the slice.
    pub fn as_slice(&self) -> &[char] {
        &self.buf[..self.len]
    }

    /// Returns the content of the stack as a mutable
    /// array slice. The most recently pushed item
    /// is the last element in the slice.
    pub fn as_mut_slice(&mut self) -> &mut [char] {
        &mut self.buf[..self.len]
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

#[test]
fn test_len_push_pop() {
    let mut stack = Stack::new();
    assert_eq!(0, stack.len());
    assert_eq!(None, stack.pop());
    stack.push('a');
    stack.push('b');
    stack.push('c');
    assert_eq!(3, stack.len());
    assert_eq!(Some('c'), stack.pop());
    assert_eq!(Some('b'), stack.pop());
    assert_eq!(Some('a'), stack.pop());
    assert_eq!(0, stack.len());
    assert_eq!(None, stack.pop());
}

#[test]
fn test_push_full() {
    let mut stack = Stack::new();
    for c in 'a'..='y' {
        stack.push(c);
    }
    for c in ('a'..='y').rev() {
        assert_eq!(Some(c), stack.pop());
    }
    assert_eq!(None, stack.pop());
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_push_overflow() {
    let mut stack = Stack::new();
    for c in 'a'..='z' {
        stack.push(c);
    }
}

#[test]
fn test_peek() {
    let mut stack = Stack::new();
    assert_eq!(None, stack.peek());
    stack.push('a');
    stack.push('b');
    stack.push('c');
    assert_eq!(Some(&'c'), stack.peek());
    assert_eq!(Some('c'), stack.pop());
    assert_eq!(Some(&'b'), stack.peek());
    assert_eq!(Some('b'), stack.pop());
    assert_eq!(Some(&'a'), stack.peek());
    assert_eq!(Some('a'), stack.pop());
    assert_eq!(None, stack.peek());
}

#[test]
fn test_peek_mut() {
    let mut stack = Stack::new();
    assert_eq!(None, stack.peek_mut());
    stack.push('a');
    stack.push('b');
    stack.push('c');
    stack.peek_mut().unwrap().make_ascii_uppercase();
    assert_eq!(Some('C'), stack.pop());
    stack.peek_mut().unwrap().make_ascii_uppercase();
    assert_eq!(Some('B'), stack.pop());
    stack.peek_mut().unwrap().make_ascii_uppercase();
    assert_eq!(Some('A'), stack.pop());
    assert_eq!(None, stack.peek_mut());
}

#[test]
fn test_slices() {
    let mut stack = Stack::new();
    assert_eq!(stack.as_slice(), &[]);
    assert_eq!(stack.as_mut_slice(), &[]);
    stack.push('a');
    stack.push('b');
    stack.push('c');
    assert_eq!(stack.as_slice(), &['a', 'b', 'c']);
    assert_eq!(stack.as_mut_slice(), &['a', 'b', 'c']);
    stack.as_mut_slice()[1] = 'B';
    assert_eq!(stack.as_slice(), &['a', 'B', 'c']);
    assert_eq!(stack.as_mut_slice(), &['a', 'B', 'c']);
    stack.pop();
    assert_eq!(stack.as_slice(), &['a', 'B']);
    stack.pop();
    stack.pop();
    assert_eq!(stack.as_slice(), &[]);
    assert_eq!(stack.as_mut_slice(), &[]);
}
