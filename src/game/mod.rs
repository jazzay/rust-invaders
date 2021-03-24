use std::{sync::{Arc, Mutex}, time::Duration};

use crate::Vector2;
use nannou::prelude::*;
use winit::event::VirtualKeyCode;

pub mod math;

// Let's multi thread this!
//
// 1. Traditional = Mutex some data/code so we can share between threads
// 2. Message Passing = reduces locking

pub trait InputHandler {
    fn key_down(&mut self, key: VirtualKeyCode);
    fn key_up(&mut self, key: VirtualKeyCode);
}

pub trait Game : InputHandler {
    fn initialize(&mut self);
    fn update(&mut self, delta_time: f32);
    fn render(&self, draw: &nannou::draw::Draw);
}

// How do we protect a shared state in each language?
// C++
// class 
// {
//     Sim mySimulation;
//     Mutex Lock;
// };
//
// Rust:
// struct Sim {
//     sim: Mutex<Sim> 
// }

pub struct InvadersGameInner {
    enemy_positions: Vec<Vector2>,
    player_position: Vector2,
    player_velocity: Vector2,
}

impl InvadersGameInner {
    pub fn new() -> Self {
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
    
        Self {
            enemy_positions,
            player_position,
            player_velocity
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player_position = self.player_position + (self.player_velocity * delta_time);
    }
}

pub struct InvadersGame {
    sim: Arc<Mutex<InvadersGameInner>>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl InvadersGame {
    pub fn new_game() -> Self {
        InvadersGame {
            sim: Arc::new(Mutex::new(InvadersGameInner::new())),
            thread_handle: None,
        }
    }

    pub fn get_status(&self) {
        println!("status");
    }

    // This function illustrates an api (join) that forces us to hand over ownership of self
    // since the member thread_handle requires that, so does the outer wrapper self
    // but since the handle is wrapped in an Option we could instead take() the handle
    // out of the Option as well
    fn shutdown(self) {
        self.thread_handle.unwrap().join().unwrap();
    }
}

impl Game for InvadersGame {
    fn initialize(&mut self) {
        let sim_copy = self.sim.clone();

        self.thread_handle = Some(std::thread::spawn(move || {
            loop {
                // Can we see why this code would be problematic?
                // let mut sim = sim_copy.lock().unwrap();
                // sim.update(30.0);
                sim_copy.lock().unwrap().update(30.0);

                std::thread::sleep(Duration::from_millis(2000));
                println!("Sim thread");
            }
        }));
    }

    fn update(&mut self, _delta_time: f32) {
        // todo: how should we get the inputs to the thread?
        // self.player_position = self.player_position + (self.player_velocity * delta_time);
    }

    fn render(&self, draw: &nannou::draw::Draw) {
        // Clear the background to purple.
        draw.background().color(PLUM);

        let sim = self.sim.lock().unwrap();

        for epos in &sim.enemy_positions {
            draw.ellipse().x(epos.x).y(epos.y).color(YELLOW);
        }

        // Draw a blue ellipse with default size and position.
        let pos = sim.player_position;
        draw.ellipse().x(pos.x).y(pos.y).color(STEELBLUE);
    }
}

impl InputHandler for InvadersGame {
    fn key_down(&mut self, key: VirtualKeyCode) {
        let mut sim = self.sim.lock().unwrap();
        match key {
            VirtualKeyCode::Left => {
                sim.player_velocity.x = -200.0;
            }
            VirtualKeyCode::Right => {
                sim.player_velocity.x = 200.0;
            }
            _ => { }
        }
    }

    fn key_up(&mut self, key: VirtualKeyCode) {
        let mut sim = self.sim.lock().unwrap();
        match key {
            VirtualKeyCode::Left => {
                sim.player_velocity.x = 0.0;
            }
            VirtualKeyCode::Right => {
                sim.player_velocity.x = 0.0;
            }
            _ => { }
        }
    }
}


