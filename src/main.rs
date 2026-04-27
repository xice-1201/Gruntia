#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod card;
mod data;
mod game;
mod render;
mod save;

fn main() -> anyhow::Result<()> {
    app::run()
}
