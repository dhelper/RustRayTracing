use std::ops::{Index, IndexMut};
use crate::sphere::Sphere;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

pub struct Intersections {
    pub(crate) values: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        return Intersections { values: Vec::new() };
    }
    pub fn len(&self) -> usize {
        return self.values.len();
    }

    pub fn hit(&self) -> Option<Intersection> {
        return self.values.clone().into_iter()
            .filter(|f| f.t > 0.0)
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Intersections {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::{Intersection, Intersections};
    use crate::sphere::Sphere;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection {
            t: 3.5,
            object: s,
        };

        assert_eq!(3.5, i.t);
        assert_eq!(s, i.object)
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection {
            t: 1.0,
            object: s,
        };

        let i2 = Intersection {
            t: 2.0,
            object: s,
        };

        let xs = Intersections { values: vec!(i1, i2) };

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    #[test]
    fn intersects_sets_the_object_on_the_intersection() {
        let origin = Tuple::point(0.0, 0.0, -5.0);
        let direction = Tuple::vector(0.0, 0.0, 1.0);
        let ray = Ray {
            origin,
            direction,
        };

        let s = Sphere::new();

        let result = ray.intersect(s);

        assert_eq!(2, result.len());
        assert_eq!(s, result[0].object);
        assert_eq!(s, result[0].object);
    }

    #[test]
    fn the_hit_when_all_interactions_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: s };
        let i2 = Intersection { t: 2.0, object: s };

        let xs = Intersections { values: vec!(i1, i2) };

        let actual = xs.hit();

        assert_eq!(Some(i1), actual);
    }

    #[test]
    fn the_hit_when_some_interactions_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection { t: -1.0, object: s };
        let i2 = Intersection { t: 1.0, object: s };

        let xs = Intersections { values: vec!(i1, i2) };

        let actual = xs.hit();

        assert_eq!(Some(i2), actual);
    }

    #[test]
    fn the_hit_when_all_interactions_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection { t: -2.0, object: s };
        let i2 = Intersection { t: -1.0, object: s };

        let xs = Intersections { values: vec!(i1, i2) };

        let actual = xs.hit();

        assert_eq!(None, actual);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection { t: 5.0, object: s };
        let i2 = Intersection { t: 7.0, object: s };
        let i3 = Intersection { t: -3.0, object: s };
        let i4 = Intersection { t: 2.0, object: s };

        let xs = Intersections { values: vec!(i1, i2, i3, i4) };

        let actual = xs.hit();

        assert_eq!(Some(i4), actual);
    }
}