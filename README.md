# Rusty Planets

`rusty-planets` is a small Rust + `macroquad` gravity sandbox. It draws a fixed sun and a few moving planets, then updates them with a simple N-body simulation each frame.

## What It Does

- Opens a real-time 2D `macroquad` window
- Simulates gravity between all bodies
- Keeps the sun fixed at the center of the system
- Draws orbit trails for moving planets
- Shows each planet's name and current speed
- Supports pause, reset, and simulation speed controls

## Current Scene

The default setup includes four bodies defined in [src/main.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/main.rs):

- `Sun`: fixed, mass `255.0`
- `Earth`: position `(150, 0)`, velocity `(0, 30)`
- `Mars`: position `(280, 0)`, velocity `(0, 1.5)`
- `Jupiter`: position `(450, 0)`, velocity `(0, 19.1)`

## Controls

- `Space`: toggle pause and play
- `R`: reset to the initial planet layout and pause
- `K`: double the simulation speed, up to `16x`
- `J`: halve the simulation speed, down to `1x`

The simulation starts paused. Internally, speed `0` means paused and `1..16` controls how many physics steps run per frame.

## Run

Requirements:

- Rust toolchain with `cargo`

Start the app:

```bash
cargo run
```

This opens a window titled `Rusty Planets`.

## Physics Model

Gravity is implemented in [src/planet.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs) with a simplified Newtonian force:

- `F = G * m1 * m2 / r^2`
- `G = 500.0`
- Distance is clamped to a minimum of `100.0`

Other details:

- Planet radius is derived from mass: `sqrt(mass) * 2.0`
- Velocity is updated from force each simulation step
- Fixed bodies do not move, but still exert gravity
- Trails store the last `120` positions for each moving body

## Project Layout

- [src/main.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/main.rs): simulation loop, controls, body setup, force accumulation
- [src/planet.rs](/home/ajay_v/DevSpace/learning/rusty-planets/src/planet.rs): `Planet` struct, drawing, trail logic, gravity calculation
- [docs/curriculum.md](/home/ajay_v/DevSpace/learning/rusty-planets/docs/curriculum.md): learning roadmap for the project

## Notes

- This is a lightweight learning project, not a physically accurate orbital simulator.
- There is no collision handling yet.
- The curriculum checklist in [docs/curriculum.md](/home/ajay_v/DevSpace/learning/rusty-planets/docs/curriculum.md) still shows some controls-related steps as incomplete even though the current code already includes pause, reset, and speed controls.
