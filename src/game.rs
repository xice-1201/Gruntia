use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::card::{Card, CardKind};
use crate::data::{Building, BuildingKind, Character, Resources, Technology};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    MainMenu,
    TransitionToBase,
    TransitionToExpedition,
    Base,
    Expedition,
    GameOver,
}

pub const MAX_PREP_CARDS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepartBlockReason {
    EmptyDeck,
    Food,
    Capacity,
}

impl DepartBlockReason {
    fn message(self) -> &'static str {
        match self {
            DepartBlockReason::EmptyDeck => "Choose at least one card before departing.",
            DepartBlockReason::Food => "Not enough food to depart.",
            DepartBlockReason::Capacity => "Storage would remain over capacity after preparation.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub screen: Screen,
    pub base: BaseState,
    pub expedition: Option<ExpeditionState>,
    pub resources: Resources,
    pub characters: Vec<Character>,
    pub technologies: Vec<Technology>,
    pub message: String,
    #[serde(default)]
    pub show_storage_dialog: bool,
    #[serde(default)]
    pub show_build_menu: bool,
    #[serde(default)]
    pub show_expedition_prep: bool,
    #[serde(default)]
    pub show_expedition_bag: bool,
    #[serde(default)]
    pub show_character_dialog: bool,
    #[serde(default)]
    pub show_crafting_dialog: bool,
    #[serde(default)]
    pub build_menu_category: usize,
    #[serde(default)]
    pub confirm_quit: bool,
    #[serde(default = "default_storage_discard_inputs")]
    pub storage_discard_inputs: [i32; 3],
    #[serde(default = "default_discovered_resources")]
    pub discovered_resources: [bool; 3],
    #[serde(default)]
    pub focused_storage_input: Option<usize>,
    #[serde(default)]
    pub pending_storage_discard: Option<usize>,
    #[serde(default)]
    pub prep_deck: Vec<Card>,
    #[serde(default)]
    pub pending_expedition_deck: Vec<Card>,
    #[serde(default)]
    pub selected_building: Option<BuildingKind>,
    #[serde(default)]
    pub transition_timer: f32,
    pub should_quit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseState {
    pub size: i32,
    pub buildings: Vec<Building>,
    #[serde(default = "default_storage_capacity")]
    pub storage_capacity: i32,
    #[serde(default = "default_base_zoom")]
    pub zoom_level: usize,
    #[serde(default)]
    pub camera_x: f32,
    #[serde(default)]
    pub camera_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpeditionState {
    pub map: ExplorationMap,
    pub player_x: i32,
    pub player_y: i32,
    pub turn: u32,
    pub energy: i32,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub cargo: Resources,
    pub enemy_hp: i32,
    pub block: i32,
    #[serde(default = "default_enemy_move_speed")]
    pub enemy_move_speed: i32,
    #[serde(default = "default_enemy_alert_range")]
    pub enemy_alert_range: i32,
    #[serde(default)]
    pub pending_move_card: Option<Card>,
    #[serde(default = "default_expedition_zoom")]
    pub zoom_level: usize,
    #[serde(default)]
    pub camera_x: f32,
    #[serde(default)]
    pub camera_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Ground,
    Forest,
    Stone,
    Berry,
    Enemy,
    Ruin,
}

impl Game {
    pub fn new() -> Game {
        Game {
            screen: Screen::MainMenu,
            base: BaseState::starter(),
            expedition: None,
            resources: Resources::starter(),
            characters: Character::starter_roster(),
            technologies: Technology::starter_tree(),
            message: "Select a menu option.".to_string(),
            show_storage_dialog: false,
            show_build_menu: false,
            show_expedition_prep: false,
            show_expedition_bag: false,
            show_character_dialog: false,
            show_crafting_dialog: false,
            build_menu_category: 0,
            confirm_quit: false,
            storage_discard_inputs: default_storage_discard_inputs(),
            discovered_resources: default_discovered_resources(),
            focused_storage_input: None,
            pending_storage_discard: None,
            prep_deck: Vec::new(),
            pending_expedition_deck: Vec::new(),
            selected_building: None,
            transition_timer: 0.0,
            should_quit: false,
        }
    }

    pub fn begin_new_campaign_transition(&mut self) {
        self.screen = Screen::TransitionToBase;
        self.transition_timer = 0.0;
        self.message = "Preparing base...".to_string();
    }

    pub fn new_campaign(&mut self) {
        *self = Game::new();
        self.screen = Screen::Base;
        self.message = "New campaign started. B build, T research, E explore.".to_string();
    }

    pub fn update(&mut self, dt: f32) {
        if self.screen == Screen::TransitionToBase {
            self.transition_timer += dt;
            if self.transition_timer >= 0.9 {
                self.new_campaign();
            }
            return;
        }

        if self.screen == Screen::TransitionToExpedition {
            self.transition_timer += dt;
            if self.transition_timer >= 0.75 {
                self.enter_expedition();
            }
            return;
        }

        self.transition_timer += dt;

        if self.characters.iter().all(|character| !character.alive) {
            self.screen = Screen::GameOver;
            self.message = "All characters are dead. Campaign failed.".to_string();
        }
    }

    pub fn back_or_quit(&mut self) {
        match self.screen {
            Screen::MainMenu => self.should_quit = true,
            Screen::TransitionToBase => {}
            Screen::TransitionToExpedition => {}
            Screen::Base => {
                self.screen = Screen::MainMenu;
                self.message = "Select a menu option.".to_string();
            }
            Screen::Expedition => self.evacuate(),
            Screen::GameOver => {
                self.screen = Screen::MainMenu;
                self.message = "Select a menu option.".to_string();
            }
        }
    }

    pub fn open_build_menu(&mut self) {
        self.confirm_quit = false;
        self.show_storage_dialog = false;
        self.show_expedition_prep = false;
        self.show_character_dialog = false;
        self.show_crafting_dialog = false;
        self.show_build_menu = true;
        self.selected_building = None;
        self.build_menu_category = 0;
        self.message = "Choose a building.".to_string();
    }

    pub fn close_build_menu(&mut self) {
        self.show_build_menu = false;
        self.message = "Build menu closed.".to_string();
    }

    pub fn set_build_menu_category(&mut self, category: usize) {
        self.build_menu_category = category.min(1);
        self.message = "Choose a building.".to_string();
    }

    pub fn select_building(&mut self, kind: BuildingKind) {
        self.show_build_menu = false;
        self.show_expedition_prep = false;
        self.selected_building = Some(kind);
        self.message = format!(
            "Selected {}. Click a base tile.",
            building_display_name(kind)
        );
    }

    pub fn cancel_building_placement(&mut self) {
        self.selected_building = None;
        self.message = "Building placement cancelled.".to_string();
    }

    pub fn cancel_building_if_selected(&mut self) {
        if self.selected_building.is_some() {
            self.selected_building = None;
            self.message = "Building placement cancelled.".to_string();
        }
    }

    pub fn place_selected_building(&mut self, x: i32, y: i32) {
        let Some(kind) = self.selected_building else {
            return;
        };

        if x < 0 || y < 0 || x >= self.base.size || y >= self.base.size {
            self.message = "Cannot build outside the base.".to_string();
            return;
        }

        if self
            .base
            .buildings
            .iter()
            .any(|building| building.x == x && building.y == y)
        {
            self.message = "That tile is already occupied.".to_string();
            return;
        }

        let cost = building_cost(kind);
        if !self.resources.can_afford(&cost) {
            self.message = "Not enough resources to build.".to_string();
            return;
        }

        self.resources.spend(&cost);
        self.base.buildings.push(Building {
            kind,
            x,
            y,
            level: 1,
        });
        if kind == BuildingKind::Campfire {
            self.base.storage_capacity += 10;
        }
        self.selected_building = None;
        self.message = format!("Built {}.", building_display_name(kind));
    }

    pub fn unlock_first_technology(&mut self) {
        if !self.has_building(BuildingKind::ResearchTable) {
            self.message = "Build a Research Table first.".to_string();
            return;
        }

        let cost = Resources {
            wood: 5,
            stone: 5,
            food: 0,
        };

        if self.technologies[0].unlocked {
            self.message = "Technology already unlocked.".to_string();
            return;
        }

        if !self.resources.can_afford(&cost) {
            self.message = "Need 5 wood and 5 stone for Base Expansion I.".to_string();
            return;
        }

        self.resources.spend(&cost);
        self.technologies[0].unlocked = true;
        self.base.size += 8;
        self.clamp_base_camera(1000, 696);
        self.message = "Unlocked Base Expansion I. Base expanded.".to_string();
    }

    pub fn open_expedition_prep(&mut self) {
        self.show_storage_dialog = false;
        self.show_build_menu = false;
        self.show_character_dialog = false;
        self.show_crafting_dialog = false;
        self.show_expedition_prep = true;
        self.prep_deck.clear();
        self.selected_building = None;
        self.message = "Choose cards for the expedition.".to_string();
    }

    pub fn close_expedition_prep(&mut self) {
        self.show_expedition_prep = false;
        self.prep_deck.clear();
        self.message = "Expedition preparation cancelled.".to_string();
    }

    pub fn add_prep_card(&mut self, card: Card) {
        if self.prep_deck.len() >= MAX_PREP_CARDS {
            self.message = "The expedition deck is full.".to_string();
            return;
        }

        self.prep_deck.push(card);
        self.message = format!("Selected {} card(s).", self.prep_deck.len());
    }

    pub fn remove_prep_card(&mut self, index: usize) {
        if index < self.prep_deck.len() {
            self.prep_deck.remove(index);
            self.message = format!("Selected {} card(s).", self.prep_deck.len());
        }
    }

    pub fn prep_food_cost(&self) -> i32 {
        self.prep_deck
            .iter()
            .map(|card| card.food_cost as i32)
            .sum()
    }

    pub fn depart_block_reason(&self) -> Option<DepartBlockReason> {
        if self.prep_deck.is_empty() {
            return Some(DepartBlockReason::EmptyDeck);
        }

        let food_cost = self.prep_food_cost();
        if self.resources.food < food_cost {
            return Some(DepartBlockReason::Food);
        }

        let remaining_used = self.storage_used() - food_cost;
        if remaining_used > self.storage_capacity() {
            return Some(DepartBlockReason::Capacity);
        }

        None
    }

    pub fn start_expedition(&mut self, deck: Vec<Card>) {
        if let Some(reason) = self.depart_block_reason() {
            self.message = reason.message().to_string();
            return;
        }

        let food_cost = deck.iter().map(|card| card.food_cost as i32).sum::<i32>();
        self.resources.food -= food_cost;
        self.show_expedition_prep = false;
        self.pending_expedition_deck = deck;
        self.prep_deck.clear();
        self.transition_timer = 0.0;
        self.screen = Screen::TransitionToExpedition;
        self.message = "Departing for the expedition...".to_string();
    }

    fn enter_expedition(&mut self) {
        let deck = std::mem::take(&mut self.pending_expedition_deck);
        self.expedition = Some(ExpeditionState::new(deck));
        self.screen = Screen::Expedition;
        self.show_expedition_bag = false;
        self.message = "Expedition started. Drag cards to play.".to_string();
    }

    pub fn play_hand_card(&mut self, index: usize) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };

        if expedition.hand.is_empty() {
            self.message = "\u{624b}\u{724c}\u{4e3a}\u{7a7a}\u{ff0c}\u{8bf7}\u{7ed3}\u{675f}\u{56de}\u{5408}\u{3002}".to_string();
            return;
        }

        if index >= expedition.hand.len() {
            return;
        }

        if expedition.pending_move_card.is_some() {
            self.message = "Choose a move destination first.".to_string();
            return;
        }

        if expedition.hand[index].kind == CardKind::Gather && !expedition.has_gather_targets() {
            self.message = "No resources in gather range.".to_string();
            return;
        }

        let card = expedition.hand.remove(index);
        if card.kind == CardKind::Move {
            expedition.pending_move_card = Some(card);
            self.message = "Choose a highlighted destination.".to_string();
        } else {
            let text = expedition.apply_card(&card);
            expedition.discard.push(card);
            expedition.move_enemies();
            self.message = text;
        }
    }

    pub fn reorder_hand_card(&mut self, from: usize, to: usize) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };
        if from >= expedition.hand.len() {
            return;
        }

