# Rusty Planets

`rusty-planets` is a small Rust learning project built with `macroquad`. It renders a simple 2D gravity sandbox with a fixed sun, several orbiting bodies, and lightweight controls for pausing, resetting, and changing simulation speed.

## Overview

The app currently starts with four bodies:

- `Sun`: fixed at the center, mass `255.0`
- `Earth`: position `(150, 0)`, velocity `(0, 30)`
- `Mars`: position `(280, 0)`, velocity `(0, 1.5)`
- `Jupiter`: position `(450, 0)`, velocity `(0, 19.1)`

Each frame, the simulation:

- computes gravitational force between every pair of bodies
- updates velocity from the summed force
- updates position for non-fixed bodies
- draws the bodies, motion trails, and a label with each body's name and current speed

This is intentionally a lightweight sandbox, not a physically accurate orbital simulator.

## Run

Requirements:

- Rust toolchain
- `cargo`

Start the app with:

```bash
cargo run
```

This opens a `macroquad` window titled `Rusty Planets`.

## Controls

- `Space`: toggle pause and play
- `R`: reset to the initial layout and pause
- `K`: double simulation speed, up to `16x`
- `J`: halve simulation speed, down to `1x`

The app starts paused. Internally, the speed value controls how many physics steps run per rendered frame:

- `0`: paused
- `1..16`: active simulation speed

`J` and `K` only affect the simulation while it is running.

## Physics Notes

Gravity is implemented in [src/planet.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs).

- Force model: `F = G * m1 * m2 / r^2`
- Gravitational constant: `500.0`
- Distance clamp: `100.0` minimum, to avoid extreme forces at very short range
- Planet radius: `sqrt(mass) * 2.0`
- Trail length: last `120` recorded positions

The sun is marked as fixed, so it still exerts gravity but does not move.

## Project Layout

- [src/main.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/main.rs): window loop, controls, initial bodies, force accumulation, simulation update
- [src/planet.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs): `Planet` struct, drawing, labels, trails, gravity helper
- [docs/curriculum.md](/home/ajay_v/DevSpace/learning/rusty-planets/docs/curriculum.md): step-by-step learning roadmap for the project

## Current Limitations

- No collision handling yet
- Parameters are tuned for visual behavior, not realism
- The curriculum still lists collision support as unfinished in [docs/curriculum.md](/home/ajay_v/DevSpace/learning/rusty-planets/docs/curriculum.md)
