// https://rustcurious.com/6

// Exercise: Uno Cards
//
// Implement the `value` and `can_play` methods
// so the unit tests pass.
//
// Bonus points:
//
// Implement each method with a single match expression.

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Symbol {
    Number(u8),
    Skip,
    Reverse,
    DrawTwo,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Card {
    Regular(Color, Symbol),
    Wild { draw_four: bool },
}

impl Card {
    /// Returns the point value of the card.
    ///
    /// - Number cards have the value given by the number.
    /// - Skip, Reverse and DrawTwo are worth 20 points.
    /// - Wild is worth 50 points.
    pub fn value(self) -> u8 {
        match self {
            Card::Regular(_, Symbol::Number(x)) => x,
            Card::Wild { .. } => 50,
            _ => 20,
        }
    }

    /// Returns whether this card can be played, given
    /// the current state of play.
    ///
    /// - `current`:  The current color of play.
    /// - `discard`: The card on top of the discard pile.
    ///
    /// Returns true iff:
    ///
    /// - `self` has the same symbol as `discard`; or
    /// - `self` has the same color as `current`; or
    /// - `self` is a Wild card.
    pub fn can_play(self, current: Color, discard: Card) -> bool {
        match (self, discard) {
            (Card::Regular(_, s1), Card::Regular(_, s2)) if s1 == s2 => true,
            (Card::Regular(color, _), _) if color == current => true,
            (Card::Wild { .. }, _) => true,
            _ => false,
        }
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

#[test]
fn test_value() {
    use Color::Red;

    assert_eq!(0, Card::Regular(Red, Symbol::Number(0)).value());
    assert_eq!(9, Card::Regular(Red, Symbol::Number(9)).value());
    assert_eq!(20, Card::Regular(Red, Symbol::Skip).value());
    assert_eq!(20, Card::Regular(Red, Symbol::Reverse).value());
    assert_eq!(20, Card::Regular(Red, Symbol::DrawTwo).value());
    assert_eq!(50, Card::Wild { draw_four: true }.value());
}

#[test]
fn test_can_play() {
    use Color::*;

    for sym in [
        Symbol::Number(6),
        Symbol::Skip,
        Symbol::Reverse,
        Symbol::DrawTwo,
    ] {
        // Current color
        assert_eq!(
            true,
            Card::Regular(Red, sym).can_play(Red, Card::Regular(Red, Symbol::Number(0)))
        );
        // Current color, wild on discard pile
        assert_eq!(
            true,
            Card::Regular(Red, sym).can_play(Red, Card::Wild { draw_four: false })
        );
        // Different color, different symbol
        assert_eq!(
            false,
            Card::Regular(Red, sym).can_play(Blue, Card::Regular(Blue, Symbol::Number(0)))
        );
        // Different color, wild on discard pile
        assert_eq!(
            false,
            Card::Regular(Red, sym).can_play(Blue, Card::Wild { draw_four: false })
        );
        // Different color, same symbol
        assert_eq!(
            true,
            Card::Regular(Red, sym).can_play(Blue, Card::Regular(Blue, sym))
        );
        // Wild can always be played
        assert_eq!(
            true,
            Card::Wild { draw_four: false }.can_play(Blue, Card::Regular(Blue, sym))
        );
    }
}
