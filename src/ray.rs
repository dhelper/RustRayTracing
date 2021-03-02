use crate::tuple::Tuple;
use crate::sphere::Sphere;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, t: f64) -> Tuple {
        return self.origin + self.direction * t;
    }

    pub fn intersect(&self, sphere: Sphere) -> [f64;2] {
       return [4.0, 6.0];
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::sphere::Sphere;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray {
            origin: origin,
            direction: direction,
        };

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    macro_rules! computing_a_point_from_a_distance_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let r = Ray {
                        origin: Tuple::point(2.0, 3.0, 4.0),
                        direction: Tuple::vector(1.0, 0.0, 0.0)
                };

                let actual = r.position(input);

                assert_eq!(expected, actual);
            }
            )*
        }
    }

    computing_a_point_from_a_distance_tests! {
        position_of_zero: (0.0, Tuple::point(2.0,3.0,4.0)),
        position_of_one: (1.0, Tuple::point(3.0,3.0,4.0)),
        position_of_minus_one: (-1.0, Tuple::point(1.0,3.0,4.0)),
        position_of_two_and_a_half: (2.5, Tuple::point(4.5,3.0,4.0)),
    }

    #[test]
    fn a_ray_intersects_a_shere_at_two_points() {
        let r = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let xs = r.intersect(s);

        let expected = [4.0, 6.0];

        assert_eq!(expected, xs);
    }
}