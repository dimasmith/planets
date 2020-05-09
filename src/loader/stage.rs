use crate::core::events::EventHandler;
use crate::core::gl::SharedGraphics;
use crate::core::text::SharedGlyphCache;
use crate::core::world::SharedWorld;
use crate::loader::loader::{ModelLoader, ToEntityBuilder};
use crate::loader::screen::LoadingScreen;
use crate::loader::state::LoadingState;
use piston::input::{Event, RenderEvent, UpdateEvent};

pub struct LoadingStage<'a> {
    world: SharedWorld,
    screen: LoadingScreen<'a>,
    loader: ModelLoader,
    state: LoadingState,
}

impl<'a> LoadingStage<'a> {
    pub fn new(
        gl: SharedGraphics,
        glyphs: SharedGlyphCache<'a>,
        world: SharedWorld,
        models: Vec<Box<dyn ToEntityBuilder>>,
    ) -> Self {
        let screen = LoadingScreen::new(gl.clone(), glyphs.clone());
        let loader = ModelLoader::new(models);
        let state = LoadingState::new();
        LoadingStage {
            world,
            screen,
            loader,
            state,
        }
    }
}

impl<'a> EventHandler for LoadingStage<'a> {
    fn handle_event(&mut self, e: Event) -> bool {
        let state = &mut self.state;
        let loader = &mut self.loader;
        let screen = &mut self.screen;
        let world = &mut (*self.world).borrow_mut();

        if let Some(args) = e.render_args() {
            screen.render(state, args);
        }
        if let Some(_) = e.update_args() {
            loader.update(state, world);
        }

        if self.state.done() {
            return true;
        }
        return false;
    }
}