        let card = expedition.hand.remove(from);
        let target = to.min(expedition.hand.len());
        expedition.hand.insert(target, card);
        self.message = "Hand reordered.".to_string();
    }

    pub fn choose_move_destination(&mut self, x: i32, y: i32) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };
        let Some(card) = expedition.pending_move_card.take() else {
            return;
        };

        if !expedition.can_move_to(x, y, card.value) {
            expedition.pending_move_card = Some(card);
            self.message = "Choose a highlighted destination.".to_string();
            return;
        }

        expedition.player_x = x;
        expedition.player_y = y;
        expedition.discard.push(card);
        expedition.move_enemies();
        self.message = "Moved.".to_string();
    }

    pub fn evacuate(&mut self) {
        self.show_expedition_bag = false;
        if let Some(expedition) = self.expedition.take() {
            self.resources.add(&expedition.cargo);
            self.refresh_discovered_resources();
            if self.resources.food < 3 {
                self.screen = Screen::GameOver;
                self.message = format!(
                    "Evacuated with {} wood, {} stone, {} food, but food was below 3. Campaign failed.",
                    expedition.cargo.wood, expedition.cargo.stone, expedition.cargo.food
                );
                return;
            }

            self.resources.food -= 3;
            self.message = format!(
                "Evacuated with {} wood, {} stone, {} food. Consumed 3 food.",
                expedition.cargo.wood, expedition.cargo.stone, expedition.cargo.food
            );
        } else {
            self.message = "No active expedition.".to_string();
        }

        self.screen = Screen::Base;
    }

    pub fn show_expedition_bag(&mut self) {
        self.show_expedition_bag = true;
        self.message = "Expedition bag opened.".to_string();
    }

    pub fn hide_expedition_bag(&mut self) {
        self.show_expedition_bag = false;
        self.message = "Expedition bag closed.".to_string();
    }

    pub fn set_message<T: Into<String>>(&mut self, message: T) {
        self.message = message.into();
    }

    pub fn show_storage_dialog(&mut self) {
        self.confirm_quit = false;
        self.show_character_dialog = false;
        self.show_crafting_dialog = false;
        self.show_build_menu = false;
        self.show_expedition_prep = false;
        self.selected_building = None;
        self.show_storage_dialog = true;
        self.focused_storage_input = None;
        self.pending_storage_discard = None;
        self.message = "Storage opened.".to_string();
    }

    pub fn hide_storage_dialog(&mut self) {
        self.show_storage_dialog = false;
        self.focused_storage_input = None;
        self.pending_storage_discard = None;
        self.message = "Storage closed.".to_string();
    }

    pub fn storage_capacity(&self) -> i32 {
        self.base.storage_capacity
    }

    pub fn storage_used(&self) -> i32 {
        self.resources.total()
    }

    pub fn storage_is_over_capacity(&self) -> bool {
        self.storage_used() > self.storage_capacity()
    }

    pub fn visible_storage_indices(&self) -> Vec<usize> {
        (0..3)
            .filter(|index| self.discovered_resources[*index] || self.resources.amount(*index) > 0)
            .collect()
    }

    pub fn refresh_discovered_resources(&mut self) {
        for index in 0..3 {
            if self.resources.amount(index) > 0 {
                self.discovered_resources[index] = true;
            }
        }
    }

    pub fn focus_storage_input(&mut self, index: usize) {
        if index < 3 {
            self.focused_storage_input = Some(index);
            self.pending_storage_discard = None;
        }
    }

    pub fn adjust_storage_discard_input(&mut self, index: usize, delta: i32) {
        if index >= 3 {
            return;
        }
        let max_amount = self.resources.amount(index);
        self.storage_discard_inputs[index] =
            (self.storage_discard_inputs[index] + delta).clamp(0, max_amount);
        self.focused_storage_input = Some(index);
        self.pending_storage_discard = None;
    }

    pub fn append_storage_discard_digit(&mut self, digit: i32) {
        let Some(index) = self.focused_storage_input else {
            return;
        };
        let max_amount = self.resources.amount(index);
        self.storage_discard_inputs[index] =
            (self.storage_discard_inputs[index] * 10 + digit).clamp(0, max_amount);
        self.pending_storage_discard = None;
    }

    pub fn backspace_storage_discard_input(&mut self) {
        let Some(index) = self.focused_storage_input else {
            return;
        };
        self.storage_discard_inputs[index] /= 10;
        self.pending_storage_discard = None;
    }

    pub fn request_storage_discard(&mut self, index: usize) {
        if index >= 3 {
            return;
        }
        let amount = self.storage_discard_inputs[index];
        if amount <= 0 {
            self.message = "Discard amount must be greater than 0.".to_string();
            return;
        }
        if amount > self.resources.amount(index) {
            self.message = "Not enough material to discard.".to_string();
            return;
        }
        self.pending_storage_discard = Some(index);
        self.message = "Confirm discard.".to_string();
    }

    pub fn confirm_storage_discard(&mut self) {
        let Some(index) = self.pending_storage_discard.take() else {
            return;
        };
        let amount = self.storage_discard_inputs[index];
        let discarded = self.resources.discard(index, amount);
        self.refresh_discovered_resources();
        self.storage_discard_inputs[index] = 0;
        self.focused_storage_input = Some(index);
        self.message = format!("Discarded {discarded} material.");
    }

    pub fn cancel_storage_discard(&mut self) {
        self.pending_storage_discard = None;
        self.message = "Discard cancelled.".to_string();
    }

    pub fn request_quit_confirmation(&mut self) {
        self.confirm_quit = true;
        self.message = "Confirm quit.".to_string();
    }

    pub fn cancel_quit_confirmation(&mut self) {
        self.confirm_quit = false;
        self.message = "Quit cancelled.".to_string();
    }

    pub fn show_character_dialog(&mut self) {
        self.confirm_quit = false;
        self.show_storage_dialog = false;
        self.show_crafting_dialog = false;
        self.show_build_menu = false;
        self.show_expedition_prep = false;
        self.selected_building = None;
        self.show_character_dialog = true;
        self.message = "Character opened.".to_string();
    }

    pub fn hide_character_dialog(&mut self) {
        self.show_character_dialog = false;
        self.message = "Character closed.".to_string();
    }

    pub fn show_crafting_dialog(&mut self) {
        if !self.has_building(BuildingKind::Workbench) {
            self.message = "Build a Workbench first.".to_string();
            return;
        }

        self.confirm_quit = false;
        self.show_storage_dialog = false;
        self.show_character_dialog = false;
        self.show_build_menu = false;
        self.show_expedition_prep = false;
        self.selected_building = None;
        self.show_crafting_dialog = true;
        self.message = "Crafting opened.".to_string();
    }

    pub fn hide_crafting_dialog(&mut self) {
        self.show_crafting_dialog = false;
        self.message = "Crafting closed.".to_string();
    }

    pub fn has_building(&self, kind: BuildingKind) -> bool {
        self.base
            .buildings
            .iter()
            .any(|building| building.kind == kind)
    }

    pub fn adjust_base_zoom(&mut self, delta: i32) {
        let next = (self.base.zoom_level as i32 + delta).clamp(0, 4) as usize;
        if next != self.base.zoom_level {
            self.base.zoom_level = next;
            self.clamp_base_camera(1000, 696);
            self.message = format!("Base zoom {}x.", self.base.zoom_level + 1);
        }
    }

    pub fn pan_base_camera(
        &mut self,
        dx: f32,
        dy: f32,
        viewport_width: usize,
        viewport_height: usize,
    ) {
        self.base.camera_x += dx;
        self.base.camera_y += dy;
        self.clamp_base_camera(viewport_width, viewport_height);
    }

    pub fn clamp_base_camera(&mut self, viewport_width: usize, viewport_height: usize) {
        let (content_width, content_height) = self.base_content_size();
        self.base.camera_x = clamp_camera_axis(self.base.camera_x, content_width, viewport_width);
        self.base.camera_y = clamp_camera_axis(self.base.camera_y, content_height, viewport_height);
    }

    pub fn base_origin(&self, viewport_width: usize, viewport_height: usize) -> (i32, i32) {
        let (content_width, content_height) = self.base_content_size();
        let x = if content_width <= viewport_width {
            ((viewport_width - content_width) / 2) as i32
        } else {
            -(self.base.camera_x.round() as i32)
        };
        let y = if content_height <= viewport_height {
            ((viewport_height - content_height) / 2) as i32
        } else {
            -(self.base.camera_y.round() as i32)
        };
        (x, y)
    }

    pub fn base_content_size(&self) -> (usize, usize) {
        let size = self.base.size.max(0) as usize;
        if size == 0 {
            return (0, 0);
        }

        let tile_size = self.base_tile_size();
        let gap = self.base_tile_gap();
        let total = size * tile_size + size.saturating_sub(1) * gap;
        (total, total)
    }

    pub fn base_tile_size(&self) -> usize {
        12 + self.base.zoom_level * 4
    }

    pub fn base_tile_gap(&self) -> usize {
        3 + self.base.zoom_level
    }

    pub fn adjust_expedition_zoom(
        &mut self,
        delta: i32,
        viewport_width: usize,
        viewport_height: usize,
    ) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };
        let next = (expedition.zoom_level as i32 + delta).clamp(0, 5) as usize;
        if next != expedition.zoom_level {
            expedition.zoom_level = next;
            expedition.clamp_camera(viewport_width, viewport_height);
            self.message = format!("Expedition zoom {}x.", expedition.zoom_level + 1);
        }
    }

    pub fn pan_expedition_camera(
        &mut self,
        dx: f32,
        dy: f32,
        viewport_width: usize,
        viewport_height: usize,
    ) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };
        expedition.camera_x += dx;
        expedition.camera_y += dy;
        expedition.clamp_camera(viewport_width, viewport_height);
    }

    pub fn transition_progress(&self) -> f32 {
        let duration = match self.screen {
            Screen::TransitionToExpedition => 0.75,
            _ => 0.9,
        };
        (self.transition_timer / duration).clamp(0.0, 1.0)
    }

    pub fn window_title(&self) -> String {
        format!("Gruntia | {:?} | {}", self.screen, self.message)
    }
}

