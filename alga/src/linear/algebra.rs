use crate::linear::vector::VectorSpace;
use crate::general::{AbstractMagma, DynamicLoop, Operator, MultiplicativeMagma, Identity,
                     AbstractSemigroup, DynamicIdentity, DynamicGroup, Multiplicative};
use approx::RelativeEq;
use crate::morphisms::LinearHomomorphism;

#[derive(Copy, Clone)]
/// Represents the Lie Bracket operator
pub struct LieBracketOp;

impl Operator for LieBracketOp{
    fn operator_token() -> Self{
        LieBracketOp
    }
}

/// Implemented trait for Lie Bracket magmas
pub trait LieBracket : AbstractMagma<LieBracketOp>{
    /// Calculate the Lie Bracket of the elements
    fn lie_bracket(&self, rhs: &Self) -> Self{
        self.operate(rhs)
    }
}
impl<T> LieBracket for T where T:AbstractMagma<LieBracketOp>{ }

/// General algebra where M is the algebra product operator that distributes over addition
pub trait Algebra<M: Operator>: VectorSpace + AbstractMagma<M>{

}

/// Algebra with unity
pub trait UnitalAlgebra<M: Operator>: Algebra<M> + DynamicIdentity<M>{

}

/// Algebra with an assocative operator
pub trait AssociativeAlgebra<M: Operator>: Algebra<M> + AbstractSemigroup<M>{

}

/// Algebra with a Lie Bracket operator
pub trait LieAlgebra: Algebra<LieBracketOp> + LieBracket{
    /// Verify that the Lie bracket is antisymmetric via relative equality
    fn anti_symmetric_bracket_rel(a: &Self, b: &Self) -> bool
    where Self: RelativeEq{
        let ab = a.lie_bracket(b);
        let ba = b.lie_bracket(a);
        relative_eq!(ab + ba, Self::zero())
    }
    /// Verify that the Lie bracket is antisymmetric via strict equality
    fn anti_symmetric_bracket(a: &Self, b: &Self) -> bool
        where Self: PartialEq{
        let ab = a.lie_bracket(b);
        let ba = b.lie_bracket(a);
        ab + ba == Self::zero()
    }
    /// Verify that the Jacobi identity holds for the Lie bracket
    fn jacobi_identity_bracket(a: &Self, b: &Self, c: &Self) -> bool
    where Self: RelativeEq{
        let abc = a.lie_bracket(&b.lie_bracket(c));
        let bca = b.lie_bracket(&c.lie_bracket(a));
        let cab = c.lie_bracket(&a.lie_bracket(b));
        let jacobi = abc + bca + cab;

        relative_eq!(jacobi, Self::zero())
    }
}

pub trait ExpLieAlgebra : LieAlgebra{
    type LieGroup: DynamicGroup<Multiplicative>;

    fn lie_exp(&self) -> Self::LieGroup;
}