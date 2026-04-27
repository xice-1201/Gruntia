use std::path::PathBuf;
use std::time::Instant;

#[cfg(target_os = "windows")]
use minifb::Icon;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
#[cfg(target_os = "windows")]
use std::str::FromStr;

use crate::card::Card;
use crate::data::BuildingKind;
use crate::game::{building_cost, Game, Screen};
use crate::render::{
    Renderer, BASE_VIEWPORT_HEIGHT, BASE_VIEWPORT_WIDTH, EXPEDITION_HAND_Y,
    EXPEDITION_VIEWPORT_HEIGHT, HEIGHT, WIDTH,
};
use crate::save;

const TARGET_FPS: usize = 60;

pub fn run() -> anyhow::Result<()> {
    let mut window = Window::new(
        "Gruntia",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )?;

    set_window_icon(&mut window);
    disable_ime(&window);

    window.set_target_fps(TARGET_FPS);

    let mut renderer = Renderer::new();
    let mut game = Game::new();
    let mut last_tick = Instant::now();
    let mut input = InputState::default();

    while window.is_open() && !game.should_quit {
        input.update(&window);
        handle_input(&window, &mut input, &mut game)?;

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        last_tick = now;

        game.update(dt);
        renderer.draw(
            &game,
            input.mouse_x,
            input.mouse_y,
            input.dragged_expedition_card,
        );
        window.update_with_buffer(renderer.buffer(), WIDTH, HEIGHT)?;
        window.set_title(&game.window_title());
    }

    Ok(())
}

#[derive(Default)]
struct InputState {
    mouse_x: usize,
    mouse_y: usize,
    mouse_delta_x: f32,
    mouse_delta_y: f32,
    scroll_y: f32,
    mouse_seen: bool,
    left_down: bool,
    left_pressed: bool,
    left_released: bool,
    right_down: bool,
    right_pressed: bool,
    dragged_expedition_card: Option<usize>,
}

impl InputState {
    fn update(&mut self, window: &Window) {
        let previous_left = self.left_down;
        let previous_right = self.right_down;
        self.left_down = window.get_mouse_down(MouseButton::Left);
        self.left_pressed = self.left_down && !previous_left;
        self.left_released = !self.left_down && previous_left;
        self.right_down = window.get_mouse_down(MouseButton::Right);
        self.right_pressed = self.right_down && !previous_right;
        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
            if self.mouse_seen {
                self.mouse_delta_x = x - self.mouse_x as f32;
                self.mouse_delta_y = y - self.mouse_y as f32;
            } else {
                self.mouse_seen = true;
            }
            self.mouse_x = x.max(0.0) as usize;
            self.mouse_y = y.max(0.0) as usize;
        }

        self.scroll_y = window
            .get_scroll_wheel()
            .map(|(_, y)| y)
            .unwrap_or_default();
    }
}

