use crate::physics::motion::Motion;
use crate::render::render_box::RenderBoxComponent;
use crate::render::sprite::{Sprite, SpriteKind};
use graphics::types::Color;
use graphics::{Context, Ellipse};
use hecs::World;
use opengl_graphics::GlGraphics;

const DEFAULT_TTL: usize = 255;
const INITIAL_ALPHA: f32 = 0.5;
const DELTA_ALPHA: f32 = INITIAL_ALPHA / DEFAULT_TTL as f32;

/// component that leaves fading traces of the object
pub struct TraceComponent {
    color: Color,
    ttl: usize,
}

impl TraceComponent {
    /// create a default trace with initial color
    ///
    /// the trace begins with an initial color faded by half.
    /// it will fade in an disappear in 256 frames.
    pub fn new(initial_color: Color) -> Self {
        let mut faded_color = initial_color;
        faded_color[3] = INITIAL_ALPHA;
        TraceComponent {
            color: faded_color,
            ttl: DEFAULT_TTL,
        }
    }

    /// get trace shape
    fn shape(&self) -> Ellipse {
        Ellipse::new(self.color)
    }

    /// update the trace. fade the color and reduce ttl
    fn update(&mut self) {
        self.ttl -= 1;
        self.color[3] -= DELTA_ALPHA;
    }

    /// returns true when trace expires
    fn expired(&self) -> bool {
        self.ttl == 0
    }
}

/// renders and updates object traces
pub struct RenderTraceSystem {}

impl RenderTraceSystem {
    /// create a default trace system
    pub fn new() -> Self {
        RenderTraceSystem {}
    }

    pub fn update(&self, world: &mut World, context: Context, gl: &mut GlGraphics) {
        let draw_state = &context.draw_state;
        // render the trace
        let mut expired_traces = vec![];
        for (id, (trace, render_box)) in
            &mut world.query::<(&mut TraceComponent, &RenderBoxComponent)>()
        {
            trace.update();
            let shape: Ellipse = trace.shape();
            shape.draw(render_box.bound(), draw_state, context.transform, gl);
            if trace.expired() {
                expired_traces.push(id);
            }
        }

        expired_traces.iter().for_each(|id| {
            world.despawn(*id).expect("can't despawn the trace");
        })
    }
}

/// component to spawn traces
pub struct SpawnTraceSystem {}

impl SpawnTraceSystem {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SpawnTraceSystem {}
    }
}

const SPAWN_INTERVAL: usize = 16;
pub struct TraceSpawnSystem {
    ticks_since_spawn: usize,
}

impl TraceSpawnSystem {
    pub fn new() -> Self {
        TraceSpawnSystem {
            ticks_since_spawn: SPAWN_INTERVAL,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        self.ticks_since_spawn -= 1;
        if self.ticks_since_spawn == 0 {
            let mut traces: Vec<(RenderBoxComponent, Color, Motion)> = vec![];
            for (_id, (render_box, sprite, motion, _trace_spawn)) in
                &mut world.query::<(&RenderBoxComponent, &Sprite, &Motion, &SpawnTraceSystem)>()
            {
                match sprite.kind() {
                    SpriteKind::Circle(_, color) => {
                        let trace_box = *render_box;
                        traces.push((trace_box, *color, *motion));
                    }
                    SpriteKind::Image(_, _) => {}
                }
            }

            for (render_box, color, motion) in traces.iter() {
                let mut m = *motion;
                m.velocity = [0.0, 0.0];
                m.acceleration = [0.0, 0.0];
                world.spawn((TraceComponent::new(*color), *render_box, m));
            }

            self.ticks_since_spawn = SPAWN_INTERVAL;
        }
    }
}
