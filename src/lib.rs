// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Procedural macro to generate binary galois fields
pub use g2gen::g2p;

/// Common trait for finite fields
///
/// All types generated by [g2p] implement this trait.
pub trait GaloisField:
Add<Output=Self>
+ AddAssign
+ Sub<Output=Self>
+ SubAssign
+ Mul<Output=Self>
+ MulAssign
+ Div<Output=Self>
+ DivAssign
+ Copy
+ PartialEq
+ Eq {
    /// The value 0 as a finite field constant
    const ZERO: Self;
    /// The value 1 as a finite field constant
    const ONE: Self;
    /// A generator of the multiplicative group of a finite field
    ///
    /// The powers of this element will generate all non-zero elements of the finite field
    ///
    /// ```rust
    /// use g2p::{GaloisField, g2p};
    ///
    /// g2p!(GF4, 2);
    ///
    /// let g = GF4::GENERATOR;
    /// assert_ne!(g * g, GF4::ONE);
    /// assert_ne!(g * g * g, GF4::ONE);
    /// assert_eq!(g * g* g *g, GF4::ONE)
    /// ```
    const GENERATOR: Self;

    /// Calculate the p-th power of a value
    ///
    /// Calculate the value of x to the power p in finite field arithmethic
    ///
    /// # Example
    /// ```rust
    /// use g2p::{GaloisField, g2p};
    ///
    /// g2p!(GF16, 4);
    ///
    /// let g: GF16 = 2.into();
    /// assert_eq!(g.pow(0), GF16::ONE);
    /// assert_eq!(g.pow(1), g);
    /// assert_eq!(g.pow(2), 4.into());
    /// assert_eq!(g.pow(3), 8.into());
    /// assert_eq!(g.pow(4), 3.into());
    /// ```
    fn pow(self, p: usize) -> Self {
        let mut val = Self::ONE;
        let mut pow_pos = 1 << (::std::mem::size_of::<usize>() * 8 - 1);
        assert_eq!(pow_pos << 1, 0);
        while pow_pos > 0 {
            val *= val;
            if (pow_pos & p) > 0 {
                val *= self;
            }
            pow_pos >>= 1;
        }
        val
    }
}
