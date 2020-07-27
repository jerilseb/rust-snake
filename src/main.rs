mod draw;
mod game;
mod snake;

use draw::*;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.6, 0.6, 0.6, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font = assets.join("OpenSans-Regular.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACK_COLOR, graphics);
            glyphs.factory.encoder.flush(_device);

            game.draw(&context, graphics, &mut glyphs);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
