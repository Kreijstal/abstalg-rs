// Copyright (C) 2020 Miklos Maroti
// Licensed under the MIT license (see LICENSE)

use crate::{
    DistributiveLattice, Domain, EuclideanDomain, Field, IntegralDomain, Lattice, PartialOrder,
    UnitaryRing,
};
use num::{Float, One, Zero};
use std::fmt::Debug;
use std::marker::PhantomData;

/// The field of real numbers approximated by a primitive floating point
/// number. NaN and infinity values are not considered as members, so all
/// operations resulting one of these will panic. The lattice order is the
/// normal total order, which is not bounded.
#[derive(Default)]
pub struct ApproxFloats<E> {
    phantom: PhantomData<E>,
}

impl<E> Domain for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    type Elem = E;

    fn contains(&self, elem: &Self::Elem) -> bool {
        elem.is_finite()
    }
}

impl<E> UnitaryRing for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn zero(&self) -> Self::Elem {
        Zero::zero()
    }

    fn neg(&self, elem: &Self::Elem) -> Self::Elem {
        let r = -*elem;
        assert!(r.is_finite());
        r
    }

    fn add(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let r = *elem1 + *elem2;
        assert!(r.is_finite());
        r
    }

    fn one(&self) -> Self::Elem {
        One::one()
    }

    fn mul(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        let r = *elem1 * *elem2;
        assert!(r.is_finite());
        r
    }
}

impl<E> IntegralDomain for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn try_div(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Option<Self::Elem> {
        self.auto_try_div(elem1, elem2)
    }

    fn associate_repr(&self, elem: &Self::Elem) -> (Self::Elem, Self::Elem) {
        self.auto_associate_repr(elem)
    }
}

impl<E> EuclideanDomain for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn quo_rem(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> (Self::Elem, Self::Elem) {
        self.auto_quo_rem(elem1, elem2)
    }
}

impl<E> Field for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn inv(&self, elem: &Self::Elem) -> Self::Elem {
        self.div(&self.one(), elem)
    }

    fn div(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(!self.is_zero(elem2));
        let r = *elem1 / *elem2;
        assert!(r.is_finite());
        r
    }
}

impl<E> PartialOrder for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn less_or_equal(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> bool {
        *elem1 <= *elem2
    }
}

impl<E> Lattice for ApproxFloats<E>
where
    E: Float + Debug + Zero + One,
{
    fn meet(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        elem1.min(*elem2)
    }

    fn join(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        elem1.max(*elem2)
    }
}

impl<E> DistributiveLattice for ApproxFloats<E> where E: Float + Debug + Zero + One {}