use fyrox::{
    core::{pool::Handle, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*},
    event::Event,
    graph::prelude::*,
    material::MaterialResource,
    scene::mesh::{
        surface::{self, Surface, SurfaceResource},
        Mesh,
    },
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::events::RayHitEvent;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "8ae58bd6-c48b-41f4-9623-d626802029f2")]
#[visit(optional)]
pub struct GridCell {
    // Add fields here.
    mesh: Handle<Mesh>,
    grid_idle_material: MaterialResource,
    grid_highlighted_material: MaterialResource,
}

impl ScriptTrait for GridCell {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        // self.mesh = context.handle.transmute();
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

    fn on_message(
        &mut self,
        #[allow(unused_variables)] message: &mut dyn fyrox::script::ScriptMessagePayload,
        #[allow(unused_variables)] ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(_ray_hit) = message.downcast_ref::<RayHitEvent>() {
            let mesh = ctx.scene.graph.try_get_mut(self.mesh).unwrap();
            let mut new_surface = Surface::new(surface::CUBE.resource.clone());
            new_surface.set_material(self.grid_highlighted_material.clone());
            mesh.set_surfaces(vec![new_surface]);
        }
    }
}
