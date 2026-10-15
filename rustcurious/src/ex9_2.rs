// https://rustcurious.com/9

// Exercise: Slab Index
//
// See TODO at the very bottom.
//
// This Slab type implements a collection based
// on a fixed-length array. It can hold up to N
// items, with each entry in the array either
// occupied or vacant.
//
// Implement Index and IndexMut so a Slab can be
// indexed using square brackets with a usize index.
//
// Accessing a vacant index should panic, just like
// indexing beyond the end of an array panics.
//
// SPOILER WARNING:
// Contains a solution to the Slab exercise.

use std::array; // for array::from_fn
use std::mem; // for mem::replace

enum Entry<T> {
    Occupied(T),
    Vacant(usize),
}

pub struct Slab<T, const N: usize> {
    buf: [Entry<T>; N],
    head: usize,
}

impl<T, const N: usize> Slab<T, N> {
    pub fn new() -> Self {
        Slab {
            buf: array::from_fn(|i| Entry::Vacant(i + 1)),
            head: 0,
        }
    }

    pub fn insert(&mut self, item: T) -> Result<usize, T> {
        let index = self.head;
        if index == N {
            return Err(item);
        }
        let Entry::Vacant(next) = self.buf[index] else {
            panic!("head must be vacant");
        };
        self.buf[index] = Entry::Occupied(item);
        self.head = next;
        Ok(index)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if let Entry::Vacant(_) = self.buf[index] {
            return None;
        }
        let Entry::Occupied(item) = mem::replace(&mut self.buf[index], Entry::Vacant(self.head))
        else {
            unreachable!();
        };
        self.head = index;
        Some(item)
    }

    /// Get a shared reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get(&self, index: usize) -> Option<&T> {
        match &self.buf[index] {
            Entry::Occupied(item) => Some(item),
            Entry::Vacant(_) => None,
        }
    }

    /// Get a mutable reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.buf[index] {
            Entry::Occupied(item) => Some(item),
            Entry::Vacant(_) => None,
        }
    }
}

#[test]
#[should_panic]
fn test_index_panic() {
    #[derive(Debug)]
    struct Foo; // private type prevents cheating

    let mut slab: Slab<Foo, 5> = Slab::new();
    let i = slab.insert(Foo).unwrap();
    slab.remove(i).unwrap();
    let _ = slab[i]; // panic
}

#[test]
#[should_panic]
fn test_index_mut_panic() {
    let mut slab: Slab<char, 5> = Slab::new();
    slab[0] = 'x'; // panic
}

#[test]
fn test() {
    let mut slab: Slab<char, 3> = Slab::new();
    let i = slab.insert('a').unwrap();

    let a = slab[i]; // uses Index
    assert_eq!(a, 'a');
    slab[i] = 'b'; // uses IndexMut
    slab[i].make_ascii_uppercase(); // uses IndexMut
    let b = slab[i]; // uses Index
    assert_eq!(b, 'B');
}

// No need to change anything above this line.
// -------------------------------------------------------

use std::ops::{Index, IndexMut};

impl<T, const N: usize> Index<usize> for Slab<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("slab entry should be occupied")
    }
}

impl<T, const N: usize> IndexMut<usize> for Slab<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("slab entry should be occupied")
    }
}
