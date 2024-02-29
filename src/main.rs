use std::vec;

use macroquad::prelude::*;
use macroquad::ui::root_ui;

const FORCE_MAGNITUDE: f32 = 0.1;
const MINIMUM_DISTANCE: f32 = 20.0;

#[derive(Clone)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    color: Color,
}
impl Particle {
    fn new(position: Vec2, velocity: Vec2, color: Color) -> Particle {
        let particle = Particle {
            position,
            velocity,
            color,
        };
        
        particle
    }
}

fn particle_move(particle : &mut Particle) {
    particle.position += particle.velocity; 
}
fn particle_gravity(particle : &mut Particle, particle2 : &mut Particle) {
    let direction = particle2.position - particle.position;
    let distance = direction.length();
    if distance > MINIMUM_DISTANCE {
        let force = (direction / distance) * FORCE_MAGNITUDE;
        particle.velocity += force;
        particle2.velocity -= force;
    }
}
fn particle_gravity_vector(particles: &mut Vec<Particle>) {
    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            let particle = &mut particles[i].clone();
            let particle2 = &mut particles[j].clone();
            new_particle_gravity(particle, particle2);
            particles[i] = particle.clone();
            particles[j] = particle2.clone();
        }
    }
}
fn particle_move_and_draw_vector(particles: &mut Vec<Particle>) {
    for i in 0..particles.len() {
        particle_move(&mut particles[i]);
        draw_circle(particles[i].position.x, particles[i].position.y, 5.0, particles[i].color);
    }
}
fn create_random_particle() -> Particle {
    let position = vec2(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height()));
    let velocity = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
    let color = Color::new(rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0), 1.0);
    Particle::new(position, velocity, color)
}
fn ui(particles: &mut Vec<Particle>) {
    root_ui().label(None, "hello megaui");
    if root_ui().button(None, "Push me") {
        let particle = create_random_particle();
        particles.push(particle);
    }
}
fn new_particle_gravity(particle : &mut Particle, particle2 : &mut Particle) {
    let direction = particle2.position - particle.position;
    let distance = direction.length();
    if distance < MINIMUM_DISTANCE {
        let force = (direction / distance) * FORCE_MAGNITUDE*20.0;
        particle.velocity -= force;
        particle2.velocity += force;
    }
    if distance > MINIMUM_DISTANCE {
        let force = (direction / distance) * FORCE_MAGNITUDE;
        particle.velocity += force;
        particle2.velocity -= force;
    }
}

#[macroquad::main("Particles")]
async fn main() {
    let mut particles = vec![
        Particle::new(vec2(500.0, 100.0), vec2(0.0, 0.3), WHITE),
        Particle::new(vec2(500.0, 300.0), vec2(0.0, 0.0), BLUE),
        Particle::new(vec2(700.0, 100.0), vec2(0.1, 0.0), GREEN),
        Particle::new(vec2(700.0, 300.0), vec2(0.0, 0.0), YELLOW),
    ];
    
    loop {
        clear_background(BLACK);
            if is_key_pressed(KeyCode::Escape) {
                break;}
            if is_key_pressed(KeyCode::Space) {
                let particle = create_random_particle();
                particles.push(particle);
            }
            
        
            
            particle_move_and_draw_vector(&mut particles);
            particle_gravity_vector(&mut particles);
            ui(&mut particles);
        
        
        next_frame().await
    }
}