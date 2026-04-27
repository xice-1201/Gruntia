use crate::card::{Card, CardKind};
use crate::data::BuildingKind;
use crate::game::{building_cost, DepartBlockReason, Game, Screen, Tile, MAX_PREP_CARDS};
use crate::save;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const BASE_VIEWPORT_WIDTH: usize = WIDTH - 280;
pub const BASE_VIEWPORT_HEIGHT: usize = HEIGHT - 24;
pub const EXPEDITION_VIEWPORT_HEIGHT: usize = HEIGHT - 132;
pub const EXPEDITION_HAND_Y: usize = HEIGHT - 112;

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
const TEXT: u32 = 0xe8edf2;
const MUTED_TEXT: u32 = 0x9aa5b1;
const WARNING: u32 = 0xe6b450;

const NEW_GAME_LABEL: &[u8] = include_bytes!("../assets/ui/new_game.label");
const LOAD_GAME_LABEL: &[u8] = include_bytes!("../assets/ui/load_game.label");
const QUIT_LABEL: &[u8] = include_bytes!("../assets/ui/quit.label");
const LOAD_SAVE_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/load_save_button.label");
const RESTART_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/restart_button.label");
const MAIN_MENU_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/main_menu_button.label");
const DEATH_FOOD_LABEL: &[u8] = include_bytes!("../assets/ui/death_food.label");
const DEATH_CHARACTERS_LABEL: &[u8] = include_bytes!("../assets/ui/death_characters.label");
const BASE_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/base_title.label");
const WOOD_LABEL: &[u8] = include_bytes!("../assets/ui/wood.label");
const STONE_LABEL: &[u8] = include_bytes!("../assets/ui/stone.label");
const FOOD_LABEL: &[u8] = include_bytes!("../assets/ui/food.label");
const SIZE_LABEL: &[u8] = include_bytes!("../assets/ui/size.label");
const BUILDINGS_LABEL: &[u8] = include_bytes!("../assets/ui/buildings.label");
const CAPACITY_LABEL: &[u8] = include_bytes!("../assets/ui/capacity.label");
const BUILD_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/build_button.label");
const RESEARCH_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/research_button.label");
const EXPLORE_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/explore_button.label");
const SAVE_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/save_button.label");
const LOAD_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/load_button.label");
const STORAGE_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/storage_button.label");
const STORAGE_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/storage_title.label");
const CRAFT_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/craft_button.label");
const CRAFT_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/craft_title.label");
const STATION_CATEGORY_LABEL: &[u8] = include_bytes!("../assets/ui/station_category.label");
const CONFIRM_QUIT_LABEL: &[u8] = include_bytes!("../assets/ui/confirm_quit.label");
const CHARACTER_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/character_button.label");
const CHARACTER_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/character_title.label");
const HP_LABEL: &[u8] = include_bytes!("../assets/ui/hp.label");
const ATTACK_STAT_LABEL: &[u8] = include_bytes!("../assets/ui/attack_stat.label");
const DEFENSE_STAT_LABEL: &[u8] = include_bytes!("../assets/ui/defense_stat.label");
const SPEED_STAT_LABEL: &[u8] = include_bytes!("../assets/ui/speed_stat.label");
const CLOSE_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/close_button.label");
const BUILD_MENU_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/build_menu_title.label");
const SMALL_STOREHOUSE_LABEL: &[u8] = include_bytes!("../assets/ui/small_storehouse.label");
const WORKBENCH_LABEL: &[u8] = include_bytes!("../assets/ui/workbench.label");
const CANCEL_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/cancel_button.label");
const WOOD_DESC_LABEL: &[u8] = include_bytes!("../assets/ui/wood_desc.label");
const STONE_DESC_LABEL: &[u8] = include_bytes!("../assets/ui/stone_desc.label");
const FOOD_DESC_LABEL: &[u8] = include_bytes!("../assets/ui/food_desc.label");
const COST_LABEL: &[u8] = include_bytes!("../assets/ui/cost.label");
const FOOTPRINT_LABEL: &[u8] = include_bytes!("../assets/ui/footprint.label");
const USAGE_LABEL: &[u8] = include_bytes!("../assets/ui/usage.label");
const FOOTPRINT_1X1_LABEL: &[u8] = include_bytes!("../assets/ui/footprint_1x1.label");
const USE_SMALL_STOREHOUSE_LABEL: &[u8] = include_bytes!("../assets/ui/use_small_storehouse.label");
const USE_STOREHOUSE_LABEL: &[u8] = include_bytes!("../assets/ui/use_storehouse.label");
const USE_WORKBENCH_LABEL: &[u8] = include_bytes!("../assets/ui/use_workbench.label");
const USE_RESEARCH_TABLE_LABEL: &[u8] = include_bytes!("../assets/ui/use_research_table.label");
const USE_TRAINING_DUMMY_LABEL: &[u8] = include_bytes!("../assets/ui/use_training_dummy.label");
const EXPEDITION_PREP_TITLE_LABEL: &[u8] =
    include_bytes!("../assets/ui/expedition_prep_title.label");
const PREP_AVAILABLE_CARDS_LABEL: &[u8] = include_bytes!("../assets/ui/prep_available_cards.label");
const PREP_CARRIED_CARDS_LABEL: &[u8] = include_bytes!("../assets/ui/prep_carried_cards.label");
const DEPART_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/depart_button.label");
const RETURN_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/return_button.label");
const BAG_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/bag_button.label");
const EXPEDITION_BAG_TITLE_LABEL: &[u8] = include_bytes!("../assets/ui/expedition_bag_title.label");
const CARD_MOVE_LABEL: &[u8] = include_bytes!("../assets/ui/card_move.label");
const CARD_GATHER_LABEL: &[u8] = include_bytes!("../assets/ui/card_gather.label");
const CARD_SEARCH_LABEL: &[u8] = include_bytes!("../assets/ui/card_search.label");
const CARD_ATTACK_LABEL: &[u8] = include_bytes!("../assets/ui/card_attack.label");
const CARD_DEFEND_LABEL: &[u8] = include_bytes!("../assets/ui/card_defend.label");
const CARD_SKILL_LABEL: &[u8] = include_bytes!("../assets/ui/card_skill.label");
const KIND_MOVE_LABEL: &[u8] = include_bytes!("../assets/ui/kind_move.label");
const KIND_GATHER_LABEL: &[u8] = include_bytes!("../assets/ui/kind_gather.label");
const KIND_SEARCH_LABEL: &[u8] = include_bytes!("../assets/ui/kind_search.label");
const KIND_ATTACK_LABEL: &[u8] = include_bytes!("../assets/ui/kind_attack.label");
const KIND_DEFEND_LABEL: &[u8] = include_bytes!("../assets/ui/kind_defend.label");
const KIND_SKILL_LABEL: &[u8] = include_bytes!("../assets/ui/kind_skill.label");
const EMPTY_SLOT_LABEL: &[u8] = include_bytes!("../assets/ui/empty_slot.label");
const EFFECT_MOVE_LABEL: &[u8] = include_bytes!("../assets/ui/effect_move.label");
const EFFECT_GATHER_LABEL: &[u8] = include_bytes!("../assets/ui/effect_gather.label");
const RETURN_FOOD_WARNING_LABEL: &[u8] = include_bytes!("../assets/ui/return_food_warning.label");
const DISCARD_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/discard_button.label");
const CONFIRM_DISCARD_LABEL: &[u8] = include_bytes!("../assets/ui/confirm_discard.label");
const CONFIRM_BUTTON_LABEL: &[u8] = include_bytes!("../assets/ui/confirm_button.label");
const DEPART_BLOCK_EMPTY_LABEL: &[u8] = include_bytes!("../assets/ui/depart_block_empty.label");
const DEPART_BLOCK_FOOD_LABEL: &[u8] = include_bytes!("../assets/ui/depart_block_food.label");
const DEPART_BLOCK_CAPACITY_LABEL: &[u8] =
    include_bytes!("../assets/ui/depart_block_capacity.label");
