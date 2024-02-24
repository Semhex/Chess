use std::collections::HashMap;

use macroquad::prelude::*;

pub const NONE: usize = 0;
pub const PAWN: usize = 1;
pub const KNIGHT: usize = 2;
pub const BISHOP: usize = 3;
pub const ROOK: usize = 4;
pub const QUEEN: usize = 5;
pub const KING: usize = 6;

pub const WHITP: usize = 8;
pub const BLACP: usize = 16;

pub struct AssetManager {
    pub textures: HashMap<usize, Texture2D>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
    pub async fn load_assets(&mut self) {
        self.add_texture(WHITP, "assets/white_cell.png").await;
        self.add_texture(BLACP, "assets/black_cell.png").await;
        self.add_texture(WHITP + KING, "assets/pieces/white_king.png")
            .await;
        self.add_texture(WHITP + QUEEN, "assets/pieces/white_queen.png")
            .await;
        self.add_texture(WHITP + ROOK, "assets/pieces/white_rook.png")
            .await;
        self.add_texture(WHITP + BISHOP, "assets/pieces/white_bishop.png")
            .await;
        self.add_texture(WHITP + KNIGHT, "assets/pieces/white_knight.png")
            .await;
        self.add_texture(WHITP + PAWN, "assets/pieces/white_pawn.png")
            .await;
        self.add_texture(BLACP + KING, "assets/pieces/black_king.png")
            .await;
        self.add_texture(BLACP + QUEEN, "assets/pieces/black_queen.png")
            .await;
        self.add_texture(BLACP + ROOK, "assets/pieces/black_rook.png")
            .await;
        self.add_texture(BLACP + BISHOP, "assets/pieces/black_bishop.png")
            .await;
        self.add_texture(BLACP + KNIGHT, "assets/pieces/black_knight.png")
            .await;
        self.add_texture(BLACP + PAWN, "assets/pieces/black_pawn.png")
            .await;
    }
    pub async fn add_texture(&mut self, texture_name: usize, texture_path: &str) {
        match load_texture(texture_path).await {
            Ok(texture) => self.textures.insert(texture_name, texture),
            Err(_) => None,
        };
    }
    pub fn get_texture(&self, &texture_name: &usize) -> &Texture2D {
        self.textures.get(&texture_name).unwrap()
    }
        
}
