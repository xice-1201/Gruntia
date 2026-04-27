# Gruntia

Gruntia is a lightweight native 2D game demo about base building, survival, faction growth, random expeditions, and card-driven turn-based actions.

The project intentionally does not use a full game engine. The first framework uses Rust with a simple framebuffer window so the game can later ship as a Windows executable that players can launch directly.

## Current Prototype

Implemented foundation:

- Native window and main loop.
- Windows GUI subsystem startup without an extra console window.
- Embedded executable icon and runtime window icon.
- Main menu, base, expedition, and game-over screens.
- Fixed square base map.
- Initial base is 5x5 with storage capacity 10.
- Basic building placement.
- Base expansion technology placeholder.
- Random expedition map.
- Turn-based card hand, deck, discard, and energy.
- Expedition preparation with a 5-card carry limit and custom card selection.
- Card-driven movement and area gathering actions.
- Evacuation from expedition back to base consumes 3 food; failing to pay it ends the campaign.
- Manual save and load at `saves/manual.json`.

## Controls

- Main menu buttons are clicked with the mouse.
- Mouse wheel: zoom the base map in or out while in the base view.
- Hold right mouse button and drag: pan the base view.
- `W` / `A` / `S` / `D`: pan the base view.
- Base actions use the Chinese buttons in the right panel.
- Build opens a building menu; choose a building, then click a base tile to place it.
- Right click cancels a pending building placement.
- Storage shows resource details; hover resource names for descriptions.
- Explore opens a preparation menu; add at least one card to the carry slots before departing.
- Prepared cards consume food when departing; cards no longer spend energy during expedition play.
- During expeditions, drag a card out of the hand to play it, or drop it back into the hand to reorder.
- Movement cards cost no food and let the player choose a destination within 3 tiles.
- Gathering cards collect all forest and stone resources on the player tile and the four adjacent tiles.
- Forests spawn as large clusters plus scattered lone trees and yield 1-5 wood.
- Stone piles yield 1-2 stone, berry bushes spawn in 2-6 tile clusters and yield 1-3 food per tile.
- Enemies are sparse; ruins are rare, at most one 2x2 site per expedition map.
- Expedition spawn points are random; nearby berries and lone trees are more likely, while enemies and ruins favor distant areas.
- Stones, enemies, and ruins block movement pathing.
- Expedition maps support mouse wheel zoom, right-drag panning, and `W` / `A` / `S` / `D` camera movement.
- `L`: load.
- `Esc`: evacuate or go back outside the base view.

## Development

Install Rust and the Windows C++ build tools, then run:

- Rust toolchain: <https://www.rust-lang.org/tools/install>
- MSVC linker: install "Visual Studio Build Tools" with the "Desktop development with C++" workload.

If a normal PowerShell cannot find `link.exe`, open "Developer PowerShell for Visual Studio" or load the Visual Studio developer environment first.

```powershell
cargo run
```

From a normal PowerShell, you can also use the helper scripts:

```powershell
.\scripts\check.ps1
.\scripts\run.ps1
.\scripts\build-release.ps1
```

Create a release build:

```powershell
cargo build --release
```

The release executable will be generated under `target/release/`.

## Documentation

The current requirements document is available at:

- `docs/REQUIREMENTS.md`
