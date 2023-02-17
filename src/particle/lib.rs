use macroquad::prelude::*;



#[derive(Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
    pub charge: f32,
    pub color: macroquad::color::Color,
    pub stationary: bool
}

impl Particle {
    pub fn x() {}
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, mass: f32, charge: f32, color: macroquad::color::Color, stationary: bool) -> Self {
        Self { x, y, vx, vy, mass, charge, color, stationary }
    }
    pub fn force(&self, other: &Particle) -> (f32, f32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let d_cubed = (dx * dx + dy * dy).powf(1.5);
        // avoiding infinite force
        if d_cubed == 0.0 { return (0.0, 0.0); }
        let fx = self.charge * other.charge * dx / d_cubed;
        let fy = self.charge * other.charge * dy / d_cubed;
        return (fx, fy);
    }
    pub fn update_vel(&mut self, fx: f32, fy: f32) {
        self.vx += (fx / self.mass).clamp(-1.0, 1.0);
        self.vy += (fy / self.mass).clamp(-1.0, 1.0);
    }
    pub fn draw(&self) {
        draw_circle(self.x, self.y, 3.0, self.color);
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub k: f32,
    pub friction_coef: f32,
}


impl ParticleSystem {
    const K: f32 = -10.0;
    const FRICTION_COEF: f32 = 1.00;
    pub fn new() -> Self {
        Self { particles: Vec::new(), k: ParticleSystem::K, friction_coef: ParticleSystem::FRICTION_COEF }
    }
    pub fn update_vel(&mut self) {
        let len = self.particles.len();
        if len == 0 { return; }
        for i in 0..len - 1 {
            for j in 1..len {
                let (fx, fy) = self.force(i, j);
                // fx = fx.clamp(-1.0, 1.0);
                // fy = fy.clamp(-1.0, 1.0);
                self.particles[i].update_vel(fx, fy);
                self.particles[j].update_vel(-fx, -fy);
            }
        }
    }
    pub fn update_pos(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) {
        let len = self.particles.len();
        for i in 0..len {
            let mut p = &mut self.particles[i];
            if p.stationary { continue; }
            p.x += p.vx;
            p.y += p.vy;
            p.vx *= self.friction_coef;
            p.vy *= self.friction_coef;
            if p.x < xmin || p.x > xmax {
                p.x = p.x.clamp(xmin, xmax);
                p.vx *= -0.5;
                p.vy *= 0.5;
            }
            if p.y < ymin || p.y > ymax {
                p.y = p.y.clamp(ymin, ymax);
                p.vy *= -0.5;
                p.vx *= 0.5;
            }
        }
    }
    pub fn draw(&self) {
        for p in &self.particles {
            p.draw();
        }
    }
    pub fn force(&self, i: usize, j: usize) -> (f32, f32) {
        let (fx, fy) = self.particles[i].force(&self.particles[j]);
        (self.k * fx, self.k * fy)
    }
    pub fn get_field(&self, x: f32, y: f32) -> (f32, f32) {
        let mut res: (f32, f32) = (0.0, 0.0);
        for p in &self.particles {
            let dx = p.x - x;
            let dy = p.y - y;
            let d_cubed = (dx * dx + dy * dy).powf(1.5);
            res.0 += p.charge * dx / d_cubed;
            res.1 += p.charge * dy / d_cubed;
        }
        res
    }
    pub fn draw_field(&self, xcount: u16, ycount: u16, width: f32, height: f32) {
        for i in 0..xcount {
            let x = width * (i + 1) as f32 / (xcount + 1) as f32;
            for j in 0..ycount {
                let y = height * (j + 1) as f32 / (ycount + 1) as f32;
                let (mut fx, mut fy) = self.get_field(x, y);
                let f = fx.hypot(fy);
                fx = 10.0 * fx / f;
                fy = 10.0 * fy / f;
                // fx = 200.0 * fx.clamp(-0.1, 0.1);
                // fy = 200.0 * fy.clamp(-0.1, 0.1);
                draw_line(x, y, x + fx, y + fy, 1.0, BLACK);
            }
        }
    }
    pub fn get_potential(&self, x: f32, y: f32) -> f32 {
        let mut res: f32 = 0.0;
        for p in &self.particles {
            let dx = p.x - x;
            let dy = p.y - y;
            let d = dx.hypot(dy);
            res += p.charge / d;
        }
        res
    }
    pub fn draw_potential(&self, xcount: u16, ycount: u16, width: f32, height: f32) {
        for i in 0..xcount {
            let x = width * (i + 1) as f32 / (xcount + 1) as f32;
            for j in 0..ycount {
                let y = height * (j + 1) as f32 / (ycount + 1) as f32;
                let pot = self.get_potential(x, y);
                let color = macroquad::color::Color::new(pot, 0.0, 255.0 - pot, 255.0);
                draw_rectangle(x, y, width / xcount as f32, height / ycount as f32, color);
            }
        }
    }
}

struct Player {
    pos: Vec2,
    //tex: Texture2D,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            //tex: load_texture("assets/player.png").unwrap(),
        }
    }
    fn draw(&self) {
        // draw_texture(self.tex, self.pos.x, self.pos.y, WHITE);
    }
}