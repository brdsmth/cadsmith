# CADSMITH

**CADSMITH** is a learning project for building a minimal, open-source, Rust-based CAD tool from scratch to see what these LLMs can do. 

## Goals

- Learn stuff about Rust, WGPU, and GUI management
- Experiment with basic CAD features like drawing lines and shapes
- Build a clean, modular codebase for geometric modeling and visualization

## Features (WIP)

- [x] Click-to-draw lines with preview
- [ ] Basic shape tools (e.g. rectangle, circle)
- [ ] Snap/grid support
- [ ] Geometry editing
- [ ] Export formats (SVG, DXF, etc.)

## Tech Stack

- Rust
- [`egui`](https://github.com/emilk/egui) for GUI
- [`wgpu`](https://github.com/gfx-rs/wgpu) for rendering

## Getting Started

```sh
git clone https://github.com/brdsmth/cadsmith.git
cd cadsmith
cargo run