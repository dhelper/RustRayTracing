use crate::tuple::Tuple;
use crate::canvas::Canvas;
use crate::color::Color;
use std::io::Write;
use std::path::Path;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::matrix::Matrix4;
use std::f64::consts::PI;

mod tuple;
mod projectile;
mod environment;
mod color;
mod canvas;
mod matrix;
mod matrix_transformations;
mod ray;
mod sphere;
mod intersection;

fn write_position(c: &mut Canvas, t: Tuple) {
    let color = Color { red: 1.0, green: 0.0, blue: 0.0 };

    let round_x = t.x as usize;
    let round_y = t.y as usize;
    let fixed_y = c.height() - round_y;

    if round_x >= c.width() || fixed_y >= c.height() {
        println!("Position {:?} - is outside of canvas!!", t);
    } else {
        c.write_pixel(round_x, fixed_y, color);
    }
}

fn save_file(c: &mut Canvas, file_name: &str) {
    println!("Start creating file");
    let ppm = c.to_ppm();

    let path = Path::new(file_name);
    let display = path.display();

    let mut file = std::fs::File::create(&path).expect("create failed");

    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


fn main() {
    let canvas_pixels = 100;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::new();
    shape.set_transform(Matrix4::identity()
        .scale(0.5, 1.0, 1.0)
        .rotate_z(PI / 4.0)
    );
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z: f64 = 10.0;
    let wall_size = 7.0;
    let pixel_size: f64 = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_pixels {
        let y_64 = y as f64;
        let world_y = half - pixel_size * y_64;

        for x in 0..canvas_pixels {
            let x_f64 = x as f64;
            let world_x = -half + pixel_size * x_f64;

            let position = Tuple::point(world_x, world_y, wall_z);
            let direction = (position - ray_origin).normalize();

            let r = Ray {
                origin: ray_origin,
                direction,
            };
            let xs = r.intersect(shape);
            let result = xs.hit();
            if result.is_some() {
                write_position(&mut c, Tuple::point(x_f64, y_64, 0.0));
            }
        }
    }

    save_file(&mut c, "c:/temp/sphere1.ppm")
}

