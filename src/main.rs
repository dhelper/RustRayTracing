use crate::projectile::Projectile;
use crate::tuple::Tuple;
use crate::environment::Environment;
use crate::canvas::Canvas;
use crate::color::Color;
use std::io::Write;
use std::path::Path;
use crate::matrix::Matrix4;
use std::f64::consts::PI;

mod tuple;
mod projectile;
mod environment;
mod color;
mod canvas;
mod matrix;
mod matrix_transformations;

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile {
        position,
        velocity,
    };
}

fn write_position(c: &mut Canvas, t: Tuple) {
    let color = Color { red: 0.0, green: 1.0, blue: 0.0 };

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
    let mut c = Canvas::new(200, 200);

    let r = 200.0 * 3.0 / 8.0;
    let twelve = Tuple::point(0.0, 1.0, 0.0);

    for hour in 0..12{
        let p = Matrix4::identity()
            .rotate_z(f64::from(hour) * PI / 6.0)
            .scale(r, r, 0.0)
            .translate(100.0, 100.0, 100.0)
            * twelve;

        write_position(&mut c, p);
    }

    save_file(&mut c, "c:/temp/clock.ppm")
}

