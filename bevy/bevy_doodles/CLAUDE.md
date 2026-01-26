# Claude Instructions for Bevy Doodles Project

## Bevy Version

This project uses **Bevy 0.18** (released January 2026).

## Documentation Resources

When working on this project, always reference the current Bevy 0.18 documentation:

- **Official Getting Started**: https://bevy.org/learn/quick-start/getting-started/
- **API Documentation**: https://docs.rs/bevy/0.18.0/
- **Release Notes**: https://bevy.org/news/bevy-0-18/
- **Unofficial Bevy Cheat Book**: https://bevy-cheatbook.github.io/

## Important Notes

- Always check the Bevy 0.18 documentation when implementing features or suggesting code patterns
- Bevy 0.18 introduced cargo feature collections for scenario-driven features (2D, 3D, UI)
- The project uses the standard `DefaultPlugins` setup
- When searching for Bevy documentation, include "Bevy 0.18" and the current year (2026) in queries

## Coding Guidelines

- Use idiomatic Rust whenever appropriate
- Keep documentation concise and avoid being overly verbose

## Development Environment

This project uses Nix flakes for reproducible development environments. The flake.nix includes all necessary Linux dependencies for Bevy:

- System libraries: udev, alsa-lib, vulkan-loader, X11, Wayland
- Build tools: pkg-config, cmake
- Rust toolchain via rust-overlay

### Usage

Enter the development environment:
```bash
nix develop
```

Or run commands directly:
```bash
nix develop -c cargo run
```

## Verifying Visual Changes

When making visual changes to the Bevy app, always create a screenshot to verify the output is correct. Claude can view and analyze screenshots.

### Automated Screenshot

Use the automated screenshot script to capture the current state:
```bash
./screenshot.sh
```

This will:
- Build and run the app
- Wait for the scene to render
- Take a screenshot to `./tmp/bevy_screenshot_auto.png`
- Exit automatically

After running the script, use the Read tool to view the screenshot and verify visual changes.

### Manual Screenshot

During normal app operation, press **F12** to save a screenshot to `./tmp/bevy_screenshot_N.png` (where N is an incrementing counter).

## Project Structure

- Basic Bevy app setup in `src/main.rs`
- Dependencies managed in `Cargo.toml`
- Development environment in `flake.nix`
