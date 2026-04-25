use crate::data::BuildingKind;
use crate::game::{Game, Screen, Tile};

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 270;

const BLACK: u32 = 0x0f1117;
const PANEL: u32 = 0x202632;
const BASE: u32 = 0x2f5d50;
const GROUND: u32 = 0x33443a;
const FOREST: u32 = 0x2f7d43;
const STONE: u32 = 0x777d86;
const ENEMY: u32 = 0xa84040;
const RUIN: u32 = 0x8a6f3d;
const PLAYER: u32 = 0xf0d060;
const CARD: u32 = 0xd7c59a;

pub struct Renderer {
    buffer: Vec<u32>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn draw(&mut self, game: &Game) {
        self.clear(BLACK);
        match game.screen {
            Screen::MainMenu => self.draw_menu(),
            Screen::Base => self.draw_base(game),
            Screen::Expedition => self.draw_expedition(game),
            Screen::GameOver => self.draw_game_over(),
        }
    }

    fn draw_menu(&mut self) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x1a2028);
        self.rect(80, 60, 320, 120, PANEL);
        self.rect(110, 90, 260, 18, 0x9eb3c7);
        self.rect(110, 126, 200, 12, CARD);
        self.rect(110, 148, 160, 12, CARD);
    }

    fn draw_game_over(&mut self) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x22181b);
        self.rect(90, 82, 300, 100, 0x5c2731);
        self.rect(120, 118, 240, 20, 0xe06c75);
    }

    fn draw_base(&mut self, game: &Game) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x17201d);
        self.rect(330, 0, 150, HEIGHT, PANEL);

        let tile_size = 10;
        let origin_x = 20;
        let origin_y = 20;

        for y in 0..game.base.size {
            for x in 0..game.base.size {
                self.rect(
                    origin_x + x as usize * tile_size,
                    origin_y + y as usize * tile_size,
                    tile_size - 1,
                    tile_size - 1,
                    BASE,
                );
            }
        }

        for building in &game.base.buildings {
            let color = match building.kind {
                BuildingKind::Campfire => 0xd87f35,
                BuildingKind::Storehouse => 0x9b7653,
                BuildingKind::Workbench => 0x88a65e,
                BuildingKind::ResearchTable => 0x6b8fd6,
                BuildingKind::TrainingDummy => 0xc47b6b,
            };

            self.rect(
                origin_x + building.x as usize * tile_size,
                origin_y + building.y as usize * tile_size,
                tile_size - 1,
                tile_size - 1,
                color,
            );
        }

        self.draw_status_bars(game);
    }

    fn draw_expedition(&mut self, game: &Game) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x12181d);
        self.rect(0, 206, WIDTH, 64, PANEL);

        let Some(expedition) = &game.expedition else {
            return;
        };

        let tile_size = 6;
        let origin_x = 20;
        let origin_y = 10;

        for y in 0..expedition.map.height {
            for x in 0..expedition.map.width {
                let color = match expedition.map.tiles[(y * expedition.map.width + x) as usize] {
                    Tile::Ground => GROUND,
                    Tile::Forest => FOREST,
                    Tile::Stone => STONE,
                    Tile::Enemy => ENEMY,
                    Tile::Ruin => RUIN,
                };

                self.rect(
                    origin_x + x as usize * tile_size,
                    origin_y + y as usize * tile_size,
                    tile_size - 1,
                    tile_size - 1,
                    color,
                );
            }
        }

        self.rect(
            origin_x + expedition.player_x as usize * tile_size,
            origin_y + expedition.player_y as usize * tile_size,
            tile_size - 1,
            tile_size - 1,
            PLAYER,
        );

        for (index, card) in expedition.hand.iter().enumerate() {
            let x = 20 + index * 54;
            let h = 34 + card.cost as usize * 4;
            self.rect(x, 220, 42, h.min(44), CARD);
        }
    }

    fn draw_status_bars(&mut self, game: &Game) {
        let alive = game
            .characters
            .iter()
            .filter(|character| character.alive)
            .count();
        self.rect(
            348,
            24,
            (game.resources.wood.max(0) as usize * 4).min(100),
            10,
            0x8fbc8f,
        );
        self.rect(
            348,
            46,
            (game.resources.stone.max(0) as usize * 4).min(100),
            10,
            0x9aa5b1,
        );
        self.rect(
            348,
            68,
            (game.resources.food.max(0) as usize * 4).min(100),
            10,
            0xd6b15f,
        );
        self.rect(348, 98, alive * 28, 10, 0xd46a6a);
    }

    fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let max_x = (x + width).min(WIDTH);
        let max_y = (y + height).min(HEIGHT);

        for py in y..max_y {
            for px in x..max_x {
                self.buffer[py * WIDTH + px] = color;
            }
        }
    }
}
