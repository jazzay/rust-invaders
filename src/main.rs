#![allow(dead_code)]

use nannou::prelude::*;

mod game;
use game::*;
//use game::math::Vector2;

fn main() {
    nannou_main();
}

fn nannou_main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> InvadersGame {
    let mut game = InvadersGame::new_game();
    game.initialize();
    game
}

fn event(_app: &App, game: &mut InvadersGame, event: Event) {
    match event {
        Event::WindowEvent {simple, ..} => {
            match simple {
                Some(value) => {
                    match value {
                        KeyPressed(key) => { game.key_down(key); }
                        KeyReleased(key) => { game.key_up(key); }
                        MouseMoved(_pos) => {}
                        MousePressed(_button) => {}
                        MouseReleased(_button) => {}
                        _ =>  { }
                    }
                }
                None => {
                }
            }
        }
        _ => { }
    }
}

fn update(_app: &App, game: &mut InvadersGame, update: Update) {
    game.update(update.since_last.as_secs_f32());
}

fn view(app: &App, game: &InvadersGame, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    game.render(&draw);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();    
}
