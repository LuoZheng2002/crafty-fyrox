//! Game project.
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    event::Event,
    gui::{message::UiMessage, UserInterface},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;
use breakable_ball_joint::BreakableBallJoint;
use breakable_prismatic_joint::BreakablePrismaticJoint;
use component_joint::ComponentJoint;

// Re-export the engine.
pub use fyrox;

use crate::resume_physics::ResumePhysics;

mod breakable_ball_joint;
mod breakable_prismatic_joint;
mod component_joint;
mod events;
mod my_event;
mod test;
mod resume_physics;
mod revolute_motor;

#[derive(Clone, Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) {
        // Register your scripts here.
        let script_constructors = &context.serialization_context.script_constructors;
        script_constructors.add::<BreakableBallJoint>("BreakableBallJoint");
        script_constructors.add::<BreakablePrismaticJoint>("BreakablePrismaticJoint");
        script_constructors.add::<ComponentJoint>("ComponentJoint");
        script_constructors.add::<ResumePhysics>("ResumePhysics");
        script_constructors.add::<revolute_motor::RevoluteMotor>("RevoluteMotor");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _ui_handle: Handle<UserInterface>
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
        context.scenes[self.scene].graph.physics.enabled.set_value_and_mark_modified(false);
    }
}