impl BaseState {
    fn starter() -> BaseState {
        BaseState {
            size: 5,
            buildings: Vec::new(),
            storage_capacity: 10,
            zoom_level: 1,
            camera_x: 0.0,
            camera_y: 0.0,
        }
    }
}

fn default_base_zoom() -> usize {
    1
}

fn default_storage_capacity() -> i32 {
    10
}

fn default_storage_discard_inputs() -> [i32; 3] {
    [0; 3]
}

fn default_discovered_resources() -> [bool; 3] {
    [false, false, true]
}

fn default_expedition_zoom() -> usize {
    3
}

fn default_enemy_move_speed() -> i32 {
    1
}

fn default_enemy_alert_range() -> i32 {
    6
}

fn clamp_camera_axis(value: f32, content_size: usize, viewport_size: usize) -> f32 {
    if content_size <= viewport_size {
        0.0
    } else {
        value.clamp(0.0, (content_size - viewport_size) as f32)
    }
}

pub fn building_cost(kind: BuildingKind) -> Resources {
    match kind {
        BuildingKind::Campfire => Resources {
            wood: 4,
            stone: 0,
            food: 0,
        },
        BuildingKind::Storehouse => Resources {
            wood: 4,
            stone: 2,
            food: 0,
        },
        BuildingKind::Workbench => Resources {
            wood: 5,
            stone: 1,
            food: 0,
        },
        BuildingKind::ResearchTable => Resources {
            wood: 5,
            stone: 3,
            food: 0,
        },
        BuildingKind::TrainingDummy => Resources {
            wood: 3,
            stone: 0,
            food: 1,
        },
    }
}

