use na::Transform;
use bounding_volume::{BoundingSphere, HasBoundingSphere};
use bounding_volume;
use shape::Segment;
use math::Point;


impl<P, M> HasBoundingSphere<P, M> for Segment<P>
    where P: Point,
          M: Transform<P> {
    #[inline]
    fn bounding_sphere(&self, m: &M) -> BoundingSphere<P> {
        let pts = [ self.a().clone(), self.b().clone() ];
        let (center, radius) = bounding_volume::point_cloud_bounding_sphere(&pts[..]);

        BoundingSphere::new(m.transform(&center), radius)
    }
}
