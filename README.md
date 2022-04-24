# Planets

A simple visualizer of planetary motion based on Newtonean mechanics.

Main intent of the project is to get some familiarity with Rust.

## Running

You can describe a simulation using [.ron](https://github.com/ron-rs/ron) files with the parameters.
See assets directory for examples.

Place compiled planets binary next to the `assets` directory and start it.

## Simulation controls

* `,` - slows simulation down;
* `.` - speeds simulation up;
* `p` - pauses/resumes simulation;
* `mouse wheel` - zooms in/out;
* `esc` - exit simulation;

## Internals

The project uses [piston](https://www.piston.rs/) engine for rendering.
World is built with [hecs](https://github.com/Ralith/hecs) - nice implementation of ECS pattern.

## Issues that will immediately kick you

* The resolution is hardcoded to 1920x1080 in fullscreen mode.
* Paths to assets and file names inside are hardcoded.

## Good words for

* JetBrains - for CLion and JetBrains Mono font
* Kerbal Space Program - for inspiration and a ton of fun :)

## Plans

* Fix immediate usage issues;
* Use patched conics for orbit prediction (and, possibly, for simulation as well);
* Change camera focus;
* Detect collisions;
