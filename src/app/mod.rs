extern crate opengl_graphics;
mod biot;

use opengl_graphics::{ GlGraphics };
use piston::input::*;

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
        use graphics::*;
        let square = rectangle::square(-25.0, -25.0, 50.0);
        let biots = self.biots.iter();
        let bgcolor = [17.0 / 255.0, 46.0 / 255.0, 106.0 / 255.0, 1.0];
        let biotcolor = [1.0, 1.0, 1.0, 1.0];
        let rectangle = Rectangle::new(biotcolor);
        let line = [50.0, 50.0, 300.0, 300.0];
        let lineobj = Line::new(biotcolor, 1.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(bgcolor, gl);
            for b in biots {
                let transform = c.transform.trans(b.x, b.y)
                    .rot_rad(b.rotation);
                rectangle.draw(square, &c.draw_state, transform, gl);
                lineobj.draw(line, &c.draw_state, transform, gl);
            }
        });
    }
}
