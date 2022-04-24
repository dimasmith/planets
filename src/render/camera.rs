use crate::physics::motion::{Motion, Position};
use crate::render::render_box::RenderBoxComponent;
use graphics::{Context, Transformed};
use hecs::{Entity, World};
use interpolation;
use piston::input::RenderArgs;
use std::collections::HashMap;

pub enum TrackingMode {
    Fixed,
    Tracking(Entity),
}

pub struct TrackingComponent {}

impl Default for TrackingComponent {
    fn default() -> Self {
        TrackingComponent::new()
    }
}

impl TrackingComponent {
    pub fn new() -> Self {
        TrackingComponent {}
    }
}

pub struct Camera {
    zoom: Zoom,
    zoom_step: f64,
    pub focus: Position,
    pub tracking: TrackingMode,
}

impl Camera {
    #[allow(dead_code)]
    pub fn tracking(zoom: f64, entity: Entity) -> Self {
        Camera {
            zoom: Zoom::new(zoom),
            zoom_step: zoom / 16.0,
            focus: [0.0, 0.0],
            tracking: TrackingMode::Tracking(entity),
        }
    }

    pub fn fixed(zoom: f64) -> Self {
        Camera {
            zoom: Zoom::new(zoom),
            zoom_step: zoom / 16.0,
            focus: [0.0, 0.0],
            tracking: TrackingMode::Fixed,
        }
    }

    pub fn project(&self, coords: Position) -> Position {
        vecmath::vec2_scale(coords, self.zoom.zoom)
    }

    pub fn zoom_in(&mut self) {
        self.zoom.change_zoom_relative(self.zoom_step, 16);
    }

    pub fn zoom_out(&mut self) {
        self.zoom.change_zoom_relative(-self.zoom_step, 16);
    }

    fn update_zoom(&mut self) {
        self.zoom.update();
    }

    fn track(&mut self, entity: Entity) {
        self.tracking = TrackingMode::Tracking(entity);
    }
}

pub struct CameraSystem {
    pub camera: Camera,
}

impl CameraSystem {
    pub fn new(camera: Camera) -> CameraSystem {
        CameraSystem { camera }
    }

    pub fn update(&mut self, context: Context, world: &mut World, args: RenderArgs) -> Context {
        self.camera.update_zoom();
        let mut projected_positions_by_entity: HashMap<Entity, Position> = HashMap::new();
        for (id, (motion, render_box)) in &mut world.query::<(&Motion, &mut RenderBoxComponent)>() {
            let projected_position = self.camera.project(motion.position);
            render_box.move_to(projected_position);
            projected_positions_by_entity.insert(id, projected_position);
        }

        for (id, (_tracking,)) in &mut world.query::<(&TrackingComponent,)>() {
            self.camera.track(id);
        }

        let screen_center = vecmath::vec2_scale(args.window_size, 0.5);
        match self.camera.tracking {
            TrackingMode::Fixed => {
                self.camera.focus = screen_center;
            }
            TrackingMode::Tracking(e) => {
                let entity_position = projected_positions_by_entity[&e];
                let focus = vecmath::vec2_sub(screen_center, entity_position);
                self.camera.focus = focus;
            }
        }
        context.trans_pos(self.camera.focus)
    }
}

struct Zoom {
    zoom: f64,
    start_zoom: f64,
    target_zoom: f64,
    steps: u32,
    steps_left: u32,
}

impl Zoom {
    fn new(initial_zoom: f64) -> Self {
        Zoom {
            start_zoom: initial_zoom,
            zoom: initial_zoom,
            target_zoom: initial_zoom,
            steps: 0,
            steps_left: 0,
        }
    }

    fn change_zoom(&mut self, new_zoom: f64, steps: u32) {
        if steps == 0 {
            panic!("zoom steps must not be zero");
        }
        self.target_zoom = new_zoom;
        self.start_zoom = self.zoom;
        self.steps = steps;
        self.steps_left = steps;
    }

    fn change_zoom_relative(&mut self, change_by: f64, steps: u32) {
        self.change_zoom(self.zoom + change_by, steps);
    }

    fn update(&mut self) {
        if self.steps_left == 0 {
            return;
        }
        self.steps_left -= 1;
        let start_zoom = self.start_zoom;
        let target_zoom = self.target_zoom;
        let ip = ((self.steps - self.steps_left) as f64 / self.steps as f64) as f64;
        let zoom = interpolation::lerp::<f64>(&start_zoom, &target_zoom, &ip);
        self.zoom = zoom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_zoom() {
        let mut zoom = Zoom::new(512.0);
        zoom.change_zoom(1024.0, 2);
        zoom.update();
        assert_eq!(zoom.zoom, 768.0, "first step of zoom must close by half");
        zoom.update();
        assert_eq!(
            zoom.zoom, 1024.0,
            "second step of zoom must reach the target"
        );
        zoom.update();
        assert_eq!(zoom.zoom, 1024.0, "other steps must not change zoom at all");
    }

    #[test]
    fn zoom_in() {
        let mut zoom = Zoom::new(512.0);
        zoom.change_zoom_relative(256.0, 4);
        assert_eq!(zoom.target_zoom, 768.0);
    }

    #[test]
    fn zoom_out() {
        let mut zoom = Zoom::new(512.0);
        zoom.change_zoom_relative(-256.0, 4);
        assert_eq!(zoom.target_zoom, 256.0);
    }
}
