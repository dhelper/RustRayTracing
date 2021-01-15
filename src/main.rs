use crate::projectile::Projectile;
use crate::tuple::Tuple;
use crate::environment::Environment;
use crate::canvas::Canvas;
use crate::color::Color;
use std::io::Write;
use std::path::Path;

mod tuple;
mod projectile;
mod environment;
mod color;
mod canvas;

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
    }else {
        c.write_pixel(round_x, fixed_y, color);
    }
}


fn main() {
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let p = Projectile { position: start, velocity };

    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);

    let mut curr_proj = p;
    println!("starting projectile {:?}", p);
    println!("Environment {:?}", e);
    println!("---------------------------------------------");
    write_position(&mut c, curr_proj.position);
    while curr_proj.position.y > 0.0 {
        curr_proj = tick(&e, &curr_proj);
        println!("{:?}", curr_proj);
        write_position(&mut c, curr_proj.position);
    }
    println!("Start creating file");
    let ppm = c.to_ppm();

    let path = Path::new("c:/temp/plot.ppm");
    let display = path.display();

    let mut file = std::fs::File::create(&path).expect("create failed");

    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
