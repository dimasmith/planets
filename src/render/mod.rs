use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

use crate::model::World;
use crate::physics::motion::Position;
use crate::render::renderable::Renderable;

pub mod renderable;

pub struct Renderer {
    pub zoom: f64,
    pub gl: GlGraphics,
}

pub struct Projector {
    pub zoom: f64,
}

impl Projector {
    pub fn project(&self, coords: &Vector2<f64>) -> Vector2<f64> {
        vecmath::vec2_scale(*coords, self.zoom)
    }
}

const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer { zoom: 2.5e-4, gl }
    }

    fn projector(&self) -> Projector {
        Projector { zoom: self.zoom }
    }

    fn center_point(&self, world: &World, args: RenderArgs) -> (f64, f64) {
        let mut position = world
            .planets
            .iter()
            .map(|p| p.motion.position)
            .fold([0.0, 0.0], |a, p| vecmath::vec2_add(a, p));
        position = vecmath::vec2_scale(position, 1.0 / world.planets.len() as f64);
        position = self.projector().project(&position);
        (
            args.window_size[0] / 2.0 - position[0],
            args.window_size[1] / 2.0 - position[1],
        )
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let mut context = self.gl.draw_begin(args.viewport());
        let (x, y) = self.center_point(world, args);
        let transform = context.trans(x, y).transform;

        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(BACK, gl);
        });
        let projector = self.projector();

        for planet in world.planets.iter_mut() {
            planet.render_all(&projector, transform, &mut context, &mut self.gl);
        }

        let mut center = GeoCenter::new(&world);
        center.render_all(&projector, transform, &mut context, &mut self.gl);

        self.gl.draw_end();
    }
}

struct GeoCenter {
    position: Position,
}

impl GeoCenter {
    fn new(world: &World) -> GeoCenter {
        let mut position = world
            .planets
            .iter()
            .map(|p| p.motion.position)
            .fold([0.0, 0.0], |a, p| vecmath::vec2_add(a, p));
        position = vecmath::vec2_scale(position, 1.0 / world.planets.len() as f64);
        GeoCenter { position }
    }
}

impl Renderable for GeoCenter {
    fn render(
        &mut self,
        projector: &Projector,
        transform: [[f64; 3]; 2],
        _context: &mut Context,
        gl: &mut GlGraphics,
    ) {
        let position: Position = projector.project(&self.position);
        let bound = graphics::rectangle::centered_square(position[0], position[1], 10.0);
        graphics::ellipse([1.0, 0.0, 0.0, 1.0], bound, transform, gl);
    }
}
