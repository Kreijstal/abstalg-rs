// Copyright (C) 2020 Miklos Maroti
// Licensed under the MIT license (see LICENSE)

use crate::{Domain, EuclideanDomain, Field, IntegralDomain, UnitaryRing};

/// The Euclidean ring of polynomials over the 2-element field.
pub const F2X: Polynomials<crate::QuotientField<crate::CheckedInts<i8>>> =
    Polynomials { base: crate::F2 };

/// The Euclidean ring of polynomials over the 3-element field.
pub const F3X: Polynomials<crate::QuotientField<crate::CheckedInts<i8>>> =
    Polynomials { base: crate::F3 };

/// The integral domain of polynomials over the ring of integers.
pub const ZX: Polynomials<crate::Integers> = Polynomials { base: crate::Z };

/// The ring of polynomials over a base ring or field where each element
/// is represented as a vector whose last element, the leading coefficient (if
/// any) must be non-zero. This means that the empty vector is the zero
/// element, and every polynomial has a unique representation.
#[derive(Clone, Debug, Default)]
pub struct Polynomials<R>
where
    R: UnitaryRing,
{
    base: R,
}

impl<R: UnitaryRing> Polynomials<R> {
    /// Creates a new ring of polynomials over the given ring. The ring cannot
    /// be trivial, that is one must be different from zero.
    pub fn new(base: R) -> Self {
        assert!(!base.is_zero(&base.one()));
        Polynomials { base }
    }

    /// Returns the base ring from which this ring was created.
    pub fn base(&self) -> &R {
        &self.base
    }

    /// Returns the degree of the given polynomial. The zero polynomial has
    /// no degree.
    #[allow(clippy::ptr_arg)]
    pub fn degree(&self, elem: &Vec<R::Elem>) -> Option<usize> {
        if elem.is_empty() {
            None
        } else {
            Some(elem.len() - 1)
        }
    }

    /// Returns the leading coefficient of the given polynomial. The zero
    /// polynomial has no leading coefficient.
    #[allow(clippy::ptr_arg)]
    pub fn leading_coef(&self, elem: &Vec<R::Elem>) -> Option<R::Elem> {
        if elem.is_empty() {
            None
        } else {
            Some(elem[elem.len() - 1].clone())
        }
    }
}

impl<R> Domain for Polynomials<R>
where
    R: UnitaryRing,
{
    type Elem = Vec<R::Elem>;

    fn contains(&self, elem: &Self::Elem) -> bool {
        let mut last = &self.base.one();
        for a in elem.iter() {
            if !self.base.contains(a) {
                return false;
            }
            last = a;
        }
        !self.base.is_zero(last)
    }

    fn equals(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> bool {
        if elem1.len() != elem2.len() {
            false
        } else {
            elem1
                .iter()
                .zip(elem2.iter())
                .all(|(x, y)| self.base.equals(x, y))
        }
    }
}

impl<R> UnitaryRing for Polynomials<R>
where
    R: UnitaryRing,
{
    fn zero(&self) -> Self::Elem {
        Vec::new()
    }

    fn is_zero(&self, elem: &Self::Elem) -> bool {
        elem.is_empty()
    }

    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        elem.iter().map(|a| self.base.neg(a)).collect()
    }

    fn add(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        if elem1.len() != elem2.len() {
            let (elem1, elem2) = if elem1.len() > elem2.len() {
                (elem1, elem2)
            } else {
                (elem2, elem1)
            };
            let mut elem3 = elem1.clone();
            for i in 0..elem2.len() {
                elem3[i] = self.base.add(&elem3[i], &elem2[i]);
            }
            elem3
        } else {
            let mut elem3 = Vec::new();
            for i in 0..elem1.len() {
                let a = self.base.add(&elem1[i], &elem2[i]);
                if !self.base.is_zero(&a) {
                    elem3.resize(i + 1, self.base.zero());
                    elem3[i] = a;
                }
            }
            elem3
        }
    }

    fn one(&self) -> Self::Elem {
        vec![self.base.one()]
    }

    fn is_one(&self, elem: &Self::Elem) -> bool {
        elem.len() == 1 && self.base.is_one(&elem[0])
    }

    fn mul(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        if elem1.is_empty() || elem2.is_empty() {
            Vec::new()
        } else {
            let mut elem3 = Vec::with_capacity(elem1.len() + elem2.len() - 1);
            elem3.resize(elem1.len() + elem2.len() - 1, self.base.zero());
            for i in 0..elem1.len() {
                for j in 0..elem2.len() {
                    let a = self.base.mul(&elem1[i], &elem2[j]);
                    elem3[i + j] = self.base.add(&elem3[i + j], &a);
                }
            }
            elem3
        }
    }
}

