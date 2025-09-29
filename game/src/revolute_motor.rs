
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*}, event::Event, graph::prelude::*, scene::joint::Joint, script::{ScriptContext, ScriptDeinitContext, ScriptTrait}
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "905cb4ea-a9cc-4d0a-bc06-dd8703b3cf63")]
#[visit(optional)]
pub struct RevoluteMotor {
    // Add fields here.
}

impl ScriptTrait for RevoluteMotor {
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
        let revolute_joint = &mut context.scene.graph[context.handle.transmute::<Joint>()];
        revolute_joint.set_motor_torque_as_revolute(-5.0, 20.0, 0.0).unwrap();
    }
}
    