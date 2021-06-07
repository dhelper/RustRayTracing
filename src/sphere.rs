use std::sync::atomic::{AtomicUsize, Ordering};
use crate::matrix::Matrix4;

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
}


#[cfg(test)]
mod tests {
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
}

