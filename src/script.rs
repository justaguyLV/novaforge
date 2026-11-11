//! Python scripting host (RustPython) and the `nova` API exposed to scripts.
use crate::ecs::{self, Bounce, Sprite, Transform, Velocity};
use hecs::Entity;
use rustpython_vm::scope::Scope;
use rustpython_vm::{Interpreter, VirtualMachine};

const PRELUDE: &str = include_str!("../assets/prelude.py");

fn ent(id: i64) -> Option<Entity> {
    Entity::from_bits(id as u64)
}

fn py_spawn(x: f64, y: f64, w: f64, h: f64, rgb: i64) -> i64 {
    ecs::with(|c| {
        let e = c.world.spawn((
            Transform { x: x as f32, y: y as f32, w: w as f32, h: h as f32 },
            Sprite { color: ecs::color_from_rgb(rgb) },
        ));
        e.to_bits().get() as i64
    })
}

fn py_despawn(id: i64) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            let _ = c.world.despawn(e);
        });
    }
}

fn py_set_pos(id: i64, x: f64, y: f64) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            if let Ok(mut t) = c.world.get::<&mut Transform>(e) {
                t.x = x as f32;
                t.y = y as f32;
            }
        });
    }
}

fn py_get_pos(id: i64) -> (f64, f64) {
    if let Some(e) = ent(id) {
        return ecs::with(|c| {
            c.world
                .get::<&Transform>(e)
                .map(|t| (t.x as f64, t.y as f64))
                .unwrap_or((0.0, 0.0))
        });
    }
    (0.0, 0.0)
}

fn py_set_size(id: i64, w: f64, h: f64) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            if let Ok(mut t) = c.world.get::<&mut Transform>(e) {
                t.w = w as f32;
                t.h = h as f32;
            }
        });
    }
}

fn py_set_color(id: i64, rgb: i64) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            if let Ok(mut s) = c.world.get::<&mut Sprite>(e) {
                s.color = ecs::color_from_rgb(rgb);
            }
        });
    }
}

fn py_set_vel(id: i64, vx: f64, vy: f64) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            let v = Velocity { x: vx as f32, y: vy as f32 };
            if let Ok(mut cur) = c.world.get::<&mut Velocity>(e) {
                *cur = v;
                return;
            }
            let _ = c.world.insert_one(e, v);
        });
    }
}

fn py_set_bounce(id: i64, on: bool) {
    if let Some(e) = ent(id) {
        ecs::with(|c| {
            if on {
                let _ = c.world.insert_one(e, Bounce);
            } else {
                let _ = c.world.remove_one::<Bounce>(e);
            }
        });
    }
}

fn py_touch() -> (f64, f64, bool) {
    ecs::with(|c| (c.input.x as f64, c.input.y as f64, c.input.down))
}

fn py_screen() -> (f64, f64) {
    ecs::with(|c| (c.screen.0 as f64, c.screen.1 as f64))
}

fn py_rand() -> f64 {
    ecs::with(|c| c.next_f64())
}

fn py_time() -> f64 {
    ecs::with(|c| c.time as f64)
}

fn py_log(msg: String) {
    log::info!("[py] {msg}");
}

fn register(vm: &VirtualMachine, scope: &Scope) {
    macro_rules! reg {
        ($name:literal, $f:expr) => {
            scope
                .globals
                .set_item($name, vm.new_function($name, $f).into(), vm)
                .expect("register native fn");
        };
    }
    reg!("_spawn", py_spawn);
    reg!("_despawn", py_despawn);
    reg!("_set_pos", py_set_pos);
    reg!("_get_pos", py_get_pos);
    reg!("_set_size", py_set_size);
    reg!("_set_color", py_set_color);
    reg!("_set_vel", py_set_vel);
    reg!("_set_bounce", py_set_bounce);
    reg!("_touch", py_touch);
    reg!("_screen", py_screen);
    reg!("_rand", py_rand);
    reg!("_time", py_time);
    reg!("_log", py_log);
}

pub struct ScriptHost {
    interp: Interpreter,
    scope: Scope,
}

impl ScriptHost {
    pub fn new(user_script: &str) -> Self {
        let interp = Interpreter::without_stdlib(Default::default());
        let scope = interp.enter(|vm| {
            let scope = vm.new_scope_with_builtins();
            register(vm, &scope);
            for (name, src) in [("prelude.py", PRELUDE), ("main.py", user_script)] {
                if let Err(e) = vm.run_code_string(scope.clone(), src, name.to_owned()) {
                    log::error!("python error while loading {name}");
                    vm.print_exception(e);
                }
            }
            scope
        });
        Self { interp, scope }
    }

    /// Call a global Python function with the frame delta time.
    pub fn call_update(&self, dt: f32) {
        self.interp.enter(|vm| {
            if let Ok(f) = self.scope.globals.get_item("_engine_update", vm) {
                if let Err(e) = f.call((dt as f64,), vm) {
                    vm.print_exception(e);
                }
            }
        });
    }

    pub fn call_start(&self) {
        self.interp.enter(|vm| {
            if let Ok(f) = self.scope.globals.get_item("_engine_start", vm) {
                if let Err(e) = f.call((), vm) {
                    vm.print_exception(e);
                }
            }
        });
    }
}
