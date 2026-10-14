// https://rustcurious.com/8

// Exercise: Slab
//
// This Slab type implements a collection based
// on a fixed-length array. It can hold up to N
// items, with each entry in the array either
// occupied or vacant. The Slab maintains a list of
// vacant entries, with each vacant entry holding
// the index of the next vacant entry. The end of
// the list is represented by the index N, which
// is beyond the end of the array.
//
// Implement the methods to pass the unit tests.

use std::array; // for array::from_fn
use std::mem; // for mem::replace

enum Entry<T> {
    Occupied(T),

    // Contains the index of the next vacant entry
    // in the list, or N if this is the end of the list.
    Vacant(usize),
}

pub struct Slab<T, const N: usize> {
    buf: [Entry<T>; N],

    // Index of the vacant entry at the head of the list,
    // or N if there are no vacant entries.
    //
    // INVARIANT: If head < N then buf[head] must be Vacant.
    // INVARIANT: If head == N then all entries are Occupied.
    head: usize,
}

impl<T, const N: usize> Slab<T, N> {
    /// Construct an empty Slab.
    ///
    /// The unit tests assume `head` will be 0,
    /// and each entry will be initialised with
    /// the index of the next entry. E.g. if N = 4:
    /// [ Vacant(1), Vacant(2), Vacant(3), Vacant(4) ]
    pub fn new() -> Self {
        Self {
            head: 0,
            buf: array::from_fn(|i| Entry::Vacant(i + 1)),
        }
    }

    /// Insert an item.
    ///
    /// On success returns `Ok(index)`.
    /// If the slab is full returns `Err(item)`.
    ///
    /// Inserts the new item at index `head`, and updates
    /// `head` to the next vacant entry in the list, i.e.
    /// the index pointed to by the previously-Vacant entry.
    pub fn insert(&mut self, item: T) -> Result<usize, T> {
        if self.head == N {
            return Err(item);
        }
        let index = self.head;
        let Entry::Vacant(next) = mem::replace(&mut self.buf[index], Entry::Occupied(item)) else {
            unreachable!("head must point to a vacant entry");
        };
        self.head = next;
        Ok(index)
    }

    /// Remove the item at `index` and return it.
    ///
    /// If the entry is vacant, makes no changes
    /// and returns `None`.
    ///
    /// If the entry is occupied, replaces it with
    /// a vacant entry which becomes the new head
    /// of the vacant list, i.e. it points to the
    /// old head, and returns `Some(item)`.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        match mem::replace(&mut self.buf[index], Entry::Vacant(self.head)) {
            Entry::Occupied(item) => {
                self.head = index;
                Some(item)
            }
            vacant => {
                self.buf[index] = vacant;
                None
            }
        }
    }

    /// Get a shared reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get(&self, index: usize) -> Option<&T> {
        match &self.buf[index] {
            Entry::Occupied(x) => Some(x),
            Entry::Vacant(_) => None,
        }
    }

    /// Get a mutable reference to the item at `index`.
    ///
    /// Returns `None` if the entry is vacant.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match &mut self.buf[index] {
            Entry::Occupied(x) => Some(x),
            Entry::Vacant(_) => None,
        }
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

#[cfg(test)]
mod test {
    use super::Slab;

    // We use this as an item type T
    // that cannot be copied or cloned.
    #[derive(Debug, PartialEq, Eq)]
    struct Foo(char);

    // Check `slab` is in the state described by `expected`.
    // For each char in `expected`, if it's ' ' then check
    // that entry is vacant, so `get`, `get_mut` and `remove`
    // all return None. Otherwise check `get` and `get_mut`
    // return a reference to the same char.
    #[track_caller]
    fn check<const N: usize>(slab: &mut Slab<Foo, N>, expected: [char; N]) {
        for (i, c) in expected.into_iter().enumerate() {
            if c == ' ' {
                assert_eq!(slab.get(i), None);
                assert_eq!(slab.get_mut(i), None);
                assert_eq!(slab.remove(i), None);
            } else {
                assert_eq!(slab.get(i), Some(&Foo(c)));
                assert_eq!(slab.get_mut(i), Some(&mut Foo(c)));
            }
        }
    }

    // This test assumes the Slab is initialised as:
    // { head: 0, buf: [Vacant(1), Vacant(2), Vacant(3)] }
    #[test]
    fn test() {
        // New empty Slab
        let mut slab: Slab<Foo, 3> = Slab::new();
        check(&mut slab, [' ', ' ', ' ']);

        // Insert items until it's full
        assert_eq!(slab.insert(Foo('a')), Ok(0));
        check(&mut slab, ['a', ' ', ' ']);

        assert_eq!(slab.insert(Foo('b')), Ok(1));
        check(&mut slab, ['a', 'b', ' ']);

        assert_eq!(slab.insert(Foo('c')), Ok(2));
        check(&mut slab, ['a', 'b', 'c']);

        // Now it's full, insertion fails
        assert_eq!(slab.insert(Foo('d')), Err(Foo('d')));
        check(&mut slab, ['a', 'b', 'c']);

        // Mutate an item in-place
        slab.get_mut(1).unwrap().0.make_ascii_uppercase();
        check(&mut slab, ['a', 'B', 'c']);

        // Remove some items
        assert_eq!(slab.remove(0).unwrap(), Foo('a'));
        check(&mut slab, [' ', 'B', 'c']);

        assert_eq!(slab.remove(1).unwrap(), Foo('B'));
        check(&mut slab, [' ', ' ', 'c']);

        // Insert an item
        assert_eq!(slab.insert(Foo('e')), Ok(1));
        check(&mut slab, [' ', 'e', 'c']);

        // Remove remaining items
        assert_eq!(slab.remove(2).unwrap(), Foo('c'));
        check(&mut slab, [' ', 'e', ' ']);

        assert_eq!(slab.remove(1).unwrap(), Foo('e'));
        check(&mut slab, [' ', ' ', ' ']);
    }
}
