use std::fs;
use std::path::Path;

use crate::game::Game;

const SAVE_PATH: &str = "saves/manual.json";

pub fn save_game(game: &Game) -> anyhow::Result<()> {
    fs::create_dir_all("saves")?;
    let serialized = serde_json::to_string_pretty(game)?;
    fs::write(SAVE_PATH, serialized)?;
    Ok(())
}

pub fn load_game() -> anyhow::Result<Game> {
    let raw = fs::read_to_string(Path::new(SAVE_PATH))?;
    let mut game: Game = serde_json::from_str(&raw)?;
    game.refresh_discovered_resources();
    Ok(game)
}

pub fn save_exists() -> bool {
    Path::new(SAVE_PATH).exists()
}