#[cfg(target_os = "windows")]
fn set_window_icon(window: &mut Window) {
    for path in icon_candidates() {
        if path.exists() {
            if let Some(path) = path.to_str() {
                if let Ok(icon) = Icon::from_str(path) {
                    window.set_icon(icon);
                    return;
                }
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn set_window_icon(_window: &mut Window) {}

#[cfg(target_os = "windows")]
fn disable_ime(window: &Window) {
    use std::ffi::c_void;

    #[link(name = "imm32")]
    unsafe extern "system" {
        fn ImmAssociateContext(hwnd: *mut c_void, himc: *mut c_void) -> *mut c_void;
    }

    unsafe {
        ImmAssociateContext(window.get_window_handle(), std::ptr::null_mut());
    }
}

#[cfg(not(target_os = "windows"))]
fn disable_ime(_window: &Window) {}

#[cfg(target_os = "windows")]
fn icon_candidates() -> Vec<PathBuf> {
    let mut candidates = vec![PathBuf::from("assets/icons/gruntia.ico")];

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join("assets/icons/gruntia.ico"));
            candidates.push(exe_dir.join("../../assets/icons/gruntia.ico"));
        }
    }

    candidates
        .into_iter()
        .map(normalize_path)
        .collect::<Vec<PathBuf>>()
}

#[cfg(target_os = "windows")]
fn normalize_path(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap_or(path)
}

fn handle_input(window: &Window, input: &mut InputState, game: &mut Game) -> anyhow::Result<()> {
    match game.screen {
        Screen::MainMenu => handle_menu_input(input, game),
        Screen::TransitionToBase => {}
        Screen::TransitionToExpedition => {}
        Screen::Base => {
            handle_base_input(window, input, game)?;
        }
        Screen::Expedition => {
            handle_expedition_input(window, input, game)?;
        }
        Screen::GameOver => handle_game_over_input(input, game),
    }

    Ok(())
}

fn handle_game_over_input(input: &InputState, game: &mut Game) {
    if !input.left_pressed {
        return;
    }

    let button_y = HEIGHT / 2 - 150 + 178;
    if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        WIDTH / 2 - 222,
        button_y,
        132,
        44,
    ) {
        if save::save_exists() {
            match save::load_game() {
                Ok(mut loaded) => {
                    loaded.should_quit = false;
                    *game = loaded;
                    game.set_message("Loaded game.");
                }
                Err(err) => game.set_message(format!("Load failed: {err}")),
            }
        }
    } else if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        WIDTH / 2 - 66,
        button_y,
        132,
        44,
    ) {
        game.new_campaign();
    } else if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        WIDTH / 2 + 90,
        button_y,
        132,
        44,
    ) {
        game.screen = Screen::MainMenu;
        game.set_message("Select a menu option.");
    }
}

