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
    pub value: i32,
}

impl Card {
    pub fn starter_deck() -> Vec<Card> {
        vec![
            Card::new("move_step", "Step", CardKind::Move, 1, 1),
            Card::new("move_step", "Step", CardKind::Move, 1, 1),
            Card::new("gather_scrap", "Gather", CardKind::Gather, 1, 2),
            Card::new("gather_scrap", "Gather", CardKind::Gather, 1, 2),
            Card::new("strike", "Strike", CardKind::Attack, 1, 4),
            Card::new("strike", "Strike", CardKind::Attack, 1, 4),
            Card::new("guard", "Guard", CardKind::Defend, 1, 4),
            Card::new("survey", "Survey", CardKind::Search, 1, 1),
        ]
    }

    fn new(id: &str, name: &str, kind: CardKind, cost: u8, value: i32) -> Card {
        Card {
            id: id.to_string(),
            name: name.to_string(),
            kind,
            cost,
            value,
        }
    }
}
