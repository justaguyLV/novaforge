//! Entity-Component-System layer (built on `hecs`) plus the shared engine context.
use hecs::World;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub color: [f32; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Marker: entity bounces off the screen edges.
#[derive(Clone, Copy, Debug)]
pub struct Bounce;

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub x: f32,
    pub y: f32,
    pub down: bool,
}

pub struct Context {
    pub world: World,
    pub input: Input,
    pub screen: (f32, f32),
    pub rng: u64,
    pub time: f32,
}

impl Context {
    fn new() -> Self {
        Self {
            world: World::new(),
            input: Input::default(),
            screen: (1280.0, 720.0),
            rng: 0x9E37_79B9_7F4A_7C15,
            time: 0.0,
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        // xorshift64*
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    }
}

thread_local! {
    static CTX: RefCell<Context> = RefCell::new(Context::new());
}

/// Run a closure with exclusive access to the engine context.
/// Never call into Python while holding this borrow.
pub fn with<R>(f: impl FnOnce(&mut Context) -> R) -> R {
    CTX.with(|c| f(&mut c.borrow_mut()))
}

pub fn color_from_rgb(rgb: i64) -> [f32; 3] {
    [
        ((rgb >> 16) & 0xFF) as f32 / 255.0,
        ((rgb >> 8) & 0xFF) as f32 / 255.0,
        (rgb & 0xFF) as f32 / 255.0,
    ]
}

/// Native systems, run every frame before scripts' rendering.
pub fn physics_system(ctx: &mut Context, dt: f32) {
    let (sw, sh) = ctx.screen;
    for (_, (t, v, b)) in ctx
        .world
        .query_mut::<(&mut Transform, &mut Velocity, Option<&Bounce>)>()
    {
        t.x += v.x * dt;
        t.y += v.y * dt;
        if b.is_some() {
            if t.x < 0.0 {
                t.x = 0.0;
                v.x = v.x.abs();
            } else if t.x + t.w > sw {
                t.x = sw - t.w;
                v.x = -v.x.abs();
            }
            if t.y < 0.0 {
                t.y = 0.0;
                v.y = v.y.abs();
            } else if t.y + t.h > sh {
                t.y = sh - t.h;
                v.y = -v.y.abs();
            }
        }
    }
}