fn building_display_name(kind: BuildingKind) -> &'static str {
    match kind {
        BuildingKind::Campfire => "Small Storehouse",
        BuildingKind::Storehouse => "Storehouse",
        BuildingKind::Workbench => "Workbench",
        BuildingKind::ResearchTable => "Research Table",
        BuildingKind::TrainingDummy => "Training Dummy",
    }
}

impl ExpeditionState {
    fn new(mut deck: Vec<Card>) -> ExpeditionState {
        deck.shuffle(&mut thread_rng());
        let mut rng = thread_rng();
        let player_x = rng.gen_range(0..32);
        let player_y = rng.gen_range(0..32);

        let mut expedition = ExpeditionState {
            map: ExplorationMap::random(32, 32, player_x, player_y),
            player_x,
            player_y,
            turn: 0,
            energy: 0,
            deck,
            hand: Vec::new(),
            discard: Vec::new(),
            cargo: Resources {
                wood: 0,
                stone: 0,
                food: 0,
            },
            enemy_hp: 12,
            block: 0,
            enemy_move_speed: default_enemy_move_speed(),
            enemy_alert_range: default_enemy_alert_range(),
            pending_move_card: None,
            zoom_level: default_expedition_zoom(),
            camera_x: 0.0,
            camera_y: 0.0,
        };

        expedition.start_turn();
        expedition
    }

