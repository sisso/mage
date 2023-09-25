#!/usr/bin/env bash
set -euo pipefail
cargo build -p g4rust
godot4 g4/project.godot
