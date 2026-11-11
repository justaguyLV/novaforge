# NovaForge

Mobile-first game engine written in Rust.

- **Rendering**: wgpu (Vulkan/GLES on Android), batched 2D renderer
- **ECS**: hecs, native physics system (velocity + edge bounce)
- **Scripting**: Python (RustPython) with a Unity-style API: `GameObject`, `Behaviour`, `Input`, `Random`
- **Target**: Android APK (built by GitHub Actions) + desktop for fast iteration

Edit `assets/main.py` to change the game. Download the APK from the Actions run artifacts.