    fn start_turn(&mut self) {
        self.turn += 1;
        self.energy = 3;
        self.block = 0;
        self.discard.append(&mut self.hand);

        while self.hand.len() < 5 {
            if self.deck.is_empty() {
                if self.discard.is_empty() {
                    break;
                }

                self.deck.append(&mut self.discard);
                self.deck.shuffle(&mut thread_rng());
            }

            if let Some(card) = self.deck.pop() {
                self.hand.push(card);
            }
        }
    }

    fn apply_card(&mut self, card: &Card) -> String {
        match card.kind {
            CardKind::Move => "Moved.".to_string(),
            CardKind::Gather => {
                let mut wood = 0;
                let mut stone = 0;
                let mut food = 0;
                let mut rng = thread_rng();
                for (x, y) in self.gather_range_tiles() {
                    match self.map.tile_at(x, y) {
                        Tile::Forest => {
                            wood += rng.gen_range(1..=5);
                            self.map.set_tile(x, y, Tile::Ground);
                        }
                        Tile::Stone => {
                            stone += rng.gen_range(1..=2);
                            self.map.set_tile(x, y, Tile::Ground);
                        }
                        Tile::Berry => {
                            food += rng.gen_range(1..=3);
                            self.map.set_tile(x, y, Tile::Ground);
                        }
                        _ => {}
                    }
                }
                self.cargo.wood += wood;
                self.cargo.stone += stone;
                self.cargo.food += food;
                format!("Gathered W{} S{} F{}.", wood, stone, food)
            }
            CardKind::Search => {
                self.cargo.food += 1;
                format!(
                    "\u{6253}\u{51fa}{}\u{ff0c}\u{627e}\u{5230}\u{98df}\u{7269}\u{3002}",
                    card.name
                )
            }
            CardKind::Attack => {
                self.enemy_hp = (self.enemy_hp - card.value).max(0);
                format!("\u{6253}\u{51fa}{}\u{ff0c}\u{654c}\u{4eba}\u{751f}\u{547d}\u{964d}\u{81f3} {}\u{3002}", card.name, self.enemy_hp)
            }
            CardKind::Defend => {
                self.block += card.value;
                format!(
                    "\u{6253}\u{51fa}{}\u{ff0c}\u{683c}\u{6321}\u{63d0}\u{9ad8}\u{81f3} {}\u{3002}",
                    card.name, self.block
                )
            }
            CardKind::Skill => format!("\u{6253}\u{51fa}{}\u{3002}", card.name),
        }
    }

