extern crate rand;
extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics };
use graphics::{ Context };

use self::rand::Rng;

pub struct Biot {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub rotation: f64,
    pub drotation: f64,
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
        let square = rectangle::square(-25.0, -25.0, 50.0);
        let biotcolor = [1.0, 1.0, 1.0, 1.0];
        let rectangle = Rectangle::new(biotcolor);
        let line = [50.0, 50.0, 300.0, 300.0];
        let lineobj = Line::new(biotcolor, 1.0);
        let transform = c.transform.trans(self.x, self.y)
            .rot_rad(self.rotation);
        rectangle.draw(square, &c.draw_state, transform, gl);
        lineobj.draw(line, &c.draw_state, transform, gl);
    }
}
