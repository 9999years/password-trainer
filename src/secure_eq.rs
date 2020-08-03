use std::iter;

use secrecy::{ExposeSecret, Secret, Zeroize};

/// A variant of [`std::cmp::Eq`][Eq] robust against timing attacks.
///
/// [Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
pub trait SecureEq<Rhs: ?Sized> {
    /// Is this value equal to `other`?
    fn eq_secure(&self, other: &Rhs) -> bool;

    /// Is this value not equal to `other`?
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq_secure(other)
    }
}

impl<S1, S2> SecureEq<S2> for Secret<S1>
where
    S1: AsRef<str> + Zeroize,
    S2: AsRef<str> + ?Sized,
{
    fn eq_secure(&self, other: &S2) -> bool {
        let mut eq = true;
        for (a, b) in other.as_ref().as_bytes().iter().copied().zip(
            self.expose_secret()
                .as_ref()
                .as_bytes()
                .iter()
                .copied()
                .chain(iter::repeat(0xffu8)),
        ) {
            eq &= a == b;
        }

        // Kathryn Made Me Do It
        // "Made"
        eq & ((other.as_ref().as_bytes().len() - self.expose_secret().as_ref().as_bytes().len())==0)
    }
}
