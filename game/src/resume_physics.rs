
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*}, event::Event, graph::prelude::*, keyboard::KeyCode, script::{ScriptContext, ScriptDeinitContext, ScriptTrait}
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "3e104901-538e-43b7-ae62-06956fea3bc3")]
#[visit(optional)]
pub struct ResumePhysics {
    // Add fields here.
}

impl ScriptTrait for ResumePhysics {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        if context.input_state.is_key_pressed(KeyCode::Space){
            context.scene.graph.physics.enabled.set_value_and_mark_modified(true);
        }
    }
}
    