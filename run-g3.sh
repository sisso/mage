#!/usr/bin/env bash
set -euo pipefail
cargo build -p g3rust
godot3 g3/project.godot &
cargo watch -x 'build -p g3rust'
