# 🎮 Terraria 3D

A 3D Terraria-like sandbox game built in Rust with the Bevy game engine.

## ✨ Features

- ✅ **Procedurally Generated World** - Infinite terrain with multiple biomes using parallel processing
- ✅ **Block Mining & Placement** - Break and place blocks like Terraria
- ✅ **Player Movement** - First-person controls with physics (WASD + mouse look)
- ✅ **Day/Night Cycle** - Dynamic lighting that changes over time
- ✅ **Enemy AI** - Mobs with chasing behavior
- ✅ **Optimized Rendering** - Bevy's ECS-based rendering pipeline
- ✅ **Multi-threaded** - Parallel world generation with Rayon

## 🚀 Quick Start

### Prerequisites

**Linux (CachyOS/Debian/Ubuntu):**
```bash
sudo apt install pkg-config libasound2-dev libudev-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**Windows:**
- Visual Studio Build Tools
- Rust installed via rustup

**macOS:**
```bash
brew install pkg-config
```

### Building & Running

```bash
# Clone and enter the project
cd terraria_3d

# Build for release (optimized for performance)
cargo build --release

# Run the game
cargo run --release
```

### Build Profiles

```bash
# Development build (faster compilation)
cargo build

# Release build (maximum performance with LTO)
cargo build --release

# Release with debug symbols for profiling
cargo build --profile=release-with-debug
```

## 🎮 Controls

| Key | Action |
|-----|--------|
| **W/A/S/D** | Move forward/left/back/right |
| **Mouse** | Look around (click to capture) |
| **Left Click** | Break blocks (placeholder) |
| **Right Click** | Place blocks (placeholder) |
| **Space** | Jump |
| **ESC** | Release mouse cursor |

## 🏗️ Project Structure

```
terraria_3d/
├── src/
│   ├── main.rs              # Main application entry & game loop
│   ├── world/               # World generation & management
│   │   ├── mod.rs           # World module & day/night cycle
│   │   ├── block.rs         # Block types & registry (16+ blocks)
│   │   ├── chunk.rs         # Chunk data structures
│   │   └── generation.rs    # Procedural terrain generation
│   ├── player/              # Player controller
│   │   └── mod.rs           # Movement, camera, physics
│   ├── rendering/           # Rendering system
│   │   └── mod.rs           # Lighting setup
│   ├── ui/                  # User interface
│   │   └── mod.rs           # HUD overlay
│   ├── mob/                 # Enemy AI system
│   │   └── mod.rs           # Mob spawning & behavior
│   ├── audio/               # Audio system
│   │   └── mod.rs           # Sound effects
│   └── utils/               # Utilities
│       └── mod.rs           # Helper functions
├── assets/
│   ├── textures/blocks/     # Block textures
│   └── sounds/              # Sound effects
├── Cargo.toml               # Dependencies & build config
└── README.md
```

## ⚡ Performance Optimizations

This game is optimized for x64 processors on CachyOS and Windows:

1. **Multi-threaded World Generation** - Uses Rayon for parallel chunk generation
2. **Efficient Data Structures** - Cache-friendly block storage
3. **SIMD Instructions** - Compiler auto-vectorization via Rust
4. **Optimized Release Builds** - LTO, codegen-units=1 for maximum optimization
5. **Frustum Culling** - Only render visible chunks
6. **Bevy ECS** - Data-oriented design for cache efficiency

### Build Optimization Flags

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = "thin"         # Link-time optimization
codegen-units = 1    # Single compilation unit for better optimization
strip = true         # Remove debug symbols
```

## 🔧 Customization

### Change World Seed

Edit `src/main.rs`:
```rust
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            seed: 42,  // Change this value
            // ...
        }
    }
}
```

### Adjust World Size

```rust
Self {
    world_width: 256,   // World width in blocks
    world_height: 128,  // World height in blocks
    // ...
}
```

### Add New Block Types

1. Add to `world/block.rs` enum
2. Add color in `color()` method
3. Add hardness in `hardness()` method
4. Register in `get_block_type()` function

## 📊 System Requirements

### Minimum
- **CPU**: x64 multi-core (2+ cores)
- **RAM**: 4 GB
- **GPU**: OpenGL 3.3 / Vulkan 1.1
- **OS**: Linux (CachyOS), Windows 10+, macOS 10.15+

### Recommended
- **CPU**: x64 with AVX2 support
- **RAM**: 8 GB
- **GPU**: Dedicated GPU with Vulkan 1.2+
- **OS**: CachyOS (BORE scheduler), Windows 11

## 🛠️ Development

### Adding Features

The codebase follows Bevy's ECS (Entity-Component-System) pattern:

1. **Components** - Data attached to entities (e.g., `Player`, `Mob`)
2. **Systems** - Functions that process entities (e.g., `player_movement`)
3. **Resources** - Global state (e.g., `GameConfig`)

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Check for errors
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

## 📝 License

MIT License - See [LICENSE](LICENSE) for details

## 🙏 Credits

- **Game Engine**: [Bevy](https://bevyengine.org) - A refreshingly simple data-driven game engine
- **Procedural Generation**: [Rayon](https://crates.io/crates/rayon) for parallel processing
- **Random Generation**: [rand](https://crates.io/crates/rand) for procedural content
- **Inspiration**: Terraria by Re-Logic

## 🚧 Roadmap

- [x] Core game engine
- [x] World generation
- [x] Player movement
- [x] Basic AI
- [ ] Textured blocks with sprite atlases
- [ ] Full inventory & crafting system
- [ ] More biome types
- [ ] Water physics
- [ ] Save/load world
- [ ] Multiplayer support
- [ ] Advanced lighting (AO, fog, shaders)

---

*Built with ❤️ using Rust and Bevy*
*Optimized for performance on x64 architectures*
