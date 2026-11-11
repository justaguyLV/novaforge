//! Desktop entry point (for fast iteration). Android uses `android_main` in lib.rs.
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("event loop");
    novaforge::app::run(event_loop);
}
