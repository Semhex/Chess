use macroquad::prelude::*;

use board::*;

mod board;

#[macroquad::main("Chess")]
async fn main() {
    let mut new_board = Board::new();

    new_board.ready().await;

    loop {
        clear_background(BACKGROUND);
        set_camera(&Camera2D {
            zoom: vec2(10. / screen_width(), 10. / screen_height()),
            target: vec2(BOARD_SIZE.0 * 0.5, BOARD_SIZE.1 * 0.5),
            ..Default::default()
        });
        new_board.draw_board();
        new_board.draw_pieces();
        next_frame().await
    }
}
