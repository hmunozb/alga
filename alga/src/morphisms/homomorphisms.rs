//! Homomorphism traits for structures that perform maps on algebraic objects
use crate::general::{Operator, AbstractMagma, AbstractSemigroup, DynamicGroup, DynamicMonoid, DynamicRing, Id, DynamicGroupAbelian, DynamicAbstractModule, Identity, DynamicModule};
use crate::linear::DynamicVectorSpace;
use crate::general::{Additive, Multiplicative};
extern crate proc_macro;
use proc_macro::TokenStream;
use std::ops::Fn;
use std::marker::PhantomData;
use approx::RelativeEq;

/// Simple base trait for morphisms
pub trait Morphism<S1, S2> {
    /// The arrow method is useful for "small" morphisms such as relations and graph edges
    fn arrow(&self, s1: &S1, s2: &S2) -> bool;
}

/// Trait for morphisms that can be represented as functions
pub trait Map<S1, S2> : Morphism<S1, S2> {
    /// Mapping from S1 to S2
    fn map(&self, s1: S1) -> S2;
}

/// Sub trait for morphisms that take objects by ref
pub trait MapRef<S1,S2>: Map<S1,S2>{
    /// Mapping from S1 to S2 by ref
    fn map_ref(&self, s1: &S1) -> S2;
}

/// Sub trait for morphisms that take objects by ref and may also mutate state during computation
pub trait MapMutRef<S1,S2>: Map<S1,S2>{
    /// Mapping from S1 to S2 by ref
    fn map_mut_ref(&mut self, s1: &S1) -> S2;
}

/// Sub trait for morphisms that write their output to a mut ref and may also mutate state
/// during computation
pub trait MapMutTo<S1, S2>: Map<S1,S2>{
    /// Mapping from S1 to S2 by mut ref
    fn map_mut_to(&mut self, s1: &S1, s2: &mut S2);
}

/// Sub trait for morphisms that write their output to a mut ref with no side effects
pub trait MapTo<S1, S2>: Map<S1,S2>{
    /// Mapping from S1 to S2 by mut ref
    fn map_to(&self, s1: &S1, s2: &mut S2);
}

/// Trait for morphism types that can be construct by composing two compatible morphisms
pub trait FromComposition<S1, S2, S3, M1, M2>: Morphism<S1, S3>{
    /// Create from morphism composition
    fn from_composition(m1: &M1, m2: &M2) -> Self;
}

/// Trait for endomorphism types that can be constructed by composition
pub trait FromEndoComposition<S, M: Morphism<S, S>>: FromComposition<S, S, S, M, M>{
    /// Create from automorphism composition
    fn from_endo_composition(m1: &M, m2: &M) -> Self;
}

// Morphism Implementations
impl<S, M:Morphism<S, S>, T> FromComposition<S, S, S, M, M> for T
where T: FromEndoComposition<S, M>{
    fn from_composition(m1: &M, m2: &M) -> Self{
        Self::from_endo_composition(m1, m2)
    }
}

impl<S1, S2, M> Morphism<S1, S2> for M where M:Map<S1, S2>{
    fn arrow(&self, s1: &S1, s2: &S2) -> bool{
        true
    }
}


//impl<S1,S2,M> Map<S1, S2> for M where M: MapRef<S1,S2>{
//    fn map(&self, s1: S1) -> S2{
//        self.map_ref(&s1)
//    }
//}

macro_rules! impl_map_for_ref{
    (<$S1:tt, $S2:tt> for $T:tt) => {
        impl Map<$S1, $S2> for $T{
            fn map(&self, s1: $S1) -> $S2{
                self.map_ref(&s1)
            }
        }
    }
}

//#[derive(Clone, Copy)]
//pub struct MorphismWrapper<S1: Object, S2: Object, M: Morphism<S1,S2>>(
//    M,PhantomData<(*const S1, *const S2)>);

