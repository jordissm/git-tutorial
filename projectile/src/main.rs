extern crate gnuplot;

use gnuplot::{Caption, Color, Figure, Fix, AxesCommon};
use nalgebra::{Vector2};

struct Particle {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
}

impl Particle {
    fn new(position: Vector2<f64>, velocity: Vector2<f64>, acceleration: Vector2<f64>) -> Particle {
        Particle {
            position,
            velocity,
            acceleration,
        }
    }

    fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt + 0.5 * self.acceleration * dt.powi(2);
        self.velocity += self.acceleration * dt;
    }
}

fn main() {
    let mut particle = Particle::new(Vector2::new(0.0, 10.0), 
                                     Vector2::new(6.0, 10.0), 
                                     Vector2::new(0.0, -9.81));
    let dt = 0.01;
    let mut x = Vec::new();
    let mut y = Vec::new();

    while particle.position.y > 0.0 {
        particle.update(dt);
        x.push(particle.position.x);
        y.push(particle.position.y);
    }

    let mut fg = Figure::new();
    fg.axes2d()
        //.set_title("Particle Position", &[])
        .set_x_label("x (m)", &[])
        .set_y_label("y (m)", &[])
        .set_x_range(Fix(-0.0), Fix(20.0))
        .set_y_range(Fix(-0.0), Fix(20.0))
        .set_x_axis(true, &[])
        .set_y_axis(true, &[])
        .lines(&x, &y, &[Caption("Position"), Color("blue")]);

    fg.show().unwrap();
}