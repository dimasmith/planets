use clap::{arg, ArgMatches, Command};
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use self::core::simulator;
use crate::core::events::EventLoop;
use crate::core::{gl, text, world};
use crate::gl::ScreenResolution;
use crate::loader::stage::LoadingStage;
use crate::model::Simulation;
use crate::simulation::SimulationStage;

mod core;
mod loader;
mod model;
mod physics;
mod render;
mod simulation;

fn main() {
    let cli_matches = Command::new("planets")
        .version("0.2.0")
        .about("Run gravitational simulations of multiple bodies. Uses Newtonian mechanics")
        .arg(
            arg!(-r --resolution <RESOLUTION>)
                .required(false)
                .default_value("1920x1080")
                .help("set simulation graphics resolution"),
        )
        .arg(
            arg!(-w - -windowed)
                .required(false)
                .help("run in windowed mode"),
        )
        .get_matches();

    let resolution = configure_resolution(cli_matches);

    simulator::run("simulation", "assets", resolution);
}

fn configure_resolution(cli_matches: ArgMatches) -> ScreenResolution {
    let mut resolution = ScreenResolution::default();
    if cli_matches.is_present("windowed") {
        resolution.set_fullscreen(false);
    }
    if let Some(res_str) = cli_matches.value_of("resolution") {
        resolution.resolution_from_str(res_str);
    }
    resolution
}
