use std::time::Instant;

use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use crate::game::{Game, Screen};
use crate::render::{Renderer, HEIGHT, WIDTH};
use crate::save;

const TARGET_FPS: usize = 60;

pub fn run() -> anyhow::Result<()> {
    let mut window = Window::new(
        "Gruntia",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    window.set_target_fps(TARGET_FPS);

    let mut renderer = Renderer::new();
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    while window.is_open() && !game.should_quit {
        handle_input(&window, &mut game)?;

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        last_tick = now;

        game.update(dt);
        renderer.draw(&game);
        window.update_with_buffer(renderer.buffer(), WIDTH, HEIGHT)?;
        window.set_title(&game.window_title());
    }

    Ok(())
}

fn handle_input(window: &Window, game: &mut Game) -> anyhow::Result<()> {
    if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        game.back_or_quit();
    }

    if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
        game.confirm();
    }

    if window.is_key_pressed(Key::N, KeyRepeat::No) {
        game.new_campaign();
    }

    if window.is_key_pressed(Key::S, KeyRepeat::No) {
        save::save_game(game)?;
        game.set_message("Saved game.");
    }

    if window.is_key_pressed(Key::L, KeyRepeat::No) {
        match save::load_game() {
            Ok(mut loaded) => {
                loaded.should_quit = false;
                *game = loaded;
                game.set_message("Loaded game.");
            }
            Err(err) => game.set_message(format!("Load failed: {err}")),
        }
    }

    match game.screen {
        Screen::MainMenu => {}
        Screen::Base => handle_base_input(window, game),
        Screen::Expedition => handle_expedition_input(window, game),
        Screen::GameOver => {}
    }

    Ok(())
}

fn handle_base_input(window: &Window, game: &mut Game) {
    if window.is_key_pressed(Key::B, KeyRepeat::No) {
        game.build_basic_structure();
    }

    if window.is_key_pressed(Key::T, KeyRepeat::No) {
        game.unlock_first_technology();
    }

    if window.is_key_pressed(Key::E, KeyRepeat::No) {
        game.start_expedition();
    }
}

fn handle_expedition_input(window: &Window, game: &mut Game) {
    if window.is_key_pressed(Key::Space, KeyRepeat::No) {
        game.play_next_card();
    }

    if window.is_key_pressed(Key::Tab, KeyRepeat::No) {
        game.end_turn();
    }

    if window.is_key_pressed(Key::V, KeyRepeat::No) {
        game.evacuate();
    }
}