fn handle_menu_input(input: &InputState, game: &mut Game) {
    if !input.left_pressed {
        return;
    }

    let panel_x = WIDTH / 2 - 230;
    let panel_y = HEIGHT / 2 - 190;
    if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        panel_x + 90,
        panel_y + 154,
        280,
        54,
    ) {
        game.begin_new_campaign_transition();
    } else if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        panel_x + 90,
        panel_y + 224,
        280,
        54,
    ) {
        match save::load_game() {
            Ok(mut loaded) => {
                loaded.should_quit = false;
                *game = loaded;
                game.set_message("Loaded game.");
            }
            Err(err) => game.set_message(format!("Load failed: {err}")),
        }
    } else if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        panel_x + 90,
        panel_y + 294,
        280,
        54,
    ) {
        game.should_quit = true;
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

fn handle_base_input(window: &Window, input: &InputState, game: &mut Game) -> anyhow::Result<()> {
    if game.show_storage_dialog {
        handle_storage_input(window, input, game);
        return Ok(());
    }

    if game.show_character_dialog {
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 240;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 150;
        if input.left_pressed
            && point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 188,
                panel_y + 246,
                104,
                40,
            )
        {
            game.hide_character_dialog();
        }
        return Ok(());
    }

    if game.show_crafting_dialog {
        let panel_x = BASE_VIEWPORT_WIDTH / 2 - 240;
        let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 130;
        if input.left_pressed
            && point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 188,
                panel_y + 206,
                104,
                40,
            )
        {
            game.hide_crafting_dialog();
        }
        return Ok(());
    }

    if game.show_build_menu {
        if input.right_pressed {
            game.close_build_menu();
            return Ok(());
        }

        if input.left_pressed {
            let panel_x = BASE_VIEWPORT_WIDTH / 2 - 250;
            let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 160;
            if point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 40,
                panel_y + 74,
                124,
                38,
            ) {
                game.set_build_menu_category(0);
            } else if point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 40,
                panel_y + 120,
                124,
                38,
            ) {
                game.set_build_menu_category(1);
            } else if point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 260,
                panel_y + 90,
                180,
                40,
            ) {
                let kind = if game.build_menu_category == 0 {
                    BuildingKind::Campfire
                } else {
                    BuildingKind::Workbench
                };
                select_building_if_affordable(game, kind);
            } else if point_in_rect(
                input.mouse_x,
                input.mouse_y,
                panel_x + 260,
                panel_y + 190,
                180,
                40,
            ) {
                game.close_build_menu();
            }
        }
        return Ok(());
    }

    if game.show_expedition_prep {
        if input.right_pressed {
            game.close_expedition_prep();
            return Ok(());
        }

        if input.left_pressed {
            if let Some(index) = expedition_card_index_at(input.mouse_x, input.mouse_y) {
                if let Some(card) = Card::available_cards().get(index).cloned() {
                    game.add_prep_card(card);
                }
            } else if let Some(index) = expedition_slot_index_at(input.mouse_x, input.mouse_y) {
                game.remove_prep_card(index);
            } else {
                let panel_x = BASE_VIEWPORT_WIDTH / 2 - 316;
                let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 212;
                if point_in_rect(
                    input.mouse_x,
                    input.mouse_y,
                    panel_x + 516,
                    panel_y + 372,
                    100,
                    40,
                ) {
                    game.start_expedition(game.prep_deck.clone());
                } else if point_in_rect(
                    input.mouse_x,
                    input.mouse_y,
                    panel_x + 396,
                    panel_y + 372,
                    100,
                    40,
                ) {
                    game.close_expedition_prep();
                }
            }
        }
        return Ok(());
    }

    if input.scroll_y > 0.0 {
        game.adjust_base_zoom(1);
    } else if input.scroll_y < 0.0 {
        game.adjust_base_zoom(-1);
    }

    let mut pan_x = 0.0;
    let mut pan_y = 0.0;
    const KEYBOARD_PAN_SPEED: f32 = 10.0;

    if input.right_down {
        pan_x -= input.mouse_delta_x;
        pan_y -= input.mouse_delta_y;
    }

    if window.is_key_down(Key::A) {
        pan_x -= KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::D) {
        pan_x += KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::W) {
        pan_y -= KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::S) {
        pan_y += KEYBOARD_PAN_SPEED;
    }

    if pan_x != 0.0 || pan_y != 0.0 {
        game.pan_base_camera(pan_x, pan_y, BASE_VIEWPORT_WIDTH, BASE_VIEWPORT_HEIGHT);
    }

    if input.left_pressed
        && input.mouse_x < BASE_VIEWPORT_WIDTH
        && input.mouse_y < BASE_VIEWPORT_HEIGHT
    {
        if game.selected_building.is_some() {
            if let Some((x, y)) = screen_to_base_tile(game, input.mouse_x, input.mouse_y) {
                game.place_selected_building(x, y);
            } else {
                game.cancel_building_placement();
            }
            return Ok(());
        }
    }

    if input.right_pressed && game.selected_building.is_some() {
        game.cancel_building_placement();
        return Ok(());
    }

    let panel_x = BASE_VIEWPORT_WIDTH + 28;
    if game.confirm_quit && input.left_pressed {
        if point_in_rect(input.mouse_x, input.mouse_y, panel_x, 612, 92, 36) {
            game.back_or_quit();
        } else {
            game.cancel_quit_confirmation();
        }
        return Ok(());
    }

    if input.left_pressed && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 224, 196, 38) {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.show_character_dialog();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 274, 196, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.show_storage_dialog();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 324, 196, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.show_crafting_dialog();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 374, 92, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.open_build_menu();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x + 104, 374, 92, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.unlock_first_technology();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 424, 196, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        game.open_expedition_prep();
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 474, 92, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        save::save_game(game)?;
        game.set_message("Saved game.");
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x + 104, 474, 92, 38)
    {
        game.cancel_building_if_selected();
        game.confirm_quit = false;
        if save::save_exists() {
            match save::load_game() {
                Ok(mut loaded) => {
                    loaded.should_quit = false;
                    *game = loaded;
                    game.set_message("Loaded game.");
                }
                Err(err) => game.set_message(format!("Load failed: {err}")),
            }
        }
    } else if input.left_pressed
        && point_in_rect(input.mouse_x, input.mouse_y, panel_x, 524, 196, 38)
    {
        game.cancel_building_if_selected();
        game.request_quit_confirmation();
    }

    Ok(())
}

