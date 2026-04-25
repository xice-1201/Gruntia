# Gruntia

Gruntia is a lightweight native 2D game demo about base building, survival, faction growth, random expeditions, and card-driven turn-based actions.

The project intentionally does not use a full game engine. The first framework uses Rust with a simple framebuffer window so the game can later ship as a Windows executable that players can launch directly.

## Current Prototype

Implemented foundation:

- Native window and main loop.
- Main menu, base, expedition, and game-over screens.
- Fixed square base map.
- Basic building placement.
- Base expansion technology placeholder.
- Random expedition map.
- Turn-based card hand, deck, discard, and energy.
- Card-driven movement, gathering, attack, defense, and search actions.
- Evacuation from expedition back to base.
- Manual save and load at `saves/manual.json`.

## Controls

- `N`: start a new campaign.
- `Enter`: confirm current screen action.
- `B`: build the next basic base structure.
- `T`: unlock the first technology when resources are available.
- `E`: start an expedition from the base.
- `Space`: play the next card in hand during expedition.
- `Tab`: end expedition turn.
- `V`: evacuate from expedition.
- `S`: save.
- `L`: load.
- `Esc`: back or evacuate.

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
