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

## Cli parameters

See cli help for details
```shell
planets --help
```

### Control resolution

The simulation runs in the fullscreen mode by default. 
You can change it to windowed mode via the `--windowed` flag.

You can control the resolution using the `--resolution=800x600` flag.

Note that when running in fullscreen mode on some systems the resolution might be igored.
The OS might start simulation in a native screen resolution.

## Internals

The project uses [piston](https://www.piston.rs/) engine for rendering.
World is built with [hecs](https://github.com/Ralith/hecs) - nice implementation of ECS pattern.

## Issues that will immediately kick you

* Paths to assets and file names inside are hardcoded.

## Good words for

* JetBrains - for CLion and JetBrains Mono font
* Kerbal Space Program - for inspiration and a ton of fun :)

## Plans

* Fix immediate usage issues;
* Use patched conics for orbit prediction (and, possibly, for simulation as well);
* Change camera focus;
* Detect collisions;
