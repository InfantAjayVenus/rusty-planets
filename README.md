# Rusty Planets

`rusty-planets` is a small Rust + `macroquad` N-body gravity simulation.
It renders a simple solar-system-style scene with a fixed sun and moving planets, including trails and live speed labels.

## Features

- Real-time 2D simulation using `macroquad`
- N-body gravity (`every body attracts every other body`)
- Fixed central sun with moving planets
- Orbit trails (last 120 positions per moving body)
- On-screen labels with each planet's current speed
- Keyboard speed controls for simulation steps

## Controls

- `K`: Increase simulation speed (2x per press, capped at 16x)
- `J`: Decrease simulation speed (halves per press, floored at 1x)

## Run Locally

### Requirements

- Rust toolchain (Cargo)

### Start

```bash
cargo run
```

This opens a `macroquad` window titled `Rusty Planets`.

## Simulation Notes

- Bodies are initialized in [`src/main.rs`](/home/ajay_v/DevSpace/learning/rusty-planets/src/main.rs).
- Gravity is computed in [`gravity`](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs) using:
  - `F = G * m1 * m2 / r^2`
  - `G = 500.0`
  - Minimum distance clamp of `100.0` to avoid extreme forces at very small distances.
- Radius is derived from mass: `radius = sqrt(mass) * 2.0`.
- Position updates are skipped for fixed bodies (the sun).

## Project Structure

- [`src/main.rs`](/home/ajay_v/DevSpace/learning/rusty-planets/src/main.rs): app loop, input handling, force accumulation, update/draw loop
- [`src/planet.rs`](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs): `Planet` type, draw/update logic, trail handling, gravity function
- [`docs/curriculum.md`](/home/ajay_v/DevSpace/learning/rusty-planets/docs/curriculum.md): learning roadmap/checklist for project milestones