impl<R> IntegralDomain for Polynomials<R>
where
    R: IntegralDomain,
{
    fn try_div(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Option<Self::Elem> {
        if elem2.is_empty() || elem1.len() < elem2.len() {
            if elem1.is_empty() {
                Some(self.zero())
            } else {
                None
            }
        } else {
            let mut quo = Vec::with_capacity(elem1.len() + elem2.len() - 1);
            quo.resize(elem1.len() - elem2.len() + 1, self.base.zero());
            let mut rem = elem1.clone();

            let a = &elem2[elem2.len() - 1];
            assert!(!self.base.is_zero(a));

            for i in (0..quo.len()).rev() {
                quo[i] = self.base.try_div(&rem[i + elem2.len() - 1], a)?;
                let b = self.base.neg(&quo[i]);
                for j in 0..(elem2.len() - 1) {
                    let c = self.base.mul(&elem2[j], &b);
                    rem[i + j] = self.base.add(&rem[i + j], &c);
                }
            }

            for d in rem.iter().take(elem2.len() - 1) {
                if !self.base.is_zero(d) {
                    return None;
                }
            }
            Some(quo)
        }
    }

    fn associate_repr(&self, elem: &Self::Elem) -> (Self::Elem, Self::Elem) {
        if elem.is_empty() {
            (self.zero(), self.one())
        } else {
            let last = elem.len() - 1;
            let (repr, unit) = self.base.associate_repr(&elem[last]);
            let mut elem: Self::Elem = elem.iter().map(|x| self.base.mul(x, &unit)).collect();
            elem[last] = repr; // overwrite for approximate operations
            (elem, vec![unit])
        }
    }
}

impl<F> EuclideanDomain for Polynomials<F>
where
    F: Field,
{
    fn quo_rem(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> (Self::Elem, Self::Elem) {
        if elem2.is_empty() || elem1.len() < elem2.len() {
            (self.zero(), elem1.clone())
        } else {
            let mut quo = Vec::with_capacity(elem1.len() + elem2.len() - 1);
            quo.resize(elem1.len() - elem2.len() + 1, self.base.zero());
            let mut rem = elem1.clone();

            let a = &elem2[elem2.len() - 1];
            assert!(!self.base.is_zero(a));

            for i in (0..quo.len()).rev() {
                quo[i] = self.base.div(&rem[i + elem2.len() - 1], a);
                let b = self.base.neg(&quo[i]);
                for j in 0..(elem2.len() - 1) {
                    let c = self.base.mul(&elem2[j], &b);
                    rem[i + j] = self.base.add(&rem[i + j], &c);
                }
            }

            let mut i = elem2.len() - 1;
            while i > 0 && self.base.is_zero(&rem[i - 1]) {
                i -= 1;
            }
            rem.truncate(i);

            (quo, rem)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{QuotientField, I32};

    #[test]
    fn field_256() {
        let field1 = QuotientField::new(I32, 2);
        let ring2 = Polynomials::new(field1);

        // the irreducible polynomial 1 + x + x^3 + x^4 + x^8
        let poly = vec![1, 1, 0, 1, 1, 0, 0, 0, 1];
        assert!(ring2.contains(&poly));
        assert_eq!(ring2.degree(&poly), Some(8));
        let field2 = QuotientField::new(ring2, poly);

        // 1 + x, primitive element, generate all 256 elements
        let gen = vec![1, 1];
        let mut elems = Vec::new();
        elems.push(field2.zero());
        elems.push(field2.one());
        let mut elem = gen.clone();
        while !field2.is_one(&elem) {
            elems.push(elem.clone());
            elem = field2.mul(&elem, &gen);
        }
        assert_eq!(elems.len(), 256);

        for a in elems {
            if !field2.is_zero(&a) {
                let b = field2.inv(&a);
                assert_eq!(field2.mul(&a, &b), field2.one())
            }
        }
    }
}
