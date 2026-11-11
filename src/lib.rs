//! NovaForge: mobile-first Rust game engine with ECS, wgpu rendering and Python scripting.
pub mod app;
pub mod ecs;
pub mod renderer;
pub mod script;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    use winit::event_loop::EventLoop;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let event_loop = EventLoop::builder()
        .with_android_app(android_app)
        .build()
        .expect("event loop");
    app::run(event_loop);
}