    pub fn can_move_to(&self, x: i32, y: i32, range: i32) -> bool {
        if x < 0 || y < 0 || x >= self.map.width || y >= self.map.height {
            return false;
        }

        if self.map.is_blocking(x, y) {
            return false;
        }

        let mut visited = vec![false; (self.map.width * self.map.height) as usize];
        let mut frontier = std::collections::VecDeque::new();
        frontier.push_back((self.player_x, self.player_y, 0));
        visited[self.map.index(self.player_x, self.player_y)] = true;

        while let Some((cx, cy, distance)) = frontier.pop_front() {
            if cx == x && cy == y {
                return distance > 0 && distance <= range;
            }
            if distance >= range {
                continue;
            }

            for (nx, ny) in [(cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)] {
                if nx < 0 || ny < 0 || nx >= self.map.width || ny >= self.map.height {
                    continue;
                }
                let index = self.map.index(nx, ny);
                if visited[index] || self.map.is_blocking(nx, ny) {
                    continue;
                }
                visited[index] = true;
                frontier.push_back((nx, ny, distance + 1));
            }
        }

        false
    }

    pub fn gather_range_tiles(&self) -> Vec<(i32, i32)> {
        const OFFSETS: [(i32, i32); 5] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
        OFFSETS
            .iter()
            .map(|(dx, dy)| (self.player_x + dx, self.player_y + dy))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.map.width && *y < self.map.height)
            .collect()
    }

    pub fn has_gather_targets(&self) -> bool {
        self.gather_range_tiles().iter().any(|(x, y)| {
            matches!(
                self.map.tile_at(*x, *y),
                Tile::Forest | Tile::Stone | Tile::Berry
            )
        })
    }

    fn move_enemies(&mut self) {
        let mut enemies = Vec::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.tile_at(x, y) == Tile::Enemy {
                    enemies.push((x, y));
                }
            }
        }

        let mut rng = thread_rng();
        for (mut x, mut y) in enemies {
            if self.map.tile_at(x, y) != Tile::Enemy {
                continue;
            }

            for _ in 0..self.enemy_move_speed.max(0) {
                let Some((nx, ny)) = self.next_enemy_step(x, y, &mut rng) else {
                    break;
                };
                self.map.set_tile(x, y, Tile::Ground);
                self.map.set_tile(nx, ny, Tile::Enemy);
                x = nx;
                y = ny;
            }
        }
    }

    fn next_enemy_step(&self, x: i32, y: i32, rng: &mut impl Rng) -> Option<(i32, i32)> {
        let distance = (self.player_x - x).abs() + (self.player_y - y).abs();
        let mut directions = if distance <= self.enemy_alert_range {
            let mut preferred = Vec::new();
            if self.player_x > x {
                preferred.push((1, 0));
            } else if self.player_x < x {
                preferred.push((-1, 0));
            }
            if self.player_y > y {
                preferred.push((0, 1));
            } else if self.player_y < y {
                preferred.push((0, -1));
            }
            preferred.shuffle(rng);
            preferred.extend([(1, 0), (-1, 0), (0, 1), (0, -1)]);
            preferred
        } else {
            let mut dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            dirs.shuffle(rng);
            dirs
        };

        directions.dedup();
        directions.into_iter().find_map(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= self.map.width || ny >= self.map.height {
                return None;
            }
            if nx == self.player_x && ny == self.player_y {
                return None;
            }
            if self.map.tile_at(nx, ny) == Tile::Ground {
                Some((nx, ny))
            } else {
                None
            }
        })
    }

    pub fn tile_size(&self) -> usize {
        12 + self.zoom_level * 4
    }

    pub fn content_size(&self) -> (usize, usize) {
        (
            self.map.width.max(0) as usize * self.tile_size(),
            self.map.height.max(0) as usize * self.tile_size(),
        )
    }

    pub fn origin(&self, viewport_width: usize, viewport_height: usize) -> (i32, i32) {
        let (content_width, content_height) = self.content_size();
        let x = if content_width <= viewport_width {
            ((viewport_width - content_width) / 2) as i32
        } else {
            -(self.camera_x.round() as i32)
        };
        let y = if content_height <= viewport_height {
            ((viewport_height - content_height) / 2) as i32
        } else {
            -(self.camera_y.round() as i32)
        };
        (x, y)
    }

    pub fn clamp_camera(&mut self, viewport_width: usize, viewport_height: usize) {
        let (content_width, content_height) = self.content_size();
        self.camera_x = clamp_camera_axis(self.camera_x, content_width, viewport_width);
        self.camera_y = clamp_camera_axis(self.camera_y, content_height, viewport_height);
    }
}