const RES_FOREST_INFO_LABEL: &[u8] = include_bytes!("../assets/ui/res_forest_info.label");
const RES_STONE_INFO_LABEL: &[u8] = include_bytes!("../assets/ui/res_stone_info.label");
const RES_BERRY_INFO_LABEL: &[u8] = include_bytes!("../assets/ui/res_berry_info.label");
const RES_ENEMY_INFO_LABEL: &[u8] = include_bytes!("../assets/ui/res_enemy_info.label");
const RES_RUIN_INFO_LABEL: &[u8] = include_bytes!("../assets/ui/res_ruin_info.label");
const NO_RECIPES_LABEL: &[u8] = include_bytes!("../assets/ui/no_recipes.label");

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

    pub fn draw(
        &mut self,
        game: &Game,
        mouse_x: usize,
        mouse_y: usize,
        dragged_expedition_card: Option<usize>,
    ) {
        self.clear(BLACK);
        match game.screen {
            Screen::MainMenu => self.draw_menu(game, mouse_x, mouse_y),
            Screen::TransitionToBase => self.draw_base_transition(game),
            Screen::TransitionToExpedition => self.draw_expedition_transition(game),
            Screen::Base => self.draw_base(game, mouse_x, mouse_y),
            Screen::Expedition => {
                self.draw_expedition(game, mouse_x, mouse_y, dragged_expedition_card)
            }
            Screen::GameOver => self.draw_game_over(game, mouse_x, mouse_y),
        }
    }

    fn draw_menu(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x1a2028);
        let panel_x = WIDTH / 2 - 230;
        let panel_y = HEIGHT / 2 - 190;
        self.rect(panel_x, panel_y, 460, 380, PANEL);
        self.draw_text("GRUNTIA", panel_x + 46, panel_y + 42, 6, TEXT);
        self.draw_text(
            "BASE AND CARD EXPEDITION DEMO",
            panel_x + 68,
            panel_y + 110,
            2,
            MUTED_TEXT,
        );
        self.menu_button(
            panel_x + 90,
            panel_y + 154,
            280,
            54,
            "NEW",
            mouse_x,
            mouse_y,
        );
        self.menu_button(
            panel_x + 90,
            panel_y + 224,
            280,
            54,
            "LOAD",
            mouse_x,
            mouse_y,
        );
        self.menu_button(
            panel_x + 90,
            panel_y + 294,
            280,
            54,
            "QUIT",
            mouse_x,
            mouse_y,
        );
        self.draw_footer(&game.message);
    }
    fn draw_base_transition(&mut self, game: &Game) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x111820);
        let panel_x = WIDTH / 2 - 240;
        let panel_y = HEIGHT / 2 - 88;
        self.rect(panel_x, panel_y, 480, 176, PANEL);
        self.draw_text("GRUNTIA", panel_x + 114, panel_y + 32, 4, TEXT);
        self.draw_text("PREPARING BASE", panel_x + 144, panel_y + 88, 2, MUTED_TEXT);

        let progress = game.transition_progress();
        let bar_width = 320;
        self.rect(panel_x + 80, panel_y + 134, bar_width, 12, 0x151a22);
        self.rect(
            panel_x + 80,
            panel_y + 134,
            (bar_width as f32 * progress) as usize,
            12,
            CARD,
        );
        self.draw_footer(&game.message);
    }
    fn draw_expedition_transition(&mut self, game: &Game) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x10161d);
        let panel_x = WIDTH / 2 - 260;
        let panel_y = HEIGHT / 2 - 95;
        self.rect(panel_x, panel_y, 520, 190, PANEL);
        self.draw_text("EXPEDITION", panel_x + 106, panel_y + 38, 4, TEXT);
        self.draw_text(
            "PREPARING MAP AND DECK",
            panel_x + 128,
            panel_y + 104,
            2,
            MUTED_TEXT,
        );

        let progress = game.transition_progress();
        let bar_width = 360;
        self.rect(panel_x + 80, panel_y + 150, bar_width, 12, 0x151a22);
        self.rect(
            panel_x + 80,
            panel_y + 150,
            (bar_width as f32 * progress) as usize,
            12,
            CARD,
        );
        self.draw_footer(&game.message);
    }
    fn draw_game_over(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x22181b);
        let panel_x = WIDTH / 2 - 300;
        let panel_y = HEIGHT / 2 - 150;
        self.rect(panel_x, panel_y, 600, 300, 0x5c2731);
        self.draw_text("CAMPAIGN FAILED", panel_x + 110, panel_y + 42, 3, TEXT);

        let death_label = if game.message.to_ascii_lowercase().contains("food") {
            DEATH_FOOD_LABEL
        } else {
            DEATH_CHARACTERS_LABEL
        };
        self.draw_label(death_label, WIDTH / 2, panel_y + 122, WARNING);

        let button_y = panel_y + 178;
        if save::save_exists() {
            self.side_button(
                WIDTH / 2 - 222,
                button_y,
                132,
                44,
                LOAD_SAVE_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        } else {
            self.disabled_side_button(WIDTH / 2 - 222, button_y, 132, 44, LOAD_SAVE_BUTTON_LABEL);
        }
        self.side_button(
            WIDTH / 2 - 66,
            button_y,
            132,
            44,
            RESTART_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.side_button(
            WIDTH / 2 + 90,
            button_y,
            132,
            44,
            MAIN_MENU_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.draw_footer(&game.message);
    }

    fn draw_base(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x17201d);

        let tile_size = game.base_tile_size();
        let gap = game.base_tile_gap();
        let step = tile_size + gap;
        let (origin_x, origin_y) = game.base_origin(BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT);

        for y in 0..game.base.size {
            for x in 0..game.base.size {
                self.rect_clipped(
                    origin_x + x * step as i32,
                    origin_y + y * step as i32,
                    tile_size,
                    tile_size,
                    BASE,
                    (0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT),
                );
            }
        }

        for building in &game.base.buildings {
            let x = origin_x + building.x * step as i32;
            let y = origin_y + building.y * step as i32;
            self.draw_base_building_icon(building.kind, x, y, tile_size);
        }

        self.rect(
            BASE_VIEWPORT_WIDTH,
            0,
            WIDTH - BASE_VIEWPORT_WIDTH,
            HEIGHT,
            PANEL,
        );
        self.draw_base_text(game, mouse_x, mouse_y);
        self.draw_building_preview(game, mouse_x, mouse_y);
        self.draw_hovered_base_building_info(game, mouse_x, mouse_y);
        if game.show_storage_dialog {
            self.draw_storage_dialog(game, mouse_x, mouse_y);
        }
        if game.show_character_dialog {
            self.draw_character_dialog(game, mouse_x, mouse_y);
        }
        if game.show_crafting_dialog {
            self.draw_crafting_dialog(mouse_x, mouse_y);
        }
        if game.show_build_menu {
            self.draw_build_menu(game, mouse_x, mouse_y);
        }
        if game.show_expedition_prep {
            self.draw_expedition_prep(game, mouse_x, mouse_y);
        }
    }

    fn draw_base_building_icon(&mut self, kind: BuildingKind, x: i32, y: i32, tile_size: usize) {
        let s = (tile_size / 12).max(1);
        let clip = (0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT);
        match kind {
            BuildingKind::Campfire => {
                self.rect_clipped(
                    x + 2 * s as i32,
                    y + 5 * s as i32,
                    8 * s,
                    5 * s,
                    0x6b4a2f,
                    clip,
                );
                self.rect_clipped(
                    x + 3 * s as i32,
                    y + 3 * s as i32,
                    6 * s,
                    3 * s,
                    0xb98957,
                    clip,
                );
                self.rect_clipped(x + 2 * s as i32, y + 4 * s as i32, 8 * s, s, 0x3f3024, clip);
                self.rect_clipped(x + 4 * s as i32, y + 6 * s as i32, s, 3 * s, 0xd7c59a, clip);
                self.rect_clipped(x + 7 * s as i32, y + 6 * s as i32, s, 3 * s, 0xd7c59a, clip);
                self.rect_clipped(
                    x + 5 * s as i32,
                    y + 2 * s as i32,
                    2 * s,
                    2 * s,
                    0x7aa06c,
                    clip,
                );
            }
            BuildingKind::Storehouse => {
                self.rect_clipped(
                    x + 2 * s as i32,
                    y + 6 * s as i32,
                    8 * s,
                    4 * s,
                    0x6b4a2f,
                    clip,
                );
                self.rect_clipped(
                    x + 3 * s as i32,
                    y + 4 * s as i32,
                    6 * s,
                    2 * s,
                    0xb98957,
                    clip,
                );
                self.rect_clipped(
                    x + 4 * s as i32,
                    y + 7 * s as i32,
                    4 * s,
                    3 * s,
                    0x3f3024,
                    clip,
                );
            }
            BuildingKind::Workbench => {
                self.rect_clipped(
                    x + 2 * s as i32,
                    y + 5 * s as i32,
                    8 * s,
                    2 * s,
                    0x6b4a2f,
                    clip,
                );
                self.rect_clipped(x + 3 * s as i32, y + 7 * s as i32, s, 3 * s, 0x3f3024, clip);
                self.rect_clipped(x + 8 * s as i32, y + 7 * s as i32, s, 3 * s, 0x3f3024, clip);
                self.rect_clipped(
                    x + 5 * s as i32,
                    y + 3 * s as i32,
                    3 * s,
                    2 * s,
                    0xc9b98e,
                    clip,
                );
            }
            BuildingKind::ResearchTable => {
                self.rect_clipped(
                    x + 3 * s as i32,
                    y + 6 * s as i32,
                    6 * s,
                    4 * s,
                    0x4f6fae,
                    clip,
                );
                self.rect_clipped(
                    x + 4 * s as i32,
                    y + 3 * s as i32,
                    4 * s,
                    3 * s,
                    0x8fb3f4,
                    clip,
                );
                self.rect_clipped(x + 5 * s as i32, y + 4 * s as i32, 2 * s, s, 0xf0d060, clip);
            }
            BuildingKind::TrainingDummy => {
                self.rect_clipped(
                    x + 5 * s as i32,
                    y + 2 * s as i32,
                    2 * s,
                    2 * s,
                    0xc47b6b,
                    clip,
                );
                self.rect_clipped(
                    x + 5 * s as i32,
                    y + 4 * s as i32,
                    2 * s,
                    5 * s,
                    0x6b4a2f,
                    clip,
                );
                self.rect_clipped(x + 3 * s as i32, y + 5 * s as i32, 6 * s, s, 0xd7c59a, clip);
                self.rect_clipped(x + 4 * s as i32, y + 9 * s as i32, 4 * s, s, 0x3f3024, clip);
            }
        }
    }

    fn draw_expedition(
        &mut self,
        game: &Game,
        mouse_x: usize,
        mouse_y: usize,
        dragged_card: Option<usize>,
    ) {
        self.rect(0, 0, WIDTH, HEIGHT, 0x12181d);
        self.rect(
            0,
            EXPEDITION_VIEWPORT_HEIGHT,
            WIDTH,
            HEIGHT - EXPEDITION_VIEWPORT_HEIGHT,
            PANEL,
        );

        let Some(expedition) = &game.expedition else {
            return;
        };

        let tile_size = expedition.tile_size();
        let (origin_x, origin_y) = expedition.origin(WIDTH, EXPEDITION_VIEWPORT_HEIGHT);

        for y in 0..expedition.map.height {
            for x in 0..expedition.map.width {
                let tile = expedition.map.tiles[(y * expedition.map.width + x) as usize];
                let color = match tile {
                    Tile::Ground => GROUND,
                    Tile::Forest | Tile::Stone | Tile::Berry | Tile::Enemy | Tile::Ruin => GROUND,
                };

                self.rect_clipped(
                    origin_x + x * tile_size as i32,
                    origin_y + y * tile_size as i32,
                    tile_size - 1,
                    tile_size - 1,
                    color,
                    (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
                );
                let tile_x = origin_x + x * tile_size as i32;
                let tile_y = origin_y + y * tile_size as i32;
                match tile {
                    Tile::Forest => self.draw_tree_icon(tile_x, tile_y, tile_size),
                    Tile::Stone => self.draw_stone_icon(tile_x, tile_y, tile_size),
                    Tile::Berry => self.draw_berry_icon(tile_x, tile_y, tile_size),
                    Tile::Enemy => {
                        self.draw_enemy_icon(tile_x, tile_y, tile_size, game.transition_timer)
                    }
                    _ => {}
                }
            }
        }

        for y in 0..expedition.map.height {
            for x in 0..expedition.map.width {
                if expedition.map.tile_at(x, y) == Tile::Ruin
                    && is_ruin_anchor(&expedition.map, x, y)
                {
                    self.draw_ruin_icon(
                        origin_x + x * tile_size as i32,
                        origin_y + y * tile_size as i32,
                        tile_size * 2,
                    );
                }
            }
        }

        let dragged_move_range = dragged_card
            .and_then(|index| expedition.hand.get(index))
            .filter(|card| card.kind == CardKind::Move)
            .map(|card| card.value);
        let move_range = expedition
            .pending_move_card
            .as_ref()
            .map(|card| card.value)
            .or(dragged_move_range);

        if let Some(range) = move_range {
            for y in 0..expedition.map.height {
                for x in 0..expedition.map.width {
                    if expedition.can_move_to(x, y, range) {
                        self.blend_rect_clipped(
                            origin_x + x * tile_size as i32 + 3,
                            origin_y + y * tile_size as i32 + 3,
                            tile_size.saturating_sub(6),
                            tile_size.saturating_sub(6),
                            0xffffff,
                            112,
                            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
                        );
                    }
                }
            }
        }

        if let Some(label) = hovered_expedition_tile_label(expedition, mouse_x, mouse_y) {
            self.resource_tooltip(mouse_x, mouse_y, label);
        }

        if let Some(index) = dragged_card {
            if let Some(card) = expedition.hand.get(index) {
                if card.kind == CardKind::Gather {
                    let color = if expedition.has_gather_targets() {
                        0xffffff
                    } else {
                        0xe06c75
                    };
                    for (x, y) in expedition.gather_range_tiles() {
                        self.blend_rect_clipped(
                            origin_x + x * tile_size as i32 + 2,
                            origin_y + y * tile_size as i32 + 2,
                            tile_size.saturating_sub(4),
                            tile_size.saturating_sub(4),
                            color,
                            104,
                            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
                        );
                    }
                }
            }
        }

        self.draw_player_marker(
            origin_x + expedition.player_x * tile_size as i32,
            origin_y + expedition.player_y * tile_size as i32,
            tile_size,
            game.transition_timer,
        );

        for (index, card) in expedition.hand.iter().enumerate() {
            if dragged_card == Some(index) {
                continue;
            }
            let x = 28 + index * 108;
            self.expedition_card(x, EXPEDITION_HAND_Y, card);
        }

        if let Some(effect_label) = hovered_expedition_card_effect(expedition, mouse_x, mouse_y) {
            self.rect(28, EXPEDITION_HAND_Y - 54, 390, 28, 0x1c2330);
            self.draw_label(effect_label, 223, EXPEDITION_HAND_Y - 40, MUTED_TEXT);
        }

        if let Some(index) = dragged_card {
            if let Some(card) = expedition.hand.get(index) {
                self.expedition_card(mouse_x.saturating_sub(46), mouse_y.saturating_sub(42), card);
            }
        }

        self.draw_expedition_text(game, mouse_x, mouse_y);
        if game.show_expedition_bag {
            self.draw_expedition_bag(game, mouse_x, mouse_y);
        }
    }

    fn draw_base_text(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        let panel_x = BASE_VIEWPORT_WIDTH + 28;
        self.draw_label(BASE_TITLE_LABEL, panel_x + 52, 32, TEXT);
        self.draw_stat_row(
            SIZE_LABEL,
            &format!("{}x{}", game.base.size, game.base.size),
            panel_x + 4,
            92,
        );
        self.draw_stat_row(
            BUILDINGS_LABEL,
            &game.base.buildings.len().to_string(),
            panel_x + 4,
            124,
        );
        self.draw_stat_row(
            CAPACITY_LABEL,
            &format!("{}/{}", game.storage_used(), game.storage_capacity()),
            panel_x + 4,
            156,
        );

        self.side_button(
            panel_x,
            224,
            196,
            38,
            CHARACTER_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.side_button(
            panel_x,
            274,
            196,
            38,
            STORAGE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        if game.storage_is_over_capacity() {
            self.warning_icon(panel_x + 174, 280);
        }
        if game.has_building(BuildingKind::Workbench) {
            self.side_button(panel_x, 324, 196, 38, CRAFT_BUTTON_LABEL, mouse_x, mouse_y);
        } else {
            self.disabled_side_button(panel_x, 324, 196, 38, CRAFT_BUTTON_LABEL);
        }
        self.side_button(panel_x, 374, 92, 38, BUILD_BUTTON_LABEL, mouse_x, mouse_y);
        if game.has_building(BuildingKind::ResearchTable) {
            self.side_button(
                panel_x + 104,
                374,
                92,
                38,
                RESEARCH_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        } else {
            self.disabled_side_button(panel_x + 104, 374, 92, 38, RESEARCH_BUTTON_LABEL);
        }
        self.side_button(
            panel_x,
            424,
            196,
            38,
            EXPLORE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.side_button(panel_x, 474, 92, 38, SAVE_BUTTON_LABEL, mouse_x, mouse_y);
        if save::save_exists() {
            self.side_button(
                panel_x + 104,
                474,
                92,
                38,
                LOAD_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        } else {
            self.disabled_side_button(panel_x + 104, 474, 92, 38, LOAD_BUTTON_LABEL);
        }
        self.side_button(panel_x, 524, 196, 38, QUIT_LABEL, mouse_x, mouse_y);
        if game.confirm_quit {
            self.rect(panel_x - 8, 574, 212, 86, 0x1c2330);
            self.draw_label(CONFIRM_QUIT_LABEL, panel_x + 98, 596, WARNING);
            self.side_button(panel_x, 612, 92, 36, CONFIRM_BUTTON_LABEL, mouse_x, mouse_y);
            self.side_button(
                panel_x + 104,
                612,
                92,
                36,
                CANCEL_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        }
        self.draw_text(
            &format!("ZOOM {}x", game.base.zoom_level + 1),
            12,
            HEIGHT - 42,
            1,
            MUTED_TEXT,
        );
        self.draw_footer(&game.message);
    }

    fn draw_character_dialog(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT, 0x11151c);
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 240;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 150;
        self.rect(panel_x, panel_y, 480, 300, PANEL);
        self.rect(panel_x + 4, panel_y + 4, 472, 292, 0x252d3a);
        self.draw_label(CHARACTER_TITLE_LABEL, panel_x + 240, panel_y + 42, TEXT);

        if let Some(character) = game.characters.first() {
            self.draw_text(&character.name, panel_x + 94, panel_y + 84, 2, TEXT);
            self.draw_stat_row(
                HP_LABEL,
                &format!("{}/{}", character.hp, character.max_hp),
                panel_x + 94,
                panel_y + 132,
            );
            self.draw_stat_row(
                ATTACK_STAT_LABEL,
                &character.attack.to_string(),
                panel_x + 94,
                panel_y + 172,
            );
            self.draw_stat_row(
                DEFENSE_STAT_LABEL,
                &character.defense.to_string(),
                panel_x + 94,
                panel_y + 212,
            );
            self.draw_stat_row(
                SPEED_STAT_LABEL,
                &character.speed.to_string(),
                panel_x + 282,
                panel_y + 132,
            );
        }

        self.side_button(
            panel_x + 188,
            panel_y + 246,
            104,
            40,
            CLOSE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
    }

    fn draw_crafting_dialog(&mut self, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT, 0x11151c);
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 240;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 130;
        self.rect(panel_x, panel_y, 480, 260, PANEL);
        self.rect(panel_x + 4, panel_y + 4, 472, 252, 0x252d3a);
        self.draw_label(CRAFT_TITLE_LABEL, panel_x + 240, panel_y + 42, TEXT);
        self.rect(panel_x + 58, panel_y + 92, 364, 72, 0x1c2330);
        self.draw_label(NO_RECIPES_LABEL, panel_x + 240, panel_y + 128, MUTED_TEXT);
        self.side_button(
            panel_x + 188,
            panel_y + 206,
            104,
            40,
            CLOSE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
    }

    fn draw_storage_dialog(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT, 0x11151c);
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 300;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 190;
        self.rect(panel_x, panel_y, 600, 380, PANEL);
        self.rect(panel_x + 4, panel_y + 4, 592, 372, 0x252d3a);
        self.draw_label(STORAGE_TITLE_LABEL, panel_x + 300, panel_y + 40, TEXT);

        let visible_resources = game.visible_storage_indices();
        for (row, index) in visible_resources.iter().copied().enumerate() {
            self.draw_storage_resource_row(
                game,
                index,
                panel_x + 72,
                panel_y + 92 + row * 56,
                mouse_x,
                mouse_y,
            );
        }
        self.draw_stat_row(
            CAPACITY_LABEL,
            &format!("{}/{}", game.storage_used(), game.storage_capacity()),
            panel_x + 74,
            panel_y + 284,
        );

        self.side_button(
            panel_x + 452,
            panel_y + 316,
            96,
            40,
            CLOSE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );

        if let Some(index) = game.pending_storage_discard {
            self.rect(panel_x + 138, panel_y + 318, 292, 38, 0x1c2330);
            self.draw_label(CONFIRM_DISCARD_LABEL, panel_x + 250, panel_y + 337, WARNING);
            self.side_button(
                panel_x + 338,
                panel_y + 316,
                84,
                40,
                CONFIRM_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
            self.draw_text(
                &format!("{}", game.storage_discard_inputs[index]),
                panel_x + 292,
                panel_y + 329,
                1,
                WARNING,
            );
        }

        for (row, index) in visible_resources.iter().copied().enumerate() {
            if point_in_rect(
                mouse_x,
                mouse_y,
                panel_x + 72,
                panel_y + 80 + row * 56,
                36,
                36,
            ) {
                let label = match index {
                    0 => WOOD_DESC_LABEL,
                    1 => STONE_DESC_LABEL,
                    2 => FOOD_DESC_LABEL,
                    _ => FOOD_DESC_LABEL,
                };
                self.draw_resource_description(label);
            }
        }
    }

    fn draw_resource_description(&mut self, label: &[u8]) {
        let width = BASE_VIEWPORT_WIDTH.saturating_sub(80);
        self.rect(40, BASE_VIEWPORT_HEIGHT - 72, width, 32, 0x1c2330);
        self.draw_label(
            label,
            BASE_VIEWPORT_WIDTH / 2,
            BASE_VIEWPORT_HEIGHT - 56,
            MUTED_TEXT,
        );
    }

    fn draw_storage_resource_row(
        &mut self,
        game: &Game,
        index: usize,
        x: usize,
        y: usize,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let amount = game.resources.amount(index);
        self.draw_resource_icon(index, x, y, 36);
        self.draw_text(&amount.to_string(), x + 58, y + 9, 2, TEXT);

        let input_x = x + 158;
        let focused = game.focused_storage_input == Some(index);
        let input_color = if focused { 0xf0d060 } else { 0x607080 };
        self.arrow_button(x + 126, y + 2, 26, 32, false, mouse_x, mouse_y);
        self.rect(input_x, y + 2, 46, 32, input_color);
        self.rect(input_x + 3, y + 5, 40, 26, 0x1c2330);
        self.draw_text(
            &game.storage_discard_inputs[index].to_string(),
            input_x + 10,
            y + 10,
            2,
            TEXT,
        );
        self.arrow_button(input_x + 52, y + 2, 26, 32, true, mouse_x, mouse_y);

        let can_discard =
            game.storage_discard_inputs[index] > 0 && game.storage_discard_inputs[index] <= amount;
        if can_discard {
            self.side_button(
                input_x + 124,
                y,
                88,
                36,
                DISCARD_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        } else {
            self.disabled_side_button(input_x + 124, y, 88, 36, DISCARD_BUTTON_LABEL);
        }
    }

    fn draw_resource_icon(&mut self, index: usize, x: usize, y: usize, size: usize) {
        match index {
            0 => {
                self.rect(x + 8, y + 6, size - 16, size - 12, 0x8b5a35);
                self.rect(x + 5, y + 11, size - 10, size - 14, 0x6b4329);
                self.rect(x + 12, y + 4, 4, size - 8, 0xb57945);
            }
            1 => {
                self.rect(x + 6, y + 14, size - 12, size - 10, 0x5f6872);
                self.rect(x + 12, y + 7, size - 18, size - 14, 0x8a929b);
                self.rect(x + 20, y + 12, size - 24, size - 18, 0xb0b7bf);
            }
            2 => {
                self.rect(x + 9, y + 12, size - 18, size - 12, 0xc44b6a);
                self.rect(x + 13, y + 8, size - 26, size - 24, 0x4fa65e);
                self.rect(x + 20, y + 6, size - 28, size - 26, 0x3f8f4f);
            }
            _ => {}
        }
    }

    fn draw_build_menu(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT, 0x11151c);
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 250;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 160;
        self.rect(panel_x, panel_y, 500, 300, PANEL);
        self.rect(panel_x + 4, panel_y + 4, 492, 292, 0x252d3a);
        self.draw_label(BUILD_MENU_TITLE_LABEL, panel_x + 250, panel_y + 40, TEXT);
        self.category_button(
            panel_x + 40,
            panel_y + 74,
            124,
            38,
            STORAGE_BUTTON_LABEL,
            game.build_menu_category == 0,
            mouse_x,
            mouse_y,
        );
        self.category_button(
            panel_x + 40,
            panel_y + 120,
            124,
            38,
            STATION_CATEGORY_LABEL,
            game.build_menu_category == 1,
            mouse_x,
            mouse_y,
        );

        let (kind, label) = if game.build_menu_category == 0 {
            (BuildingKind::Campfire, SMALL_STOREHOUSE_LABEL)
        } else {
            (BuildingKind::Workbench, WORKBENCH_LABEL)
        };

        self.building_button(
            game,
            panel_x + 260,
            panel_y + 90,
            180,
            40,
            label,
            kind,
            mouse_x,
            mouse_y,
        );
        self.side_button(
            panel_x + 260,
            panel_y + 190,
            180,
            40,
            CANCEL_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );

        if point_in_rect(mouse_x, mouse_y, panel_x + 260, panel_y + 90, 180, 40) {
            self.draw_building_details(game, kind, panel_x + 20, panel_y + 242);
        } else {
            self.draw_text(
                "HOVER A BUILDING TO VIEW DETAILS",
                panel_x + 56,
                panel_y + 252,
                1,
                MUTED_TEXT,
            );
        }
    }

    fn category_button(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        label: &[u8],
        active: bool,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let hovered = point_in_rect(mouse_x, mouse_y, x, y, width, height);
        let background = if active {
            0x3a4654
        } else if hovered {
            0x303946
        } else {
            0x252d3a
        };
        self.rect(x, y, width, height, background);
        if active {
            self.rect(x, y, 4, height, WARNING);
        }
        self.draw_label(label, x + width / 2, y + height / 2, TEXT);
    }

    fn draw_building_details(&mut self, game: &Game, kind: BuildingKind, x: usize, y: usize) {
        let cost = building_cost(kind);
        self.rect(x, y, 460, 46, 0x1c2330);
        self.draw_label(COST_LABEL, x + 34, y + 14, TEXT);
        let mut cost_x = x + 76;
        self.draw_cost_part(0, cost.wood, game.resources.wood, &mut cost_x, y + 14);
        self.draw_cost_part(1, cost.stone, game.resources.stone, &mut cost_x, y + 14);
        self.draw_cost_part(2, cost.food, game.resources.food, &mut cost_x, y + 14);
        self.draw_label(FOOTPRINT_LABEL, x + 292, y + 14, TEXT);
        self.draw_label(FOOTPRINT_1X1_LABEL, x + 354, y + 14, TEXT);
        self.draw_label(USAGE_LABEL, x + 34, y + 34, MUTED_TEXT);
        self.draw_label(building_usage_label(kind), x + 180, y + 34, MUTED_TEXT);
    }

    fn draw_expedition_prep(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.rect(0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT, 0x11151c);
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 316;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 212;
        self.rect(panel_x, panel_y, 632, 424, PANEL);
        self.rect(panel_x + 4, panel_y + 4, 624, 416, 0x252d3a);
        self.draw_label(
            EXPEDITION_PREP_TITLE_LABEL,
            panel_x + 316,
            panel_y + 34,
            TEXT,
        );
        self.draw_label(
            PREP_AVAILABLE_CARDS_LABEL,
            panel_x + 176,
            panel_y + 66,
            TEXT,
        );
        self.draw_label(PREP_CARRIED_CARDS_LABEL, panel_x + 532, panel_y + 66, TEXT);

        for (index, card) in Card::available_cards().iter().enumerate() {
            let x = panel_x + 40 + (index % 3) * 126;
            let y = panel_y + 96 + (index / 3) * 126;
            self.prep_card(x, y, 100, 104, card, mouse_x, mouse_y);
        }

        for index in 0..MAX_PREP_CARDS {
            let y = panel_y + 74 + index * 56;
            self.rect(panel_x + 472, y, 120, 44, 0xd7c59a);
            if let Some(card) = game.prep_deck.get(index) {
                self.rect(panel_x + 476, y + 4, 112, 36, 0xc9b98e);
                self.draw_label(card_name_label(card.kind), panel_x + 532, y + 22, BLACK);
            } else {
                self.rect(panel_x + 476, y + 4, 112, 36, 0x333a46);
                self.draw_label(EMPTY_SLOT_LABEL, panel_x + 532, y + 22, MUTED_TEXT);
            }
        }

        if let Some(effect_label) = hovered_prep_card_effect(mouse_x, mouse_y) {
            self.rect(panel_x + 24, panel_y + 316, 390, 34, 0x1c2330);
            self.draw_label(effect_label, panel_x + 219, panel_y + 333, MUTED_TEXT);
        }

        self.draw_prep_food_info(game, panel_x, panel_y);
        self.side_button(
            panel_x + 396,
            panel_y + 372,
            100,
            40,
            RETURN_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        if game.depart_block_reason().is_some() {
            self.disabled_side_button(panel_x + 516, panel_y + 372, 100, 40, DEPART_BUTTON_LABEL);
            if point_in_rect(mouse_x, mouse_y, panel_x + 516, panel_y + 372, 100, 40) {
                if let Some(reason) = game.depart_block_reason() {
                    self.depart_block_tooltip(panel_x + 396, panel_y + 336, reason);
                }
            }
        } else {
            self.side_button(
                panel_x + 516,
                panel_y + 372,
                100,
                40,
                DEPART_BUTTON_LABEL,
                mouse_x,
                mouse_y,
            );
        }
    }

    fn depart_block_tooltip(&mut self, x: usize, y: usize, reason: DepartBlockReason) {
        let label = match reason {
            DepartBlockReason::EmptyDeck => DEPART_BLOCK_EMPTY_LABEL,
            DepartBlockReason::Food => DEPART_BLOCK_FOOD_LABEL,
            DepartBlockReason::Capacity => DEPART_BLOCK_CAPACITY_LABEL,
        };
        self.rect(x, y, 220, 30, 0x1c2330);
        self.draw_label(label, x + 110, y + 15, WARNING);
    }

    fn prep_card(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        card: &Card,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let hovered = point_in_rect(mouse_x, mouse_y, x, y, width, height);
        let outer = if hovered { 0xf0d060 } else { 0xd7c59a };
        let inner = if hovered { 0xe6c66d } else { 0xc9b98e };

        self.rect(x, y, width, height, outer);
        self.rect(x + 4, y + 4, width - 8, height - 8, inner);
        self.draw_label(card_name_label(card.kind), x + width / 2, y + 24, BLACK);
        self.draw_label(card_kind_label(card.kind), x + width / 2, y + 52, BLACK);
        self.draw_label(FOOD_LABEL, x + 38, y + 80, BLACK);
        self.draw_text(&card.food_cost.to_string(), x + 68, y + 73, 2, BLACK);
    }

    fn expedition_card(&mut self, x: usize, y: usize, card: &Card) {
        self.rect(x, y, 92, 84, CARD);
        self.rect(x + 4, y + 4, 84, 76, 0xc9b98e);
        self.draw_label(card_name_label(card.kind), x + 46, y + 24, BLACK);
        self.draw_label(card_kind_label(card.kind), x + 46, y + 52, BLACK);
    }

    fn draw_prep_food_info(&mut self, game: &Game, panel_x: usize, panel_y: usize) {
        let cost = game.prep_food_cost();
        let remaining = game.resources.food - cost;
        let color = if game.resources.food < cost || remaining < 3 {
            0xe06c75
        } else {
            TEXT
        };
        self.rect(panel_x + 24, panel_y + 368, 310, 40, 0x1c2330);
        self.draw_label(FOOD_LABEL, panel_x + 60, panel_y + 388, color);
        self.draw_text(
            &format!(": {} [-{}]", game.resources.food, cost),
            panel_x + 94,
            panel_y + 381,
            2,
            color,
        );
        if game.resources.food >= cost && remaining < 3 {
            self.rect(panel_x + 24, panel_y + 350, 310, 18, 0x1c2330);
            self.draw_label(
                RETURN_FOOD_WARNING_LABEL,
                panel_x + 179,
                panel_y + 360,
                color,
            );
        }
    }

    fn draw_building_preview(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        let Some(kind) = game.selected_building else {
            return;
        };
        if mouse_x >= BASE_VIEWPORT_WIDTH || mouse_y >= BASE_VIEWPORT_HEIGHT {
            return;
        }

        let Some((tile_x, tile_y)) = screen_to_base_tile(game, mouse_x, mouse_y) else {
            return;
        };

        let tile_size = game.base_tile_size();
        let step = game.base_tile_size() + game.base_tile_gap();
        let (origin_x, origin_y) = game.base_origin(BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT);
        let occupied = game
            .base
            .buildings
            .iter()
            .any(|building| building.x == tile_x && building.y == tile_y);
        let color = if occupied {
            0xb94b4b
        } else {
            building_color(kind)
        };

        self.blend_rect_clipped(
            origin_x + tile_x * step as i32,
            origin_y + tile_y * step as i32,
            tile_size,
            tile_size,
            color,
            104,
            (0, 0, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT),
        );
        if !occupied {
            self.draw_base_building_icon(
                kind,
                origin_x + tile_x * step as i32,
                origin_y + tile_y * step as i32,
                tile_size,
            );
        }
    }

    fn draw_hovered_base_building_info(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        if game.selected_building.is_some()
            || game.show_storage_dialog
            || game.show_build_menu
            || game.show_expedition_prep
        {
            return;
        }

        let Some((tile_x, tile_y)) = screen_to_base_tile(game, mouse_x, mouse_y) else {
            return;
        };
        let Some(building) = game
            .base
            .buildings
            .iter()
            .find(|building| building.x == tile_x && building.y == tile_y)
        else {
            return;
        };

        self.base_building_tooltip(mouse_x, mouse_y, building_usage_label(building.kind));
    }

    fn base_building_tooltip(&mut self, mouse_x: usize, mouse_y: usize, label: &[u8]) {
        let width = label_width(label) + 24;
        let x = mouse_x
            .saturating_add(14)
            .min(BASE_VIEWPORT_WIDTH.saturating_sub(width + 8));
        let y = mouse_y
            .saturating_add(14)
            .min(BASE_VIEWPORT_HEIGHT.saturating_sub(34));
        self.rect(x, y, width, 28, 0x1c2330);
        self.draw_label(label, x + width / 2, y + 14, TEXT);
    }

    fn draw_expedition_text(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        self.draw_text("DRAG CARD TO PLAY", 28, EXPEDITION_HAND_Y - 18, 1, TEXT);
        let button_x = WIDTH - 132;
        self.side_button(
            button_x,
            HEIGHT - 104,
            104,
            40,
            BAG_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.side_button(
            button_x,
            HEIGHT - 58,
            104,
            40,
            RETURN_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
        self.draw_footer(&game.message);
    }

    fn draw_expedition_bag(&mut self, game: &Game, mouse_x: usize, mouse_y: usize) {
        let Some(expedition) = &game.expedition else {
            return;
        };
        let panel_x = WIDTH / 2 - 180;
        self.rect(panel_x, 144, 360, 230, PANEL);
        self.rect(panel_x + 4, 148, 352, 222, 0x252d3a);
        self.draw_label(EXPEDITION_BAG_TITLE_LABEL, panel_x + 180, 184, TEXT);
        self.draw_stat_row(
            WOOD_LABEL,
            &expedition.cargo.wood.to_string(),
            panel_x + 100,
            228,
        );
        self.draw_stat_row(
            STONE_LABEL,
            &expedition.cargo.stone.to_string(),
            panel_x + 100,
            268,
        );
        self.draw_stat_row(
            FOOD_LABEL,
            &expedition.cargo.food.to_string(),
            panel_x + 100,
            300,
        );
        self.side_button(
            panel_x + 128,
            324,
            104,
            40,
            CLOSE_BUTTON_LABEL,
            mouse_x,
            mouse_y,
        );
    }

    fn draw_footer(&mut self, message: &str) {
        self.rect(0, HEIGHT - 24, WIDTH, 24, 0x11151c);
        self.draw_text(message, 12, HEIGHT - 17, 1, MUTED_TEXT);
    }

    fn menu_button(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        label: &str,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let hovered = point_in_rect(mouse_x, mouse_y, x, y, width, height);
        let outer = if hovered { 0xf0d060 } else { 0xd7c59a };
        let inner = if hovered { 0xe6c66d } else { 0xbda87c };
        let text = if hovered { 0x11151c } else { BLACK };

        self.rect(x, y, width, height, outer);
        self.rect(x + 4, y + 4, width - 8, height - 8, inner);
        if hovered {
            self.rect(x + 8, y + height - 9, width - 16, 3, 0x6f5930);
        }

        let label = match label {
            "NEW" => NEW_GAME_LABEL,
            "LOAD" => LOAD_GAME_LABEL,
            "QUIT" => QUIT_LABEL,
            _ => NEW_GAME_LABEL,
        };
        self.draw_label(label, x + width / 2, y + height / 2, text);
    }

    fn side_button(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        label: &[u8],
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let hovered = point_in_rect(mouse_x, mouse_y, x, y, width, height);
        let outer = if hovered { 0xf0d060 } else { 0xd7c59a };
        let inner = if hovered { 0xe6c66d } else { 0xbda87c };

        self.rect(x, y, width, height, outer);
        self.rect(x + 3, y + 3, width - 6, height - 6, inner);
        self.draw_label(label, x + width / 2, y + height / 2, BLACK);
    }

    fn arrow_button(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        points_right: bool,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        let hovered = point_in_rect(mouse_x, mouse_y, x, y, width, height);
        let outer = if hovered { 0xf0d060 } else { 0xd7c59a };
        let inner = if hovered { 0xe6c66d } else { 0xbda87c };

        self.rect(x, y, width, height, outer);
        self.rect(x + 3, y + 3, width - 6, height - 6, inner);
        let mid_x = x + width / 2;
        let mid_y = y + height / 2;
        for row in 0..11 {
            let span = if row <= 5 { row } else { 10 - row };
            let py = mid_y + row - 5;
            for offset in 0..=span {
                let px = if points_right {
                    mid_x + offset / 2
                } else {
                    mid_x.saturating_sub(offset / 2)
                };
                self.rect(px, py, 2, 1, BLACK);
            }
        }
    }

    fn building_button(
        &mut self,
        game: &Game,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        label: &[u8],
        kind: BuildingKind,
        mouse_x: usize,
        mouse_y: usize,
    ) {
        if game.resources.can_afford(&building_cost(kind)) {
            self.side_button(x, y, width, height, label, mouse_x, mouse_y);
        } else {
            self.disabled_side_button(x, y, width, height, label);
        }
    }

    fn disabled_side_button(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        label: &[u8],
    ) {
        self.rect(x, y, width, height, 0x59606a);
        self.rect(x + 3, y + 3, width - 6, height - 6, 0x3f4650);
        self.draw_label(label, x + width / 2, y + height / 2, MUTED_TEXT);
    }

    fn draw_stat_row(&mut self, label: &[u8], value: &str, x: usize, y: usize) {
        self.draw_label(label, x + 22, y, TEXT);
        self.draw_text(value, x + 72, y - 7, 2, TEXT);
    }

    fn draw_cost_part(
        &mut self,
        resource_index: usize,
        amount: i32,
        available: i32,
        x: &mut usize,
        y: usize,
    ) {
        if amount <= 0 {
            return;
        }

        let color = if available >= amount { TEXT } else { 0xe06c75 };
        self.draw_resource_icon(resource_index, *x, y.saturating_sub(11), 22);
        if color != TEXT {
            self.rect(*x, y + 11, 22, 2, color);
        }
        *x += 28;
        self.draw_text(&amount.to_string(), *x, y - 7, 2, color);
        *x += 28;
    }

    fn warning_icon(&mut self, x: usize, y: usize) {
        self.rect(x, y, 16, 16, 0xb94b4b);
        self.rect(x + 6, y + 3, 4, 7, TEXT);
        self.rect(x + 6, y + 12, 4, 2, TEXT);
    }

    fn draw_player_marker(&mut self, x: i32, y: i32, tile_size: usize, time: f32) {
        let scale = (tile_size / 12).max(1);
        let bob = ((time * 5.0).sin() * scale as f32).round() as i32;
        let px = x + (tile_size as i32 / 2) - (6 * scale) as i32;
        let py = y + (tile_size as i32 / 2) - (6 * scale) as i32 + bob;
        self.rect_clipped(
            px + 4 * scale as i32,
            py + scale as i32,
            4 * scale,
            4 * scale,
            0xffdf7a,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + 3 * scale as i32,
            py + 5 * scale as i32,
            6 * scale,
            4 * scale,
            PLAYER,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + scale as i32,
            py + 6 * scale as i32,
            2 * scale,
            3 * scale,
            0x6b8fd6,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + 9 * scale as i32,
            py + 6 * scale as i32,
            2 * scale,
            3 * scale,
            0x6b8fd6,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + 3 * scale as i32,
            py + 9 * scale as i32,
            2 * scale,
            3 * scale,
            0x1a2430,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + 7 * scale as i32,
            py + 9 * scale as i32,
            2 * scale,
            3 * scale,
            0x1a2430,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            px + 2 * scale as i32,
            py,
            8 * scale,
            scale,
            0x5a3e2b,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
    }

    fn draw_tree_icon(&mut self, x: i32, y: i32, tile_size: usize) {
        let s = (tile_size / 12).max(1);
        let center = x + (tile_size / 2) as i32;
        let top = y + 2 * s as i32;
        let trunk_y = y + 7 * s as i32;
        self.rect_clipped(
            center - s as i32,
            trunk_y,
            2 * s,
            4 * s,
            0x6b4a2f,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            center - 3 * s as i32,
            top + 2 * s as i32,
            6 * s,
            3 * s,
            FOREST,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            center - 2 * s as i32,
            top,
            4 * s,
            3 * s,
            0x3fa35a,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            center - 4 * s as i32,
            top + 4 * s as i32,
            8 * s,
            3 * s,
            0x2f7d43,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
    }

    fn draw_stone_icon(&mut self, x: i32, y: i32, tile_size: usize) {
        let s = (tile_size / 12).max(1);
        let left = x + 2 * s as i32;
        let top = y + 4 * s as i32;
        self.rect_clipped(
            left,
            top + 2 * s as i32,
            8 * s,
            5 * s,
            0x5f6872,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            left + 2 * s as i32,
            top,
            5 * s,
            4 * s,
            STONE,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            left + 5 * s as i32,
            top + s as i32,
            3 * s,
            3 * s,
            0x9aa5b1,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
    }

    fn draw_berry_icon(&mut self, x: i32, y: i32, tile_size: usize) {
        let s = (tile_size / 12).max(1);
        let cx = x + (tile_size / 2) as i32;
        let cy = y + (tile_size / 2) as i32;
        self.rect_clipped(
            cx - 4 * s as i32,
            cy,
            8 * s,
            4 * s,
            0x3f8f4f,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            cx - 3 * s as i32,
            cy - 2 * s as i32,
            6 * s,
            3 * s,
            0x4fa65e,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        for (dx, dy) in [(-3, 0), (0, -1), (3, 1), (-1, 2)] {
            self.rect_clipped(
                cx + dx * s as i32,
                cy + dy * s as i32,
                2 * s,
                2 * s,
                0xc44b6a,
                (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
            );
        }
    }

    fn draw_enemy_icon(&mut self, x: i32, y: i32, tile_size: usize, time: f32) {
        let s = (tile_size / 12).max(1);
        let bob =
            ((time * 4.0 + x as f32 * 0.03 + y as f32 * 0.05).sin() * s as f32).round() as i32;
        let cx = x + (tile_size / 2) as i32;
        let cy = y + (tile_size / 2) as i32 + bob;
        self.rect_clipped(
            cx - 4 * s as i32,
            cy - 3 * s as i32,
            8 * s,
            7 * s,
            ENEMY,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            cx - 6 * s as i32,
            cy - s as i32,
            2 * s,
            5 * s,
            0x6d2020,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            cx + 4 * s as i32,
            cy - s as i32,
            2 * s,
            5 * s,
            0x6d2020,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            cx - 2 * s as i32,
            cy - s as i32,
            s,
            s,
            0xffd1d1,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            cx + s as i32,
            cy - s as i32,
            s,
            s,
            0xffd1d1,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
    }

    fn draw_ruin_icon(&mut self, x: i32, y: i32, tile_size: usize) {
        let s = (tile_size / 12).max(1);
        let left = x + 2 * s as i32;
        let top = y + 2 * s as i32;
        self.rect_clipped(
            left,
            top + 6 * s as i32,
            8 * s,
            3 * s,
            RUIN,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            left + s as i32,
            top + 2 * s as i32,
            2 * s,
            5 * s,
            0xd6b15f,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            left + 5 * s as i32,
            top + s as i32,
            2 * s,
            6 * s,
            0xd6b15f,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
        self.rect_clipped(
            left,
            top,
            9 * s,
            2 * s,
            0xf0d060,
            (0, 0, WIDTH, EXPEDITION_VIEWPORT_HEIGHT),
        );
    }

    fn resource_tooltip(&mut self, mouse_x: usize, mouse_y: usize, label: &[u8]) {
        let width = label_width(label) + 24;
        let x = mouse_x
            .saturating_add(14)
            .min(WIDTH.saturating_sub(width + 8));
        let y = mouse_y
            .saturating_add(14)
            .min(EXPEDITION_VIEWPORT_HEIGHT.saturating_sub(34));
        self.rect(x, y, width, 28, 0x1c2330);
        self.draw_label(label, x + width / 2, y + 14, TEXT);
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

    fn rect_clipped(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        color: u32,
        clip: (usize, usize, usize, usize),
    ) {
        let (clip_x, clip_y, clip_width, clip_height) = clip;
        let min_x = x.max(clip_x as i32).max(0) as usize;
        let min_y = y.max(clip_y as i32).max(0) as usize;
        let max_x = (x + width as i32)
            .min((clip_x + clip_width) as i32)
            .min(WIDTH as i32);
        let max_y = (y + height as i32)
            .min((clip_y + clip_height) as i32)
            .min(HEIGHT as i32);

        if max_x <= min_x as i32 || max_y <= min_y as i32 {
            return;
        }

        for py in min_y..max_y as usize {
            for px in min_x..max_x as usize {
                self.buffer[py * WIDTH + px] = color;
            }
        }
    }

    fn blend_rect_clipped(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        color: u32,
        alpha: u32,
        clip: (usize, usize, usize, usize),
    ) {
        let (clip_x, clip_y, clip_width, clip_height) = clip;
        let min_x = x.max(clip_x as i32).max(0) as usize;
        let min_y = y.max(clip_y as i32).max(0) as usize;
        let max_x = (x + width as i32)
            .min((clip_x + clip_width) as i32)
            .min(WIDTH as i32);
        let max_y = (y + height as i32)
            .min((clip_y + clip_height) as i32)
            .min(HEIGHT as i32);

        if max_x <= min_x as i32 || max_y <= min_y as i32 {
            return;
        }

        for py in min_y..max_y as usize {
            for px in min_x..max_x as usize {
                self.blend_pixel(px, py, color, alpha);
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, scale: usize, color: u32) {
        let mut cursor_x = x;
        let max_chars = ((WIDTH.saturating_sub(x)) / (6 * scale.max(1))).max(1);

        for ch in text.chars().take(max_chars) {
            if !ch.is_ascii() {
                cursor_x += 6 * scale;
                continue;
            }
            self.draw_char(ch, cursor_x, y, scale, color);
            cursor_x += 6 * scale;
        }
    }

    fn draw_char(&mut self, ch: char, x: usize, y: usize, scale: usize, color: u32) {
        let glyph = glyph(ch);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..5 {
                if (bits >> (4 - col)) & 1 == 1 {
                    self.rect(x + col * scale, y + row * scale, scale, scale, color);
                }
            }
        }
    }

    fn draw_label(&mut self, label: &[u8], center_x: usize, center_y: usize, color: u32) {
        if label.len() < 4 {
            return;
        }

        let width = u16::from_le_bytes([label[0], label[1]]) as usize;
        let height = u16::from_le_bytes([label[2], label[3]]) as usize;
        let x = center_x.saturating_sub(width / 2);
        let y = center_y.saturating_sub(height / 2);

        for row in 0..height {
            for col in 0..width {
                let alpha = label[4 + row * width + col] as u32;
                if alpha > 8 {
                    self.blend_pixel(x + col, y + row, color, alpha);
                }
            }
        }
    }

    fn blend_pixel(&mut self, x: usize, y: usize, color: u32, alpha: u32) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let index = y * WIDTH + x;
        let dst = self.buffer[index];
        let inv = 255 - alpha;
        let rb = (((color & 0x00ff00ff) * alpha + (dst & 0x00ff00ff) * inv) >> 8) & 0x00ff00ff;
        let g = (((color & 0x0000ff00) * alpha + (dst & 0x0000ff00) * inv) >> 8) & 0x0000ff00;
        self.buffer[index] = rb | g;
    }
}

fn point_in_rect(
    x: usize,
    y: usize,
    rect_x: usize,
    rect_y: usize,
    width: usize,
    height: usize,
) -> bool {
    x >= rect_x && x < rect_x + width && y >= rect_y && y < rect_y + height
}

fn screen_to_base_tile(game: &Game, mouse_x: usize, mouse_y: usize) -> Option<(i32, i32)> {
    let tile_size = game.base_tile_size() as i32;
    let step = (game.base_tile_size() + game.base_tile_gap()) as i32;
    let (origin_x, origin_y) = game.base_origin(BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT);
    let local_x = mouse_x as i32 - origin_x;
    let local_y = mouse_y as i32 - origin_y;

    if local_x < 0 || local_y < 0 {
        return None;
    }

    let tile_x = local_x / step;
    let tile_y = local_y / step;
    if tile_x < 0 || tile_y < 0 || tile_x >= game.base.size || tile_y >= game.base.size {
        return None;
    }

    if local_x % step >= tile_size || local_y % step >= tile_size {
        return None;
    }

    Some((tile_x, tile_y))
}

fn building_color(kind: BuildingKind) -> u32 {
    match kind {
        BuildingKind::Campfire => 0x9b7653,
        BuildingKind::Storehouse => 0x9b7653,
        BuildingKind::Workbench => 0x88a65e,
        BuildingKind::ResearchTable => 0x6b8fd6,
        BuildingKind::TrainingDummy => 0xc47b6b,
    }
}

fn hovered_prep_card_effect(mouse_x: usize, mouse_y: usize) -> Option<&'static [u8]> {
    let panel_x = BASE_VIEWPORT_WIDTH / 2 - 316;
    let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 212;
    for (index, card) in Card::available_cards().iter().enumerate() {
        let x = panel_x + 40 + (index % 3) * 126;
        let y = panel_y + 96 + (index / 3) * 126;
        if point_in_rect(mouse_x, mouse_y, x, y, 100, 104) {
            return Some(match card.kind {
                CardKind::Move => EFFECT_MOVE_LABEL,
                CardKind::Gather => EFFECT_GATHER_LABEL,
                _ => EFFECT_MOVE_LABEL,
            });
        }
    }

    None
}

fn hovered_expedition_tile_label(
    expedition: &crate::game::ExpeditionState,
    mouse_x: usize,
    mouse_y: usize,
) -> Option<&'static [u8]> {
    if mouse_y >= EXPEDITION_VIEWPORT_HEIGHT {
        return None;
    }

    let tile_size = expedition.tile_size() as i32;
    let (origin_x, origin_y) = expedition.origin(WIDTH, EXPEDITION_VIEWPORT_HEIGHT);
    let local_x = mouse_x as i32 - origin_x;
    let local_y = mouse_y as i32 - origin_y;
    if local_x < 0 || local_y < 0 {
        return None;
    }

    let tile_x = local_x / tile_size;
    let tile_y = local_y / tile_size;
    if tile_x < 0 || tile_y < 0 || tile_x >= expedition.map.width || tile_y >= expedition.map.height
    {
        return None;
    }

    match expedition.map.tile_at(tile_x, tile_y) {
        Tile::Forest => Some(RES_FOREST_INFO_LABEL),
        Tile::Stone => Some(RES_STONE_INFO_LABEL),
        Tile::Berry => Some(RES_BERRY_INFO_LABEL),
        Tile::Enemy => Some(RES_ENEMY_INFO_LABEL),
        Tile::Ruin => Some(RES_RUIN_INFO_LABEL),
        Tile::Ground => None,
    }
}

fn is_ruin_anchor(map: &crate::game::ExplorationMap, x: i32, y: i32) -> bool {
    let left_is_ruin = x > 0 && map.tile_at(x - 1, y) == Tile::Ruin;
    let top_is_ruin = y > 0 && map.tile_at(x, y - 1) == Tile::Ruin;
    !left_is_ruin && !top_is_ruin
}

fn hovered_expedition_card_effect(
    expedition: &crate::game::ExpeditionState,
    mouse_x: usize,
    mouse_y: usize,
) -> Option<&'static [u8]> {
    if mouse_y < EXPEDITION_HAND_Y || mouse_y >= EXPEDITION_HAND_Y + 84 {
        return None;
    }

    for (index, card) in expedition.hand.iter().enumerate() {
        let x = 28 + index * 108;
        if point_in_rect(mouse_x, mouse_y, x, EXPEDITION_HAND_Y, 92, 84) {
            return Some(match card.kind {
                CardKind::Move => EFFECT_MOVE_LABEL,
                CardKind::Gather => EFFECT_GATHER_LABEL,
                _ => EFFECT_MOVE_LABEL,
            });
        }
    }

    None
}

fn building_usage_label(kind: BuildingKind) -> &'static [u8] {
    match kind {
        BuildingKind::Campfire => USE_SMALL_STOREHOUSE_LABEL,
        BuildingKind::Storehouse => USE_STOREHOUSE_LABEL,
        BuildingKind::Workbench => USE_WORKBENCH_LABEL,
        BuildingKind::ResearchTable => USE_RESEARCH_TABLE_LABEL,
        BuildingKind::TrainingDummy => USE_TRAINING_DUMMY_LABEL,
    }
}

fn card_name_label(kind: CardKind) -> &'static [u8] {
    match kind {
        CardKind::Move => CARD_MOVE_LABEL,
        CardKind::Gather => CARD_GATHER_LABEL,
        CardKind::Search => CARD_SEARCH_LABEL,
        CardKind::Attack => CARD_ATTACK_LABEL,
        CardKind::Defend => CARD_DEFEND_LABEL,
        CardKind::Skill => CARD_SKILL_LABEL,
    }
}

fn card_kind_label(kind: CardKind) -> &'static [u8] {
    match kind {
        CardKind::Move => KIND_MOVE_LABEL,
        CardKind::Gather => KIND_GATHER_LABEL,
        CardKind::Search => KIND_SEARCH_LABEL,
        CardKind::Attack => KIND_ATTACK_LABEL,
        CardKind::Defend => KIND_DEFEND_LABEL,
        CardKind::Skill => KIND_SKILL_LABEL,
    }
}

fn label_width(label: &[u8]) -> usize {
    if label.len() < 2 {
        0
    } else {
        u16::from_le_bytes([label[0], label[1]]) as usize
    }
}

fn glyph(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'B' => [0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e],
        'C' => [0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e],
        'D' => [0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e],
        'E' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f],
        'F' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10],
        'G' => [0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0f],
        'H' => [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'I' => [0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0e],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f],
        'M' => [0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'P' => [0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10],
        'Q' => [0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d],
        'R' => [0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11],
        'S' => [0x0f, 0x10, 0x10, 0x0e, 0x01, 0x01, 0x1e],
        'T' => [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0a, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1b, 0x11],
        'X' => [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f],
        '0' => [0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e],
        '1' => [0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e],
        '2' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1f],
        '3' => [0x1e, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x1e],
        '4' => [0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02],
        '5' => [0x1f, 0x10, 0x10, 0x1e, 0x01, 0x01, 0x1e],
        '6' => [0x0e, 0x10, 0x10, 0x1e, 0x11, 0x11, 0x0e],
        '7' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e],
        '9' => [0x0e, 0x11, 0x11, 0x0f, 0x01, 0x01, 0x0e],
        ':' => [0x00, 0x04, 0x04, 0x00, 0x04, 0x04, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x08],
        '-' => [0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00],
        '/' => [0x01, 0x01, 0x02, 0x04, 0x08, 0x10, 0x10],
        '+' => [0x00, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x00],
        '[' => [0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0e],
        ']' => [0x0e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x0e],
        '!' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '?' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
        ' ' => [0x00; 7],
        _ => [0x1f, 0x11, 0x05, 0x02, 0x05, 0x11, 0x1f],
    }
}
