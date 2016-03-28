extern crate rand;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use graphics::Context;

use self::rand::Rng;
use std::f64::consts::PI;

const SEGMENT_MAX: usize = 10;
const LIFESPAN_MAX: u32 = 50000;

pub struct Biot {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    rotation: f64,
    drotation: f64,
    segment_count: u8,
    segments: [[f64; 4]; SEGMENT_MAX],
    pub lifespan: u32
}

impl Biot {
    pub fn new() -> Biot {
        let mut rng = rand::thread_rng();
        let mut biot = Biot {
            x: rng.gen_range(0.0, 1024.0),
            y: rng.gen_range(0.0, 768.0),
            dx: rng.gen_range(-0.5, 0.5),
            dy: rng.gen_range(-0.5, 0.5),
            rotation: 0.0,
            drotation: rng.gen_range(-0.05, 0.05),
            segment_count: rng.gen_range(2, SEGMENT_MAX as u8),
            segments: [[0.0; 4]; SEGMENT_MAX],
            lifespan: rng.gen_range(1, LIFESPAN_MAX),
        };
        biot.generate_lines();
        biot
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

        self.lifespan -= 1;
    }

    pub fn generate_lines(&mut self) {
        let line_length = 20.0;

        for n in 0..self.segment_count {
            let fraction = n as f64 / self.segment_count as f64;
            let line = [
                0.0, 0.0,
                (PI * 2.0 * fraction).sin() * line_length,
                (PI * 2.0 * fraction).cos() * line_length];
            self.segments[n as usize] = line;
        }
    }

    pub fn draw(&self, c: Context, gl: &mut GlGraphics) {
        use graphics::*;
        let biotcolor = [1.0, 1.0, 1.0, 1.0];
        let line = Line::new(biotcolor, 0.5);
        let transform = c.transform.trans(self.x, self.y)
            .rot_rad(self.rotation);

        for n in 0..self.segment_count {
            line.draw(self.segments[n as usize], &c.draw_state, transform, gl);
        }
    }
}