fn handle_storage_input(window: &Window, input: &InputState, game: &mut Game) {
    if let Some(digit) = pressed_digit(window) {
        game.append_storage_discard_digit(digit);
    }
    if window.is_key_pressed(Key::Backspace, KeyRepeat::No) {
        game.backspace_storage_discard_input();
    }

    if !input.left_pressed {
        return;
    }

    let panel_x = BASE_VIEWPORT_WIDTH / 2 - 300;
    let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 190;

    if game.pending_storage_discard.is_some() {
        if point_in_rect(
            input.mouse_x,
            input.mouse_y,
            panel_x + 338,
            panel_y + 316,
            84,
            40,
        ) {
            game.confirm_storage_discard();
        } else {
            game.cancel_storage_discard();
        }
        return;
    }

    if point_in_rect(
        input.mouse_x,
        input.mouse_y,
        panel_x + 452,
        panel_y + 316,
        96,
        40,
    ) {
        game.hide_storage_dialog();
        return;
    }

    for (row, index) in game.visible_storage_indices().into_iter().enumerate() {
        let row_x = panel_x + 72;
        let row_y = panel_y + 92 + row * 56;
        let input_x = row_x + 158;
        if point_in_rect(input.mouse_x, input.mouse_y, row_x + 126, row_y + 2, 26, 32) {
            game.adjust_storage_discard_input(index, -1);
        } else if point_in_rect(input.mouse_x, input.mouse_y, input_x, row_y + 2, 46, 32) {
            game.focus_storage_input(index);
        } else if point_in_rect(
            input.mouse_x,
            input.mouse_y,
            input_x + 52,
            row_y + 2,
            26,
            32,
        ) {
            game.adjust_storage_discard_input(index, 1);
        } else if point_in_rect(input.mouse_x, input.mouse_y, input_x + 124, row_y, 88, 36) {
            game.request_storage_discard(index);
        }
    }
}

fn pressed_digit(window: &Window) -> Option<i32> {
    let keys = [
        (Key::Key0, 0),
        (Key::Key1, 1),
        (Key::Key2, 2),
        (Key::Key3, 3),
        (Key::Key4, 4),
        (Key::Key5, 5),
        (Key::Key6, 6),
        (Key::Key7, 7),
        (Key::Key8, 8),
        (Key::Key9, 9),
    ];

    keys.iter()
        .find(|(key, _)| window.is_key_pressed(*key, KeyRepeat::No))
        .map(|(_, digit)| *digit)
}

fn select_building_if_affordable(game: &mut Game, kind: BuildingKind) {
    if game.resources.can_afford(&building_cost(kind)) {
        game.select_building(kind);
    } else {
        game.set_message("Not enough resources.");
    }
}

fn expedition_card_index_at(x: usize, y: usize) -> Option<usize> {
    let panel_x = BASE_VIEWPORT_WIDTH / 2 - 316;
    let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 212;
    for index in 0..Card::available_cards().len() {
        let card_x = panel_x + 40 + (index % 3) * 126;
        let card_y = panel_y + 96 + (index / 3) * 126;
        if point_in_rect(x, y, card_x, card_y, 100, 104) {
            return Some(index);
        }
    }

    None
}

