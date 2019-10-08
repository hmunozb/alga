use approx::RelativeEq;

use crate::general::{Operator, AbstractSemigroup, AbstractMonoid, AbstractLoop, AbstractQuasigroup,
                     AbstractGroup, Module, RingCommutative, AbstractRing, AbstractRingCommutative,
                     AbstractGroupAbelian, AbstractModule};
use crate::general::identity::DynamicIdentity;
use crate::general::{Additive, Multiplicative};
use crate::general::ClosedMul;

///Dynamic Loop
pub trait DynamicLoop<O: Operator>: AbstractQuasigroup<O> + DynamicIdentity<O> {

}

impl<O: Operator, T: AbstractLoop<O>> DynamicLoop<O> for T{}

/// A Dynamic Monoid assigns an identity to each element such that a * e = e * a = a
pub trait DynamicMonoid<O: Operator>: AbstractSemigroup<O> + DynamicIdentity<O> {
    /// Checks whether operating on an argument with its canonical identity element is a no-op
    /// Approximate equality is used for verifications.
    fn prop_operating_identity_element_is_noop_approx(args: (Self,)) -> bool
        where
            Self: RelativeEq,
    {
        let (a,) = args;
        let id = a.identity();
        relative_eq!(a.operate(&id), a)
            && relative_eq!(id.operate(&a), a)
    }

    /// Checks whether operating on the argument with its canonical identity element is a no-op
    fn prop_operating_identity_element_is_noop(args: (Self,)) -> bool
        where
            Self: Eq,
    {
        let (a,) = args;
        let id = a.identity();
        a.operate(&id) == a && id.operate(&a) == a
    }
}

impl<O: Operator, T: AbstractMonoid<O>> DynamicMonoid<O> for T{

}


/// A dynamic group is a loop and a dynamic monoid
pub trait DynamicGroup<O: Operator>: DynamicLoop<O> + DynamicMonoid<O> { }

impl<O: Operator, T: AbstractGroup<O>> DynamicGroup<O> for T {

}

/// Dynamic Abelian Group
pub trait DynamicGroupAbelian<O: Operator>: DynamicGroup<O> {
    /// Returns `true` if the operator is commutative for the given argument tuple. Approximate
    /// equality is used for verifications.
    fn prop_is_commutative_approx(args: (Self, Self)) -> bool
        where
            Self: RelativeEq,
    {
        let (a, b) = args;
        relative_eq!(a.operate(&b), b.operate(&a))
    }

    /// Returns `true` if the operator is commutative for the given argument tuple.
    fn prop_is_commutative(args: (Self, Self)) -> bool
        where
            Self: Eq,
    {
        let (a, b) = args;
        a.operate(&b) == b.operate(&a)
    }
}

impl<O: Operator, T: AbstractGroupAbelian<O>> DynamicGroupAbelian<O> for T { }

///Dynamic Ring
pub trait DynamicRing<A: Operator = Additive, M: Operator = Multiplicative>:
DynamicGroupAbelian<A> + DynamicMonoid<M>
{ }

impl<A: Operator, M: Operator, T : AbstractRing<A, M>> DynamicRing<A, M> for T{ }

///Dynamic Commutative Ring
pub trait DynamicRingCommutative<A: Operator = Additive, M: Operator = Multiplicative>:
DynamicRing<A, M>
{ }

impl<A: Operator, M: Operator, T : AbstractRingCommutative<A, M>> DynamicRingCommutative<A, M> for T{ }

///Dynamic (abstract) Module
pub trait DynamicAbstractModule<
    OpGroup: Operator = Additive,
    OpAdd: Operator = Additive,
    OpMul: Operator = Multiplicative,
>: DynamicGroupAbelian<OpGroup>
{
    /// The underlying scalar field.
    type AbstractRing: AbstractRingCommutative<OpAdd, OpMul>;

    /// Multiplies an element of the ring with an element of the module.
    fn multiply_by(&self, r: Self::AbstractRing) -> Self;
}

impl<OpGroup: Operator, OpAdd: Operator, OpMul: Operator, T>
DynamicAbstractModule<OpGroup, OpAdd,OpMul> for T
where T: AbstractModule<OpGroup, OpAdd, OpMul>
{
    type AbstractRing = <T as AbstractModule<OpGroup, OpAdd, OpMul>>::AbstractRing;

    fn multiply_by(&self, r: Self::AbstractRing) -> Self{
        <Self as AbstractModule<OpGroup, OpAdd, OpMul>>::multiply_by(&self, r)
    }
}

///Dynamic Module (Additive group, Additive-multiplicative ring)
pub trait DynamicModule:
DynamicAbstractModule<AbstractRing = <Self as DynamicModule>::Ring>
+ DynamicGroupAbelian<Additive>
+ ClosedMul<<Self as DynamicModule>::Ring>
{
    /// The underlying scalar field.
    type Ring: AbstractRingCommutative;
}

impl<T: Module> DynamicModule for T {
    type Ring = <T as Module>::Ring;
}