impl ExplorationMap {
    fn random(width: i32, height: i32, player_x: i32, player_y: i32) -> ExplorationMap {
        let mut rng = thread_rng();
        let mut map = ExplorationMap {
            width,
            height,
            tiles: vec![Tile::Ground; (width * height) as usize],
        };
        map.seed_near_player_resources(&mut rng, player_x, player_y);
        map.seed_large_forests(&mut rng);
        map.seed_lone_trees(&mut rng);
        map.seed_stones(&mut rng);
        map.seed_berry_clusters(&mut rng);
        map.seed_enemies(&mut rng, player_x, player_y);
        map.seed_ruin(&mut rng, player_x, player_y);
        map.set_tile(player_x, player_y, Tile::Ground);
        map
    }

    fn seed_near_player_resources(&mut self, rng: &mut impl Rng, player_x: i32, player_y: i32) {
        for _ in 0..rng.gen_range(2..=4) {
            let x = (player_x + rng.gen_range(-3..=3)).clamp(0, self.width - 1);
            let y = (player_y + rng.gen_range(-3..=3)).clamp(0, self.height - 1);
            let tile = if rng.gen_bool(0.55) {
                Tile::Berry
            } else {
                Tile::Forest
            };
            let size = if tile == Tile::Berry {
                rng.gen_range(2..=5)
            } else {
                rng.gen_range(1..=4)
            };
            self.seed_cluster_from(rng, tile, size, x, y);
        }
    }

