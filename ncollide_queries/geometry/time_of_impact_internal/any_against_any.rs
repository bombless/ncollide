use na;
use na::Translate;
use math::{Scalar, Point, Vect, Isometry};
use entities::inspection;
use entities::inspection::Repr;
use entities::shape::{Ball, Plane};
use entities::bounding_volume::HasAABB;
use geometry::time_of_impact_internal;

/// Computes the minimum distance separating two shapes.
///
/// Returns `0.0` if the objects are touching or penetrating.
pub fn any_against_any<P, M, G1: ?Sized, G2: ?Sized>(m1: &M, vel1: &P::Vect, g1: &G1,
                                                     m2: &M, vel2: &P::Vect, g2: &G2)
                                                     -> Option<<P::Vect as Vect>::Scalar>
    where P:  Point,
          P::Vect: Translate<P>,
          M:  Isometry<P, P::Vect>,
          G1: Repr<P, M> + HasAABB<P, M>,
          G2: Repr<P, M> + HasAABB<P, M> {
    let r1 = g1.repr();
    let r2 = g2.repr();

    if let (Some(b1), Some(b2)) = (r1.downcast_ref::<Ball<<P::Vect as Vect>::Scalar>>(),
                                   r2.downcast_ref::<Ball<<P::Vect as Vect>::Scalar>>()) {
        let p1 = m1.translate(&na::orig());
        let p2 = m2.translate(&na::orig());

        time_of_impact_internal::ball_against_ball(&p1, vel1, b1, &p2, vel2, b2)
    }
    else if let (Some(p1), Some(s2)) =
            (r1.downcast_ref::<Plane<P::Vect>>(), inspection::maybe_repr_desc_as_support_map::<P, M>(r2)) {
        time_of_impact_internal::plane_against_support_map(m1, vel1, p1, m2, vel2, s2)
    }
    else if let (Some(s1), Some(p2)) =
            (inspection::maybe_repr_desc_as_support_map::<P, M>(r1), r2.downcast_ref::<Plane<P::Vect>>()) {
        time_of_impact_internal::support_map_against_plane(m1, vel1, s1, m2, vel2, p2)
    }
    else if let (Some(s1), Some(s2)) =
            (inspection::maybe_repr_desc_as_support_map::<P, M>(r1), inspection::maybe_repr_desc_as_support_map::<P, M>(r2)) {
        time_of_impact_internal::support_map_against_support_map(m1, vel1, s1, m2, vel2, s2)
    }
    else if let Some(c1) = inspection::maybe_repr_desc_as_composite_shape::<P, M>(r1) {
        time_of_impact_internal::composite_shape_against_any(m1, vel1, c1, m2, vel2, g2)
    }
    else if let Some(c2) = inspection::maybe_repr_desc_as_composite_shape::<P, M>(r2) {
        time_of_impact_internal::any_against_composite_shape(m1, vel1, g1, m2, vel2, c2)
    }
    else {
        panic!("No algorithm known to compute a contact point between the given pair of shapes.")
    }
}
