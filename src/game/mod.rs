use crate::Vector2;
use nannou::prelude::*;
use winit::event::VirtualKeyCode;

pub mod math;

pub trait InputHandler {
    fn key_down(&mut self, key: VirtualKeyCode);
    fn key_up(&mut self, key: VirtualKeyCode);
}

pub trait Game : InputHandler {
    fn initialize(&mut self);
    fn update(&mut self, delta_time: f32);
    fn render(&self, draw: &nannou::draw::Draw);
}

pub struct InvadersGame {
    enemy_positions: Vec<Vector2>,
    player_position: Vector2,
    player_velocity: Vector2,
}

impl InvadersGame {
    pub fn new_game() -> Self {
        let enemy_positions = vec![
            Vector2::new(-300.0, 200.0),
            Vector2::new(0.0, 200.0),
            Vector2::new(300.0, 200.0),
        ];
    
        let player_position = Vector2 {
            x: 0.0,
            y: -200.0
        };
        let player_velocity = Vector2::zero();
    
        InvadersGame {
            enemy_positions,
            player_position,
            player_velocity
        }
    }

    pub fn get_status(&self) {
        println!("status");
    }
}

impl Game for InvadersGame {
    fn initialize(&mut self) {
    }

    fn update(&mut self, delta_time: f32) {
        self.player_position = self.player_position + (self.player_velocity * delta_time);
    }

    fn render(&self, draw: &nannou::draw::Draw) {
        // Clear the background to purple.
        draw.background().color(PLUM);

        for epos in &self.enemy_positions {
            draw.ellipse().x(epos.x).y(epos.y).color(YELLOW);
        }

        // Draw a blue ellipse with default size and position.
        let pos = self.player_position;
        draw.ellipse().x(pos.x).y(pos.y).color(STEELBLUE);


    }
}

impl InputHandler for InvadersGame {
    fn key_down(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Left => {
                self.player_velocity.x = -200.0;
            }
            VirtualKeyCode::Right => {
                self.player_velocity.x = 200.0;
            }
            _ => { }
        }
    }

    fn key_up(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Left => {
                self.player_velocity.x = 0.0;
            }
            VirtualKeyCode::Right => {
                self.player_velocity.x = 0.0;
            }
            _ => { }
        }
    }
}


