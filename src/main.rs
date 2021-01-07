use crate::projectile::Projectile;
use crate::tuple::Tuple;
use crate::environment::Environment;

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

fn main() {
    let p = Projectile {
        position: Tuple::point(0.0, 5.0, 0.0),
        velocity: Tuple::vector(1.0, 2.0, 0.0),
    };

    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut curr_proj = p;


    println!("starting projectile {:?}", p);
    println!("Environment {:?}", e);
    println!("---------------------------------------------");
    while curr_proj.position.y > 0.0 {
        curr_proj = tick(&e, &curr_proj);
        println!("{:?}", curr_proj);
    }
}
