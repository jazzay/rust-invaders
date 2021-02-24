#![allow(dead_code)]

use nannou::prelude::*;

mod game;
use game::Game;
use game::math::Vector2;

fn main() {
    let player = Vector2 {
        x: 100.0,
        y: 100.0
    };
    let player_velocity = Vector2::zero();

    let enemy_positions = vec![
        Vector2::new(100.0, 100.0),
        Vector2::new(100.0, 100.0),
        Vector2::new(100.0, 100.0),
    ];

    println!("Enemies: {:?}", enemy_positions);

    let pos2 = player + player_velocity;
    println!("pos2: {:?}", pos2);

    nannou_main();
}

fn nannou_main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Game {
    Game::new()
}

fn event(_app: &App, _model: &mut Game, _event: Event) {
}

fn update(_app: &App, _model: &mut Game, _update: Update) {
}

fn view(app: &App, _model: &Game, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with default size and position.
    draw.ellipse().color(STEELBLUE);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();    
}
