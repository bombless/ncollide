//! Traits and methods to inspect and retrieve the capabilities and representations of shapes at runtime.

pub use self::repr::{ReprDesc, Repr};
pub use self::maybe_as_composite_shape::{
    maybe_as_composite_shape,
    maybe_repr_desc_as_composite_shape,
    composite_shape_repr_id
};
pub use self::maybe_as_support_map::{
    maybe_as_support_map,
    maybe_repr_desc_as_support_map,
    support_map_repr_id
};

use na::{Pnt2, Iso2, Pnt3, Iso3};

#[doc(hidden)]
pub mod repr;
mod maybe_as_composite_shape;
mod maybe_as_support_map;

/// A 2d dynamic representation object.
pub type Repr2<N> = Repr<Pnt2<N>, Iso2<N>>;
/// A 3d dynamic representation object.
pub type Repr3<N> = Repr<Pnt3<N>, Iso3<N>>;
