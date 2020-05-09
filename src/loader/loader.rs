use crate::loader::state::LoadingState;
use crate::physics::gravity::MassComponent;
use crate::render::camera::TrackingComponent;
use hecs::{EntityBuilder, World};

pub struct ModelLoader {
    loaded: usize,
    models: Vec<Box<dyn ToEntityBuilder>>,
}

impl ModelLoader {
    pub fn new(models: Vec<Box<dyn ToEntityBuilder>>) -> Self {
        ModelLoader { loaded: 0, models }
    }

    pub fn update(&mut self, loading_state: &mut LoadingState, world: &mut World) {
        if self.loaded < self.models.len() {
            let e = self.models.get(self.loaded).unwrap();
            world.spawn(e.to_entity_builder().build());
            let progress = self.loaded as f64 / self.models.len() as f64;
            loading_state.set_progress(progress);
            self.loaded += 1;
        } else {
            let heaviest_body = world
                .query::<(&MassComponent,)>()
                .iter()
                .max_by(|x, y| {
                    let a = ((x.1).0).mass;
                    let b = ((y.1).0).mass;
                    a.partial_cmp(&b).unwrap()
                })
                .unwrap()
                .0;

            world
                .insert_one(heaviest_body, TrackingComponent::new())
                .unwrap();

            loading_state.set_progress(1.0);
        }
    }
}

pub trait ToEntityBuilder {
    fn to_entity_builder(&self) -> EntityBuilder;
}
