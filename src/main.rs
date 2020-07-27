mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;
use draw::*;
use game::Game;

const BACK_COLOR: Color = [0.6, 0.6, 0.6, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACK_COLOR, graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
