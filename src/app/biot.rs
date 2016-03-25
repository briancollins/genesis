extern crate rand;

use self::rand::Rng;

pub struct Biot {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub rotation: f64,
    pub drotation: f64
}

impl Biot {
    pub fn new() -> Biot {
        let mut rng = rand::thread_rng();
        Biot {
            x: rng.gen_range(0.0, 1024.0),
            y: rng.gen_range(0.0, 768.0),
            dx: rng.gen_range(-0.1, 0.1),
            dy: rng.gen_range(-0.1, 0.1),
            rotation: 0.0,
            drotation: rng.gen_range(-0.1, 0.1),
        }
    }

    pub fn tick(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.rotation += self.drotation;
    }
}
