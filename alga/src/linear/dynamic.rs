use crate::general::{DynamicModule, Field};

/// A vector space has a module structure over a field instead of a ring.
pub trait DynamicVectorSpace: DynamicModule<Ring = <Self as DynamicVectorSpace>::Field>
/* +
ClosedDiv<<Self as VectorSpace>::Field> */
{
    /// The underlying scalar field.
    type Field: Field;
}
