use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardKind {
    Move,
    Gather,
    Search,
    Attack,
    Defend,
    Skill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub kind: CardKind,
    pub cost: u8,
    #[serde(default)]
    pub food_cost: u8,
    pub value: i32,
}

impl Card {
    pub fn available_cards() -> Vec<Card> {
        vec![Card::move_step(), Card::gather_scrap()]
    }

    pub fn move_step() -> Card {
        Card::new("move_step", "\u{79fb}\u{52a8}", CardKind::Move, 0, 3)
    }

    pub fn gather_scrap() -> Card {
        Card::new("gather_scrap", "\u{91c7}\u{96c6}", CardKind::Gather, 1, 2)
    }

    fn new(id: &str, name: &str, kind: CardKind, food_cost: u8, value: i32) -> Card {
        Card {
            id: id.to_string(),
            name: name.to_string(),
            kind,
            cost: 0,
            food_cost,
            value,
        }
    }
}
