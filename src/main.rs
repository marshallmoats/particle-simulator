#[allow(unused, dead_code)]
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gravity Simulation".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

// #[derive(Copy, Clone)]

fn draw_cursor(m: (f32, f32), t: f32) {
    draw_circle_lines(m.0, m.1, 20.0 * (t / 10.0).sin(), 1.0, BLACK);
}





use crate::particle::lib::Particle;
use crate::particle::lib::ParticleSystem;

pub mod particle;
#[macroquad::main(window_conf)]
async fn main() {
    let mut fps = fps_counter::FPSCounter::new();
    let mut t = 0;
    let mut ps = ParticleSystem::new();
    let mut protons = 0;
    let mut electrons = 0;
    //let x = Rect::new
    '_gameloop: loop {
        let m = mouse_position();
        let w = screen_width();
        let h = screen_height();
        
        if t % 2 == 0 {
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            ps.particles.push(Particle::new(m.0, m.1, 0.0, 0.0, 100.0, 10.0, RED, false));
            protons += 1;
        }
        if is_mouse_button_down(macroquad::input::MouseButton::Right) {
            ps.particles.push(Particle::new(m.0, m.1, 0.0, 0.0, 100.0, -10.0, BLUE, false));
            electrons += 1;
        }
        if is_mouse_button_down(macroquad::input::MouseButton::Middle) {
            ps.k *= -1.;
        }
        if is_key_down(macroquad::input::KeyCode::Backspace) {
            let p = ps.particles.pop();
            match p {
                Some(p) => {
                    if p.charge > 0.0 {
                        protons -= 1;
                    } else {
                        electrons -= 1;
                    }
                }
                None => {}
            }
        }
        if is_key_down(macroquad::input::KeyCode::Enter) {
            ps.particles.clear();
            protons = 0;
            electrons = 0;
        }
        //macroquad::input::mouse_wheel()
        if is_key_down(macroquad::input::KeyCode::Escape) {
            break;
        }
    }


        ps.update_vel();
        ps.update_pos(0.0, w, 0.0, h);

        clear_background(WHITE);
        ps.draw_field(50, 35, w, h);
        // ps.draw_potential(50, 35, w, h);
        ps.draw();
        draw_cursor(m, t as f32);
        draw_text(format!("FPS: {:?}", fps.tick()).as_str(), 50.0, 50.0, 30.0, BLACK);
        draw_text(format!("Protons: {protons} Electrons: {electrons}").as_str(), 50.0, 80.0, 30.0, BLACK);
        // draw_text(format!("Mouse wheel: {:?}", mouse_wheel()).as_str(), 30.0, 110.0, 30.0, BLACK);
        let (a, b) = mouse_wheel();
        if a != 0.0 || b != 0.0 {
            println!("{} {}", a, b);
        }
        t += 1;
        next_frame().await
    }
}



fn _quad(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let root1 = (-b - sqrt_disc) / 2.0;
    let root2 = (-b + sqrt_disc) / 2.0;
    Some((root1, root2))
}