//pub trait Composable<M2, S2:Object, S3: Object> where M2: Morphism<S2, S3>{ }

///// Trait to define composition between two classes of morphisms
//pub trait MorphismComposition<S1, S2, S3, M1, M2, M3>
//where M1:Morphism<S1,S2>, M2:Morphism<S2,S3>, M3: Morphism<S1,S3> {
//    ///Compose the two morphisms
//    fn compose_morphisms(&self, m1: &M1, m2: &M2) -> M3;
//}
//
///// Trait specialized for automorphism composition
//pub trait AutomorphismComposition<S, M> : MorphismComposition<S, S, S, M, M, M >
//where M: Morphism<S, S> {
//    ///Compose the two automorphisms
//    fn compose_automorphisms(&self, m1: &M, m2: &M) -> M;
//}
//
//impl<S, M, T> MorphismComposition<S, S, S, M, M, M> for T
//where T: AutomorphismComposition<S, M>, M: Morphism<S, S>{
//    fn compose_morphisms(&self, m1: &M, m2: &M) -> M{
//        self.compose_automorphisms(m1, m2)
//    }
//}



//impl<M1, M2, S1: Object, S2: Object, S3: Object>
//Composable<M2, S2, S3> for M1 where M1:Morphism<S1, S2>, M2:Morphism<S2, S3>{ }
//

//
//pub trait Compose<RHS>{
//    type Output;
//    fn compose(&self, rhs: & RHS) -> Self::Output;
//}

//pub trait ObjectComposition <S1:Object, S2:Object, S3:Object>{
//    type CompositionType: Morphism<S1, S3>;
//
//    fn compose<F1: Morphism<S1, S2>, F2: Morphism<S2, S3>>(&self, f1: & F1, f2: & F2)
//                                                           -> Self::CompositionType;
//}

//impl<M1, M2, S1: Object, S2: Object, S3: Object> Compose<M2>
//for M1 where M1:Morphism<S1, S2>, M2:Morphism<S2, S3>{
//    type Output: Morphism<S1, S3>;
//
//    fn compose(&self, rhs: &M2) -> Self::Output{
//
//    }
//}


//Abstract composition operator on a single object
/// Represents composition operator for endomorphisms on the same object S
pub struct Composite<S>{
    _phantom_data: PhantomData<*const S>
}
impl<S> Clone for Composite<S>{
    fn clone(&self) -> Self{
        Composite{_phantom_data: PhantomData}
    }
}
impl<S> Copy for Composite<S>{

}


impl<S> Operator for Composite<S>{
    fn operator_token()->Self { Composite{_phantom_data: PhantomData} }
}

impl<S> Map<S, S> for Id<Composite<S>>{
    fn map(&self, s: S) -> S{
        s
    }
}

impl<S:Clone> MapRef<S, S> for Id<Composite<S>>
{
    fn map_ref(&self, s: &S) -> S{
        s.clone()
    }
}

impl<S:Clone> MapTo<S, S> for Id<Composite<S>>
{
    fn map_to(&self, s1: &S, s2: &mut S) { s2.clone_from(s1); }
}

//impl<S, M>
//AbstractMagma<Composite<S>> for M where M:Morphism<S, S> + FromEndoComposition<S, M> {
//    fn operate(&self, right: &M) -> M{
//        Self::from_auto_composition(&self, right)
//    }
//}

//impl<S1: Object, C: ObjectComposition<S1, S1,S1>,
//    M1:Map<S1,S1>+Compose<C::CompositionType>, >
//AbstractMagma<Composite<S1>> for MorphismWrapper<S1, S1, M1> {
//    fn operate(&self, right: & Self) -> Self{
//        return MorphismWrapper(self.0.compose(right));
//    }
//}



//
// Homomorphism traits
//

