use game::Game;
use macroquad::window::next_frame;


mod map;
mod ant;
mod game;
mod pos;
mod draw;


#[macroquad::main("Ant-IO")]
async fn main() {

    let mut game = Game::new();

    let mut won = 0;
    let mut lost = 0;

    loop {
        if game.update(won, lost) {
            if game.game_won {
                won += 1;
            } else {
                lost += 1;
            }
            game = Game::new();
        }
        next_frame().await;
    }
}
