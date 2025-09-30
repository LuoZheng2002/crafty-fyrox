//! Executor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::dpi::{PhysicalSize, Size};
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;

fn main() {
    Log::set_file_name("crafty.log");
    let mut window_attributes = WindowAttributes::default();
    window_attributes.resizable = true;
    window_attributes.title = "Fyrox Game".to_string();
    window_attributes.inner_size = Some(Size::Physical(PhysicalSize::new(1920, 1080)));
    let graphics_context_params = GraphicsContextParams {
        window_attributes,
        vsync: true,
        msaa_sample_count: None,
        graphics_server_constructor: Default::default(),
        named_objects: false,
    };

    // let mut executor = Executor::new(Some(EventLoop::new().unwrap()));
    let mut executor =
        Executor::from_params(Some(EventLoop::new().unwrap()), graphics_context_params);

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        executor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use crafty::Game;
        executor.add_plugin(Game::default());
    }

    executor.run()
}
