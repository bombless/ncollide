use na::Transform;
use math::{Point, Vect};

/// Trait of objects that can be tested for point inclusion and projection.
pub trait LocalPointQuery<P: Point> {
    /// Projects a point on `self`.
    fn project_point(&self, pt: &P, solid: bool) -> P;

    /// Computes the minimal distance between a point and `self`.
    fn distance_to_point(&self, pt: &P) -> <P::Vect as Vect>::Scalar;

    /// Tests if the given point is inside of `self`.
    fn contains_point(&self, pt: &P) -> bool;
}

/// Trait of objects that can be transformed and tested for point inclusion and projection.
pub trait PointQuery<P: Point, M: Transform<P>>: LocalPointQuery<P> {
    /// Projects a point on `self` transformed by `m`.
    #[inline]
    fn project_point_with_transform(&self, m: &M, pt: &P, solid: bool) -> P {
        m.transform(&self.project_point(&m.inv_transform(pt), solid))
    }

    /// Computes the minimal distance between a point and `self` transformed by `m`.
    #[inline]
    fn distance_to_point_with_transform(&self, m: &M, pt: &P) -> <P::Vect as Vect>::Scalar {
        self.distance_to_point(&m.inv_transform(pt))
    }

    /// Tests if the given point is inside of `self` transformed by `m`.
    #[inline]
    fn contains_point_with_transform(&self, m: &M, pt: &P) -> bool {
        self.contains_point(&m.inv_transform(pt))
    }
}
