use crate::gl::ScreenResolution;
use crate::{
    gl, text, world, EventLoop, EventSettings, Events, LoadingStage, OpenGL, Simulation,
    SimulationStage, WindowSettings,
};
use assets_manager::AssetCache;
use glutin_window::GlutinWindow as Window;
use winit::window::Fullscreen;

pub fn run(simulation_file: &str, assets_path: &str, resolution: ScreenResolution) {
    let assets_cache = AssetCache::new(assets_path).unwrap();
    let asset_lock = assets_cache.load::<Simulation>(simulation_file).unwrap();
    let simulation = asset_lock.read();

    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", resolution.resolution())
        .graphics_api(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // workraound for the borderless fullscreen bug in pistoncore-glutin_window
    // see details here: https://github.com/PistonDevelopers/glutin_window/issues/210
    if resolution.fullscreen() {
        window.window.set_fullscreen(Some(Fullscreen::Borderless(window.window.current_monitor())));
    }

    let gl = gl::create(opengl);
    let glyphs = text::create();
    let world = world::create();
    let events = Events::new(EventSettings::new());
    let mut event_loop = EventLoop::new(events);

    let mut loading_stage = LoadingStage::new(
        gl.clone(),
        glyphs.clone(),
        world.clone(),
        simulation.models(),
    );
    let mut simulation_stage = SimulationStage::new(gl, glyphs.clone(), world);

    event_loop.activate_stage(&mut loading_stage, &mut window);
    event_loop.activate_stage(&mut simulation_stage, &mut window);
}
