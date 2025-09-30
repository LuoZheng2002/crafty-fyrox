//! Game project.
use breakable_ball_joint::BreakableBallJoint;
use breakable_prismatic_joint::BreakablePrismaticJoint;
use component_joint::ComponentJoint;
use fyrox::{
    core::{algebra::Vector2, log::Log, pool::Handle, reflect::prelude::*, visitor::prelude::*},
    dpi::PhysicalSize,
    event::Event,
    graph::prelude::*,
    gui::{
        message::{MessageDirection, UiMessage},
        text::{Text, TextBuilder, TextMessage},
        UserInterface,
    },
    keyboard::KeyCode,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{
        camera::{self, Camera},
        graph::physics::RayCastOptions,
        Scene,
    },
};
use std::path::Path;

// Re-export the engine.
pub use fyrox;

// use crate::resume_physics::ResumePhysics;

mod breakable_ball_joint;
mod breakable_prismatic_joint;
mod component_joint;
mod events;
mod grid_cell;
mod my_event;
mod revolute_motor;
mod test;

#[derive(Clone, Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,
    text: Handle<Text>,
    camera: Handle<Camera>,
}

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) {
        // Register your scripts here.
        let script_constructors = &context.serialization_context.script_constructors;
        script_constructors.add::<BreakableBallJoint>("BreakableBallJoint");
        script_constructors.add::<BreakablePrismaticJoint>("BreakablePrismaticJoint");
        script_constructors.add::<ComponentJoint>("ComponentJoint");
        // script_constructors.add::<ResumePhysics>("ResumePhysics");
        script_constructors.add::<revolute_motor::RevoluteMotor>("RevoluteMotor");
        script_constructors.add::<grid_cell::GridCell>("GridCell");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        context.task_pool.spawn_plugin_task(
            UserInterface::load_from_file("data/ui_scene.ui", context.resource_manager.clone()),
            |result, game: &mut Game, ctx| match result {
                Ok(ui) => {
                    let context_user_interface = ctx.user_interfaces.first_mut();
                    *context_user_interface = ui;
                    let debug_text = context_user_interface
                        .find_by_name_from_root("DebugText")
                        .unwrap();
                    game.text = debug_text.0.transmute();
                    // context_user_interface.try_get(game.text);
                }
                Err(e) => Log::err(format!("Unable to load a user interface! Reason: {:?}", e)),
            },
        );
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext) {
        // Add your global update code here.
        let Some(scene) = context.scenes.try_get_mut(self.scene) else {
            return;
        };
        let debug_text = context
            .user_interfaces
            .first_mut()
            .try_get_mut(self.text)
            .unwrap();
        let display_text = |message: &str| {
            debug_text
                .formatted_text
                .borrow_mut()
                .set_text(message)
                .build();
        };
        if context.input_state.is_key_pressed(KeyCode::Space) {
            scene
                .graph
                .physics
                .enabled
                .set_value_and_mark_modified(!scene.graph.physics.enabled.get_value_ref());
        }
        if context.input_state.is_key_pressed(KeyCode::KeyW) {
            // context
            //     .user_interfaces
            //     .first_mut()
            //     .send_message(TextMessage::text(
            //         self.text.transmute(),
            //         MessageDirection::ToWidget,
            //         "Some text".to_string(),
            //     ));
            display_text("Reset");
        }
        if let Some(camera) = scene.graph.try_get_mut(self.camera) {
            let PhysicalSize { width, height } = context
                .graphics_context
                .as_initialized_ref()
                .window
                .inner_size();
            let screen_size = Vector2::new(width as f32, height as f32);
            let screen_coord = context.input_state.mouse.position;
            let ray = camera.make_ray(screen_coord, screen_size);
            let mut query_buffer = Vec::new();
            scene.graph.physics.cast_ray(
                RayCastOptions {
                    ray_origin: ray.origin.into(),
                    ray_direction: ray.dir,
                    max_len: f32::MAX,
                    groups: Default::default(),
                    sort_results: false,
                },
                &mut query_buffer,
            );
            if let Some(hit) = query_buffer.first() {
                display_text(&format!("Hit: {:?}, at: {}", hit.collider, hit.toi));
            }
        }
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _ui_handle: Handle<UserInterface>,
    ) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
        context.scenes[self.scene]
            .graph
            .physics
            .enabled
            .set_value_and_mark_modified(false);
        // self.camera = context.scenes[self.scene]
        //     .graph
        //     .find_by_name_from_root("Camera")
        //     .unwrap()
        //     .0
        //     .transmute();
        if let Some(camera) = context.scenes[self.scene]
            .graph
            .find_by_name_from_root("Camera")
        {
            self.camera = camera.0.transmute();
        }
    }
}