/// Trait representing a Homomorphism between two magmas
pub trait MagmaHomomorphism<O1: Operator, O2:Operator,
    M1: AbstractMagma<O1>, M2: AbstractMagma<O2> > : Map<M1, M2>
{
    /// Checks to see whether the map preserves the magma operator
    fn prop_preserves_operator(&self, a: M1, b:M1) -> bool
    where M2 : RelativeEq {
        let ab = a.operate(&b);
        let fab  = self.map(ab);
        let fafb = self.map(a).operate(&self.map(b));
        relative_eq!(fab, fafb)
    }
}

/// Homomorphism between associative magmas
pub trait SemigroupHomomorphism<O1: Operator, O2: Operator,
    G1: AbstractSemigroup<O1>, G2: AbstractSemigroup<O2>> : MagmaHomomorphism<O1, O2, G1, G2>
{
    ///Checks whether the map preserves associativity
    fn prop_preserves_associative_operator(&self, a: G1, b: G1, c: G1) -> bool
    where G2: RelativeEq {
        let a_op_bc = a.operate(&b.operate(&c));
        let ab_op_c = a.operate(&b).operate(&c);
        let f_a_op_bc = self.map(a_op_bc);
        let f_ab_op_c =  self.map(ab_op_c );
        relative_eq!(f_a_op_bc, f_ab_op_c)
    }
}

/// Homomorphism between monoids
pub trait MonoidHomomorphism<O1: Operator, O2: Operator,
    G1: DynamicMonoid<O1>, G2: DynamicMonoid<O2> > : SemigroupHomomorphism<O1, O2, G1,G2>
{
    ///Checks whether the map preserves the identity element
    fn prop_preserves_identity(&self)-> bool
    where G1: Identity<O1>,
          G2: RelativeEq+Identity<O2>{
        let id1 = <G1 as Identity<O1>>::identity();
        let id2 = <G2 as Identity<O2>>::identity();
        relative_eq!(self.map(id1), id2)
    }
}

/// Group Homomorphism
pub trait GroupHomomorphism<O1: Operator, O2: Operator,
    G1: DynamicGroup<O1>, G2: DynamicGroup<O2>> : MonoidHomomorphism<O1, O2, G1, G2>
{

}

/// Ring Homomorphism
pub trait RingHomomorphism<A1: Operator, M1: Operator, A2: Operator, M2: Operator,
R1: DynamicRing<A1, M1>, R2: DynamicRing<A2,M2>>
: GroupHomomorphism<A1, A2, R1, R2> + MonoidHomomorphism<M1, M2, R1, R2>
{

}

/// Module Homomorphism. The commutative ring of G1 must be directly castable into the
/// commutative ring of G2
pub trait AbstractModuleHomomorphism<A1: Operator, A2: Operator, RA: Operator, RM: Operator,
    G1: DynamicAbstractModule<A1, RA, RM>, G2: DynamicAbstractModule<A2, RA, RM>>
: GroupHomomorphism<A1, A2, G1, G2>
where G2::AbstractRing : From<G1::AbstractRing> {

}

/// Module Homomorphism. The commutative ring of G1 must be directly castable into the
/// commutative ring of G2
pub trait ModuleHomomorphism<G1: DynamicModule, G2: DynamicModule>
: AbstractModuleHomomorphism<Additive, Additive, Additive, Multiplicative, G1, G2>
    where G2::Ring : From<G1::Ring> {
}

impl<T, G1, G2> ModuleHomomorphism<G1, G2> for T
where T: AbstractModuleHomomorphism<Additive, Additive,Additive, Multiplicative, G1, G2>,
        G1: DynamicModule, G2: DynamicModule, G2::Ring : From<G1::Ring>
{}

/// Vector Space Homomorphism (i.e. linear operator)
pub trait LinearHomomorphism<V1: DynamicVectorSpace, V2: DynamicVectorSpace>
: AbstractModuleHomomorphism<Additive, Additive, Additive, Multiplicative, V1, V2>
where V2::Field: From<V1::Field>{

}