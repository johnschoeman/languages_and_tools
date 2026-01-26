# Bevy Doodles

A 3D interactive visualization built with [Bevy](https://bevyengine.org/) 0.18, featuring intersecting cubes with rotation controls.

## Features

- Two intersecting cubes with rigid-body rotation
- Interactive rotation controls via keyboard and UI buttons
- Automatic rotation mode with toggle
- Real-time lighting and shadows
- Screenshot capture functionality

## Screenshots

![Bevy Doodles](tmp/bevy_screenshot_auto.png)

## Controls

### Keyboard
- **W/↑** - Rotate up (X-axis)
- **S/↓** - Rotate down (X-axis)
- **A/←** - Rotate left (Y-axis)
- **D/→** - Rotate right (Y-axis)
- **Space** - Toggle automatic rotation
- **R** - Reset rotation to 0°
- **F12** - Take screenshot

### UI Buttons
All keyboard controls are also available as clickable buttons in the left panel.

## Building and Running

### Prerequisites

This project uses Nix flakes for reproducible development environments.

### Run the App

```bash
# Enter nix development environment and run
nix develop -c cargo run
```

Or enter the development environment first:

```bash
# Enter environment
nix develop

# Run the app
cargo run
```

### Take a Screenshot

Use the automated screenshot script:

```bash
./screenshot.sh
```

This builds, runs, captures a screenshot to `./tmp/bevy_screenshot_auto.png`, and exits automatically.

## Project Structure

```
bevy_doodles/
├── src/
│   └── main.rs          # Main application code
├── tmp/                 # Screenshots (auto-created)
├── flake.nix            # Nix development environment
├── screenshot.sh        # Automated screenshot script
├── CLAUDE.md            # Claude Code instructions
└── README.md            # This file
```

## Technical Details

- **Engine**: Bevy 0.18
- **Language**: Rust (Edition 2024)
- **Renderer**: Vulkan (via wgpu)
- **Platform**: Linux (NixOS)

### Scene Setup

- Central cube (beige) acts as parent entity
- Second cube (blue-gray) is a child, rotated 45° on X and Y axes
- Rotation happens around the central cube's center
- Point light with shadows at position (4, 8, 4)
- Camera positioned at (-2.5, 4.5, 9.0) looking at origin

## Development

See [CLAUDE.md](CLAUDE.md) for detailed development instructions and Claude Code configuration.

## License

This is a personal doodle project.
