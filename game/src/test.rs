
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "43b9f179-6bcf-4b6c-b4e7-bf7ebbe5fe8f")]
#[visit(optional)]
pub struct Test {
    // Add fields here.
}

impl ScriptTrait for Test {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        // let mut mbc = context.scene.graph.begin_multi_borrow();
        // context.scene.graph.pair_iter_mut()
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
    }
}
    