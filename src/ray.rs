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

    pub fn intersect(&self, sphere: Sphere) -> Vec<f64> {
        let sphere_to_ray = self.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminate = (b * b) - (4.0 * a * c);

        if discriminate < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminate.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminate.sqrt()) / (2.0 * a);

        return vec!(t1, t2);
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

    macro_rules! sphere_intersection_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, expected) = $value;

                let r = Ray {
                        origin: origin,
                        direction: Tuple::vector(0.0, 0.0, 1.0)
                };

                let s = Sphere::new();

                let actual = r.intersect(s);

                assert_eq!(expected, actual);
            }
            )*
        }
    }

    sphere_intersection_tests! {
        a_ray_intersects_a_sphere_at_two_points: (Tuple::point(0.0, 0.0, -5.0), vec!(4.0, 6.0)),
        a_ray_intersects_a_sphere_at_a_tangent: (Tuple::point(0.0, 1.0, -5.0), vec!(5.0, 5.0)),
        a_ray_misses_a_sphere: (Tuple::point(0.0, 2.0, -5.0), Vec::<f64>::new()),
        a_ray_originates_inside_a_sphere: (Tuple::point(0.0, 0.0, 0.0), vec!(-1.0, 1.0)),
    }
}