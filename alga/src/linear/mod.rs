//! Traits dedicated to linear algebra.

pub use self::matrix::{InversibleSquareMatrix, Matrix, MatrixMut, SquareMatrix, SquareMatrixMut};
pub use self::transformation::{
    AffineTransformation, DirectIsometry, Isometry, OrthogonalTransformation,
    ProjectiveTransformation, Rotation, Scaling, Similarity, Transformation, Translation,
};
pub use self::vector::{
    AffineSpace, EuclideanSpace, FiniteDimInnerSpace, FiniteDimVectorSpace, InnerSpace,
    NormedSpace, VectorSpace,
};
pub use self::algebra::{Algebra, AssociativeAlgebra, LieAlgebra, LieBracket, LieBracketOp};
pub use self::dynamic::*;
mod id;
mod matrix;
mod transformation;
mod vector;
mod algebra;
mod dynamic;