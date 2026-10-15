// https://rustcurious.com/9

// Exercise: Enumerate
//
// See TODO at the very bottom.
//
// Implement Iterator for the Enumerate iterator adapter
// so the unit tests pass. See commented-out unit tests
// for bonus challenges.

/// An iterator adapter that tracks the iteration count.
pub struct Enumerate<I> {
    // The underlying iterator
    iter: I,

    // The iteration count
    n: usize,
}

/// Wraps the iterator `iter` in the Enumerate adapter.
pub fn enumerate<I: IntoIterator>(iter: I) -> Enumerate<I::IntoIter> {
    Enumerate {
        iter: iter.into_iter(),
        n: 0,
    }
}

#[test]
fn test_for() {
    for (i, x) in enumerate(0..10) {
        assert_eq!(i, x);
    }
}

#[test]
fn test_next() {
    let mut iter = enumerate('a'..='e');
    assert_eq!(iter.next(), Some((0, 'a')));
    assert_eq!(iter.next(), Some((1, 'b')));
    assert_eq!(iter.next(), Some((2, 'c')));
    assert_eq!(iter.next(), Some((3, 'd')));
    assert_eq!(iter.next(), Some((4, 'e')));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_for_array() {
    let arr = [0, 1, 2, 3];
    for (i, x) in enumerate(arr.iter()) {
        assert_eq!(i, *x);
    }
}

// BONUS:
// Uncomment this test and override size_hint so it calls
// the size_hint method on the underlying iterator.
#[test]
fn test_size_hint() {
    let iter = enumerate('a'..='e');
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(Iterator::size_hint(&iter), (5, Some(5)));
}

// BONUS:
// Uncomment this test and implement ExactSizeIterator
// if the underlying iterator I implements it.

#[test]
fn test_exact_size() {
    let iter = enumerate(10..15);
    assert_eq!(iter.len(), 5);
    assert_eq!(ExactSizeIterator::len(&iter), 5);
}

// BONUS that I thought of after recording the video:
// Uncomment this test and change the enumerate function
// so it accepts any type that implements IntoIterator.
// (Requires making changes above, only to `enumerate`.)

#[test]
fn test_intoiter() {
    let arr = [0, 1, 2, 3];
    // &arr is not Iterator, but it is IntoIterator
    for (i, x) in enumerate(&arr) {
        assert_eq!(i, *x);
    }
}

// No need to change anything above this line.
// -------------------------------------------------------

// The Item type should be a tuple containing the iteration
// count and the item from the underlying iterator.

impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let count = self.n;
        self.n += 1;
        Some((count, item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I: Iterator> ExactSizeIterator for Enumerate<I>
where
    I: ExactSizeIterator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}
