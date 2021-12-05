use std::sync::atomic::{AtomicUsize, Ordering};
use crate::matrix::Matrix4;
use crate::tuple::Tuple;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub(crate) transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        return Sphere {
            id,
            transform: Matrix4::identity(),
        };
    }

    pub fn set_transform(&mut self, new_transform: Matrix4) {
        self.transform = new_transform;
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::point(0.0,0.0,0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;

        return world_normal.normalize();
    }
}


#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};
    use crate::sphere::{Sphere};
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(Matrix4::identity(), s.transform);
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();

        let t = Matrix4::translation(2.0, 3.0, 4.0);
        s.set_transform(t);

        assert_eq!(t, s.transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.set_transform(Matrix4::scaling(2.0, 2.0, 2.0));
        let xs = r.intersect(s);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let mut s = Sphere::new();
        s.set_transform(Matrix4::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2));

        assert_eq!(Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2).round(), n.round());
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n.round());
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45(){
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(Tuple::vector(1.0,1.0,0.0), r);
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface(){
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), r.round());
    }
}

