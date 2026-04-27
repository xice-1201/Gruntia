use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub wood: i32,
    pub stone: i32,
    pub food: i32,
}

impl Resources {
    pub fn starter() -> Resources {
        Resources {
            wood: 0,
            stone: 0,
            food: 10,
        }
    }

    pub fn can_afford(&self, cost: &Resources) -> bool {
        self.wood >= cost.wood && self.stone >= cost.stone && self.food >= cost.food
    }

    pub fn spend(&mut self, cost: &Resources) {
        self.wood -= cost.wood;
        self.stone -= cost.stone;
        self.food -= cost.food;
    }

    pub fn add(&mut self, gain: &Resources) {
        self.wood += gain.wood;
        self.stone += gain.stone;
        self.food += gain.food;
    }

    pub fn total(&self) -> i32 {
        self.wood + self.stone + self.food
    }

    pub fn amount(&self, index: usize) -> i32 {
        match index {
            0 => self.wood,
            1 => self.stone,
            2 => self.food,
            _ => 0,
        }
    }

    pub fn discard(&mut self, index: usize, amount: i32) -> i32 {
        let amount = amount.max(0).min(self.amount(index));
        match index {
            0 => self.wood -= amount,
            1 => self.stone -= amount,
            2 => self.food -= amount,
            _ => {}
        }
        amount
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    #[serde(default = "default_character_speed")]
    pub speed: i32,
    pub alive: bool,
}

impl Character {
    pub fn starter_roster() -> Vec<Character> {
        vec![Character::new("Lian", 20, 5, 0, 10)]
    }

    fn new(name: &str, hp: i32, attack: i32, defense: i32, speed: i32) -> Character {
        Character {
            name: name.to_string(),
            hp,
            max_hp: hp,
            attack,
            defense,
            speed,
            alive: true,
        }
    }
}

fn default_character_speed() -> i32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub kind: BuildingKind,
    pub x: i32,
    pub y: i32,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingKind {
    Campfire,
    Storehouse,
    Workbench,
    ResearchTable,
    TrainingDummy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub id: String,
    pub name: String,
    pub unlocked: bool,
}

impl Technology {
    pub fn starter_tree() -> Vec<Technology> {
        vec![Technology {
            id: "base_expansion_1".to_string(),
            name: "Base Expansion I".to_string(),
            unlocked: false,
        }]
    }
}
