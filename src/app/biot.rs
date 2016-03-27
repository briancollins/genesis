extern crate rand;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Context;

use self::rand::Rng;
use std::f64::consts::PI;
use std::f64;

pub struct Biot {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    rotation: f64,
    drotation: f64,
    segments: u8,
}

impl Biot {
    pub fn new() -> Biot {
        let mut rng = rand::thread_rng();
        Biot {
            x: rng.gen_range(0.0, 1024.0),
            y: rng.gen_range(0.0, 768.0),
            dx: rng.gen_range(-0.5, 0.5),
            dy: rng.gen_range(-0.5, 0.5),
            rotation: 0.0,
            drotation: rng.gen_range(-0.05, 0.05),
            segments: 6
        }
    }

    pub fn tick(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.rotation += self.drotation;
        if self.x > 1024.0 || self.x < 0.0 {
            self.dx = -self.dx;
        }

        if self.y > 768.0 || self.y < 0.0 {
            self.dy = -self.dy;
        }
    }

    pub fn draw(&self, c: Context, gl: &mut GlGraphics) {
        use graphics::*;
        let biotcolor = [1.0, 1.0, 1.0, 1.0];
        let lineobj = Line::new(biotcolor, 0.5);
        let transform = c.transform.trans(self.x, self.y)
            .rot_rad(self.rotation);

        for n in 0..self.segments {
            let fraction = n as f64 / self.segments as f64;
            let line = [
                0.0, 0.0,
                (PI * 2.0 * fraction).sin() * 10.0,
                (PI * 2.0 * fraction).cos() * 10.0];
            lineobj.draw(line, &c.draw_state, transform, gl);
        }
    }
}