    fn seed_large_forests(&mut self, rng: &mut impl Rng) {
        let cluster_count = rng.gen_range(2..=4);
        for _ in 0..cluster_count {
            let size = rng.gen_range(15..=30);
            self.seed_cluster(rng, Tile::Forest, size);
        }
    }

    fn seed_lone_trees(&mut self, rng: &mut impl Rng) {
        let cluster_count = rng.gen_range(8..=14);
        for _ in 0..cluster_count {
            let size = if rng.gen_bool(0.78) {
                1
            } else {
                rng.gen_range(2..=4)
            };
            self.seed_cluster(rng, Tile::Forest, size);
        }
    }

    fn seed_stones(&mut self, rng: &mut impl Rng) {
        for _ in 0..rng.gen_range(12..=22) {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            self.set_tile(x, y, Tile::Stone);
        }
    }

    fn seed_berry_clusters(&mut self, rng: &mut impl Rng) {
        let cluster_count = rng.gen_range(5..=8);
        for _ in 0..cluster_count {
            let size = rng.gen_range(2..=6);
            self.seed_cluster(rng, Tile::Berry, size);
        }
    }

    fn seed_enemies(&mut self, rng: &mut impl Rng, player_x: i32, player_y: i32) {
        let attempts = rng.gen_range(8..=14);
        for _ in 0..attempts {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            let distance = (player_x - x).abs() + (player_y - y).abs();
            let chance =
                ((distance as f64 / (self.width + self.height) as f64) * 0.42).clamp(0.02, 0.32);
            if rng.gen_bool(chance) {
                self.set_tile(x, y, Tile::Enemy);
            }
        }
    }

    fn seed_ruin(&mut self, rng: &mut impl Rng, player_x: i32, player_y: i32) {
        if !rng.gen_bool(0.28) || self.width < 2 || self.height < 2 {
            return;
        }

        let mut best = None;
        let mut best_distance = -1;
        for _ in 0..30 {
            let candidate_x = rng.gen_range(0..self.width - 1);
            let candidate_y = rng.gen_range(0..self.height - 1);
            let distance = (player_x - candidate_x).abs() + (player_y - candidate_y).abs();
            if distance > best_distance {
                best = Some((candidate_x, candidate_y));
                best_distance = distance;
            }
            if distance >= 18 {
                break;
            }
        }
        let Some((x, y)) = best else {
            return;
        };
        for dy in 0..2 {
            for dx in 0..2 {
                self.set_tile(x + dx, y + dy, Tile::Ruin);
            }
        }
    }

    fn seed_cluster(&mut self, rng: &mut impl Rng, tile: Tile, target_size: i32) {
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.seed_cluster_from(rng, tile, target_size, start_x, start_y);
    }

    fn seed_cluster_from(
        &mut self,
        rng: &mut impl Rng,
        tile: Tile,
        target_size: i32,
        start_x: i32,
        start_y: i32,
    ) {
        let mut frontier = vec![(start_x, start_y)];
        let mut placed = 0;

        while placed < target_size {
            let Some((x, y)) = frontier.pop() else {
                break;
            };
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                continue;
            }

            self.set_tile(x, y, tile);
            placed += 1;

            let mut neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            neighbors.shuffle(rng);
            frontier.extend(neighbors.into_iter().take(3));
        }
    }

    pub fn index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.tiles[self.index(x, y)]
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let index = self.index(x, y);
        self.tiles[index] = tile;
    }

    pub fn is_blocking(&self, x: i32, y: i32) -> bool {
        matches!(self.tile_at(x, y), Tile::Enemy | Tile::Ruin)
    }
}
