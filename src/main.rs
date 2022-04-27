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
    simulator::run("simulation", "assets", ScreenResolution::default());
}