fn expedition_slot_index_at(x: usize, y: usize) -> Option<usize> {
    let panel_x = BASE_VIEWPORT_WIDTH / 2 - 316;
    let panel_y = BASE_VIEWPORT_HEIGHT / 2 - 212;
    for index in 0..5 {
        if point_in_rect(x, y, panel_x + 472, panel_y + 74 + index * 56, 120, 44) {
            return Some(index);
        }
    }

    None
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

fn handle_expedition_input(
    window: &Window,
    input: &mut InputState,
    game: &mut Game,
) -> anyhow::Result<()> {
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

    if game.show_expedition_bag {
        let panel_x = WIDTH / 2 - 180;
        if input.left_pressed
            && point_in_rect(input.mouse_x, input.mouse_y, panel_x + 128, 324, 104, 40)
        {
            game.hide_expedition_bag();
        }
        return Ok(());
    }

    if input.scroll_y > 0.0 {
        game.adjust_expedition_zoom(1, WIDTH, EXPEDITION_VIEWPORT_HEIGHT);
    } else if input.scroll_y < 0.0 {
        game.adjust_expedition_zoom(-1, WIDTH, EXPEDITION_VIEWPORT_HEIGHT);
    }

    let mut pan_x = 0.0;
    let mut pan_y = 0.0;
    const KEYBOARD_PAN_SPEED: f32 = 10.0;

    if input.right_down {
        pan_x -= input.mouse_delta_x;
        pan_y -= input.mouse_delta_y;
    }

    if window.is_key_down(Key::A) {
        pan_x -= KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::D) {
        pan_x += KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::W) {
        pan_y -= KEYBOARD_PAN_SPEED;
    }
    if window.is_key_down(Key::S) {
        pan_y += KEYBOARD_PAN_SPEED;
    }

    if pan_x != 0.0 || pan_y != 0.0 {
        game.pan_expedition_camera(pan_x, pan_y, WIDTH, EXPEDITION_VIEWPORT_HEIGHT);
    }

    if input.left_pressed {
        let button_x = WIDTH - 132;
        if point_in_rect(
            input.mouse_x,
            input.mouse_y,
            button_x,
            HEIGHT - 104,
            104,
            40,
        ) {
            game.show_expedition_bag();
            return Ok(());
        }

        if point_in_rect(input.mouse_x, input.mouse_y, button_x, HEIGHT - 58, 104, 40) {
            game.evacuate();
            return Ok(());
        }

        if let Some((tile_x, tile_y)) = expedition_map_tile_at(game, input.mouse_x, input.mouse_y) {
            game.choose_move_destination(tile_x, tile_y);
            return Ok(());
        }

        if game
            .expedition
            .as_ref()
            .and_then(|expedition| expedition.pending_move_card.as_ref())
            .is_none()
        {
            if let Some(index) = expedition_hand_card_index_at(input.mouse_x, input.mouse_y) {
                input.dragged_expedition_card = Some(index);
            }
        }
    }

    if input.left_released {
        if let Some(index) = input.dragged_expedition_card.take() {
            if input.mouse_y >= EXPEDITION_VIEWPORT_HEIGHT {
                let target = expedition_hand_drop_index(input.mouse_x);
                game.reorder_hand_card(index, target);
            } else {
                game.play_hand_card(index);
            }
        }
    }

    Ok(())
}

fn expedition_hand_card_index_at(x: usize, y: usize) -> Option<usize> {
    if y < EXPEDITION_HAND_Y || y >= EXPEDITION_HAND_Y + 84 {
        return None;
    }

    for index in 0..5 {
        let card_x = 28 + index * 108;
        if point_in_rect(x, y, card_x, EXPEDITION_HAND_Y, 92, 84) {
            return Some(index);
        }
    }

    None
}

fn expedition_hand_drop_index(x: usize) -> usize {
    if x <= 28 {
        return 0;
    }

    ((x - 28) / 108).min(4)
}

fn expedition_map_tile_at(game: &Game, x: usize, y: usize) -> Option<(i32, i32)> {
    if y >= EXPEDITION_VIEWPORT_HEIGHT {
        return None;
    }

    let Some(expedition) = &game.expedition else {
        return None;
    };
    let tile_size = expedition.tile_size() as i32;
    let (origin_x, origin_y) = expedition.origin(WIDTH, EXPEDITION_VIEWPORT_HEIGHT);
    let local_x = x as i32 - origin_x;
    let local_y = y as i32 - origin_y;
    if local_x < 0 || local_y < 0 {
        return None;
    }

    let tile_x = local_x / tile_size;
    let tile_y = local_y / tile_size;
    if tile_x >= 0 && tile_y >= 0 && tile_x < expedition.map.width && tile_y < expedition.map.height
    {
        Some((tile_x as i32, tile_y as i32))
    } else {
        None
    }
}
