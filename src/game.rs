use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::card::{Card, CardKind};
use crate::data::{Building, BuildingKind, Character, Resources, Technology};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    MainMenu,
    Base,
    Expedition,
    GameOver,
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
    pub should_quit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseState {
    pub size: i32,
    pub buildings: Vec<Building>,
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
            message: "Press Enter or N to begin.".to_string(),
            should_quit: false,
        }
    }

    pub fn new_campaign(&mut self) {
        *self = Game::new();
        self.screen = Screen::Base;
        self.message = "New campaign started. B build, T research, E explore.".to_string();
    }

    pub fn update(&mut self, _dt: f32) {
        if self.characters.iter().all(|character| !character.alive) {
            self.screen = Screen::GameOver;
            self.message = "All characters are dead. Campaign failed.".to_string();
        }
    }

    pub fn confirm(&mut self) {
        match self.screen {
            Screen::MainMenu | Screen::GameOver => self.new_campaign(),
            Screen::Base => self.start_expedition(),
            Screen::Expedition => self.play_next_card(),
        }
    }

    pub fn back_or_quit(&mut self) {
        match self.screen {
            Screen::MainMenu => self.should_quit = true,
            Screen::Base => {
                self.screen = Screen::MainMenu;
                self.message = "Returned to main menu.".to_string();
            }
            Screen::Expedition => self.evacuate(),
            Screen::GameOver => {
                self.screen = Screen::MainMenu;
                self.message = "Press N to start again.".to_string();
            }
        }
    }

    pub fn build_basic_structure(&mut self) {
        let cost = Resources {
            wood: 3,
            stone: 1,
            food: 0,
        };

        if !self.resources.can_afford(&cost) {
            self.message = "Not enough resources to build.".to_string();
            return;
        }

        let next_index = self.base.buildings.len() as i32;
        let x = 2 + next_index % (self.base.size - 4).max(1);
        let y = 2 + next_index / (self.base.size - 4).max(1);
        let kind = match self.base.buildings.len() % 5 {
            0 => BuildingKind::Campfire,
            1 => BuildingKind::Storehouse,
            2 => BuildingKind::Workbench,
            3 => BuildingKind::ResearchTable,
            _ => BuildingKind::TrainingDummy,
        };

        self.resources.spend(&cost);
        self.base.buildings.push(Building {
            kind,
            x,
            y,
            level: 1,
        });
        self.message = format!("Built {:?}.", kind);
    }

    pub fn unlock_first_technology(&mut self) {
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
        self.message = "Unlocked Base Expansion I. Base expanded.".to_string();
    }

    pub fn start_expedition(&mut self) {
        self.expedition = Some(ExpeditionState::new());
        self.screen = Screen::Expedition;
        self.message =
            "Expedition started. Space plays cards, Tab ends turn, V evacuates.".to_string();
    }

    pub fn play_next_card(&mut self) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };

        if expedition.hand.is_empty() {
            self.message = "No cards in hand. End the turn.".to_string();
            return;
        }

        let card = expedition.hand.remove(0);
        if expedition.energy < card.cost as i32 {
            expedition.hand.insert(0, card);
            self.message = "Not enough energy.".to_string();
            return;
        }

        expedition.energy -= card.cost as i32;
        let text = expedition.apply_card(&card);
        expedition.discard.push(card);
        self.message = text;
    }

    pub fn end_turn(&mut self) {
        let Some(expedition) = self.expedition.as_mut() else {
            return;
        };

        expedition.enemy_phase(&mut self.characters);
        expedition.start_turn();
        self.message = format!(
            "Turn {}. Drew {} cards.",
            expedition.turn,
            expedition.hand.len()
        );
    }

    pub fn evacuate(&mut self) {
        if let Some(expedition) = self.expedition.take() {
            self.resources.add(&expedition.cargo);
            self.message = format!(
                "Evacuated with {} wood, {} stone, {} food.",
                expedition.cargo.wood, expedition.cargo.stone, expedition.cargo.food
            );
        } else {
            self.message = "No active expedition.".to_string();
        }

        self.screen = Screen::Base;
    }

    pub fn set_message<T: Into<String>>(&mut self, message: T) {
        self.message = message.into();
    }

    pub fn window_title(&self) -> String {
        format!(
            "Gruntia | {:?} | Wood {} Stone {} Food {} | {}",
            self.screen,
            self.resources.wood,
            self.resources.stone,
            self.resources.food,
            self.message
        )
    }
}

impl BaseState {
    fn starter() -> BaseState {
        BaseState {
            size: 16,
            buildings: Vec::new(),
        }
    }
}

impl ExpeditionState {
    fn new() -> ExpeditionState {
        let mut deck = Card::starter_deck();
        deck.shuffle(&mut thread_rng());

        let mut expedition = ExpeditionState {
            map: ExplorationMap::random(32, 32),
            player_x: 16,
            player_y: 16,
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
            CardKind::Move => {
                self.player_x = (self.player_x + card.value).clamp(0, self.map.width - 1);
                format!("Played {}. Moved east.", card.name)
            }
            CardKind::Gather => {
                let tile = self.map.tile_at(self.player_x, self.player_y);
                match tile {
                    Tile::Forest => {
                        self.cargo.wood += card.value;
                        self.map
                            .set_tile(self.player_x, self.player_y, Tile::Ground);
                        format!("Played {}. Gathered wood.", card.name)
                    }
                    Tile::Stone => {
                        self.cargo.stone += card.value;
                        self.map
                            .set_tile(self.player_x, self.player_y, Tile::Ground);
                        format!("Played {}. Gathered stone.", card.name)
                    }
                    _ => "Nothing useful to gather here.".to_string(),
                }
            }
            CardKind::Search => {
                self.cargo.food += 1;
                format!("Played {}. Found food.", card.name)
            }
            CardKind::Attack => {
                self.enemy_hp = (self.enemy_hp - card.value).max(0);
                format!("Played {}. Enemy HP is now {}.", card.name, self.enemy_hp)
            }
            CardKind::Defend => {
                self.block += card.value;
                format!("Played {}. Block is now {}.", card.name, self.block)
            }
            CardKind::Skill => format!("Played {}.", card.name),
        }
    }

    fn enemy_phase(&mut self, characters: &mut [Character]) {
        if self.enemy_hp <= 0 {
            self.enemy_hp = thread_rng().gen_range(8..=16);
            self.cargo.food += 2;
            return;
        }

        let damage = (3 - self.block).max(0);
        if damage == 0 {
            return;
        }

        if let Some(character) = characters.iter_mut().find(|character| character.alive) {
            character.hp -= damage;
            if character.hp <= 0 {
                character.hp = 0;
                character.alive = false;
            }
        }
    }
}

impl ExplorationMap {
    fn random(width: i32, height: i32) -> ExplorationMap {
        let mut rng = thread_rng();
        let mut tiles = Vec::with_capacity((width * height) as usize);

        for _ in 0..width * height {
            let roll = rng.gen_range(0..100);
            let tile = match roll {
                0..=9 => Tile::Forest,
                10..=15 => Tile::Stone,
                16..=18 => Tile::Enemy,
                19..=20 => Tile::Ruin,
                _ => Tile::Ground,
            };
            tiles.push(tile);
        }

        ExplorationMap {
            width,
            height,
            tiles,
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.tiles[self.index(x, y)]
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let index = self.index(x, y);
        self.tiles[index] = tile;
    }
}
