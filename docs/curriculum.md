# Rusty Planets Curriculum

## Tech Stack
- **Language**: Rust (edition 2024)
- **Rendering**: macroquad (2D game library)
- **Physics**: N-body gravitational simulation (every planet attracts every other)

## Module 1: First Window
Goal: Open a window and confirm macroquad works.
- [x] Step 1.1: Add macroquad dependency to Cargo.toml
- [x] Step 1.2: Replace main.rs with a macroquad game loop that opens a black window
- [x] Step 1.3: Draw a yellow circle (the sun) at the center of the screen

## Module 2: The Planet Struct
Goal: Define data for a single planet and draw it.
- [x] Step 2.1: Create a `Planet` struct with position, velocity, mass, and color
- [x] Step 2.2: Instantiate one planet and draw it on screen
- [x] Step 2.3: Give the planet a starting velocity so it moves each frame
- [x] Step 2.4: Checkpoint — understanding structs and the game loop

## Module 3: Gravity
Goal: Make the planet orbit the sun using gravitational force.
- [x] Step 3.1: Implement a `gravitational_force` function (Newton's law)
- [x] Step 3.2: Apply the force to update the planet's velocity each frame
- [x] Step 3.3: Watch a stable orbit form with the right initial conditions
- [x] Step 3.4: Checkpoint — understanding vectors and force application

## Module 4: N-Body Simulation
Goal: Add multiple planets that all attract each other.
- [x] Step 4.1: Store planets in a `Vec<Planet>`
- [x] Step 4.2: Calculate forces between every pair of planets
- [x] Step 4.3: Add the sun as a special heavy body in the planet list
- [x] Step 4.4: Tune masses and initial velocities for stable-ish orbits
- [x] Step 4.5: Checkpoint — understanding Vec iteration and Rust borrow rules

## Module 5: Polish
Goal: Make the simulation beautiful and informative.
- [x] Step 5.1: Add orbit trails (store last N positions per planet)
- [x] Step 5.2: Display planet names and stats with macroquad's text API
- [x] Step 5.3: Add keyboard controls (speed up/slow down, reset)
- [ ] Step 5.4: Handle planet collisions (merge them)
