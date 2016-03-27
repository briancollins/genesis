extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use piston::input::*;
use graphics;

mod biot;

pub struct App {
    gl: GlGraphics,
    biots: Vec<biot::Biot>
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        let mut app = App {
            gl: gl,
            biots: Vec::new()
        };

        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app.biots.push(biot::Biot::new());
        app
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for mut b in &mut self.biots {
            b.tick();
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let biots = self.biots.iter();
        let bgcolor = [17.0 / 255.0, 46.0 / 255.0, 106.0 / 255.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(bgcolor, gl);
            for b in biots {
                b.draw(c, gl);
            }
        });
    }
}
