use macroquad::prelude::*;

use chess::asset_manager::*;

pub const _START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_FEN: &str = "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 w - 0 1";

pub const CELL_SIZE: (f32, f32) = (16., 12.);
pub const PIECE_TEXTURE_SIZE: (f32, f32) = (16., 30.);
pub const GRID_SIZE: (f32, f32) = (8., 8.);
pub const BOARD_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

pub const BACKGROUND: Color = Color::new(0.1, 0.1, 0.1, 1.0);
pub const TRANSPARENT_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.0);

pub struct Board {
    grid: Vec<usize>,
    assets: AssetManager,
    texture: Texture2D,
    pieces: Vec<Texture2D>,

    fen: String,
}
impl Board {
    pub fn new() -> Self {
        Self {
            grid: vec![NONE; (GRID_SIZE.0 * GRID_SIZE.1) as usize],
            assets: AssetManager::new(),
            texture: Texture2D::from_image(&Image::gen_image_color(
                BOARD_SIZE.0 as u16,
                BOARD_SIZE.1 as u16,
                BACKGROUND,
            )),
            pieces: vec![Texture2D::empty(); GRID_SIZE.1 as usize + 1],
            fen: TEST_FEN.to_string(),
        }
    }
    pub async fn ready(&mut self) {
        self.pieces[GRID_SIZE.0 as usize] =Texture2D::from_image(&Image::gen_image_color(
            PIECE_TEXTURE_SIZE.0 as u16,
            PIECE_TEXTURE_SIZE.1 as u16,
            BACKGROUND,
        ));
        self.assets.load_assets().await;
        self.convert_fen();
        self.calculate_board_texture();
        self.calculate_piece_texture();
    }
    pub fn draw_board(&self) {
        draw_texture(&self.texture, 0., 0., WHITE);
    }
    pub fn draw_pieces(&self) {
        for part in 0..GRID_SIZE.0 as usize {
            draw_texture(
                &self.pieces[part],
                0.,
                CELL_SIZE.1 * part as f32 - 18.,
                WHITE,
            );
        }
        if self.pieces[GRID_SIZE.0 as usize] != Texture2D::empty() {
            draw_texture(&self.pieces[GRID_SIZE.0 as usize], 0., 0., TRANSPARENT_COLOR);
        }
    }
    fn calculate_piece_texture(&mut self) {
        for i in 0..GRID_SIZE.0 as usize {
            self.pieces[i] = Texture2D::from_image(&Image::gen_image_color(
                PIECE_TEXTURE_SIZE.0 as u16,
                PIECE_TEXTURE_SIZE.1 as u16,
                TRANSPARENT_COLOR,
            ));
            for j in 0..GRID_SIZE.1 as usize {
                let index = self.calculate_grid_pos(i, j);
                if self.grid[index] == 0 {
                    continue;
                }
                let piece_img = self
                    .assets
                    .get_texture(&self.grid[index])
                    .get_texture_data();
                self.pieces[i].update_part(
                    &piece_img,
                    PIECE_TEXTURE_SIZE.0 as i32 * j as i32,
                    0,
                    piece_img.width() as i32,
                    piece_img.height() as i32,
                )
            }
            self.pieces[i as usize].set_filter(FilterMode::Nearest);
        }
    }
    fn calculate_board_texture(&mut self) {
        let white_cell = self.assets.get_texture(&WHITP);
        let black_cell = self.assets.get_texture(&BLACP);

        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                let cell = if ((i + j) % 2) == 0 {
                    white_cell
                } else {
                    black_cell
                };
                self.texture.update_part(
                    &cell.get_texture_data(),
                    (CELL_SIZE.0 * i as f32) as i32,
                    (CELL_SIZE.1 * j as f32) as i32,
                    (CELL_SIZE.0) as i32,
                    (CELL_SIZE.1) as i32,
                );
            }
        }
        self.texture.set_filter(FilterMode::Nearest);
    }
    fn convert_fen(&mut self) {
        let fen_parts: Vec<&str> = self.fen.split(' ').collect();
        let pos: &str = &fen_parts[0];
        let mut index: usize = 0;
        for i in pos.chars() {
            let mut piece_typ: usize = 0;
            match i.to_ascii_lowercase() {
                '/' => continue,
                'k' => piece_typ = KING,
                'q' => piece_typ = QUEEN,
                'r' => piece_typ = ROOK,
                'b' => piece_typ = BISHOP,
                'n' => piece_typ = KNIGHT,
                'p' => piece_typ = PAWN,
                _ => {
                    if (i as usize - 48) < 9 {
                        index += i as usize - 48;
                    }
                    continue;
                }
            }
            let piece_color: usize = if i.is_ascii_uppercase() { WHITP } else { BLACP };
            piece_typ += piece_color;
            self.grid[index] = piece_typ;
            index += 1;
        }
    }
    fn calculate_grid_pos(&self, pos1: usize, pos2: usize) -> usize {
        (pos1 * GRID_SIZE.0 as usize) + pos2
    }
}
