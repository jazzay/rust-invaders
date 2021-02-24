use crate::Vector2;

pub mod math;

pub struct Game {
    enemy_positions: Vec<Vector2>
}

impl Game {
    pub fn new() -> Self {
        let enemy_positions = vec![
            Vector2::new(100.0, 100.0),
            Vector2::new(100.0, 100.0),
            Vector2::new(100.0, 100.0),
        ];
    
        Game {
            enemy_positions
        }
    }
}