use std::any::TypeId;

use fyrox::{
    core::{
        arrayvec::ArrayVec, log::{Log, MessageKind}, pool::{ErasedHandle, Handle}, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*, warn
    },
    event::Event,
    graph::{BaseSceneGraph, SceneGraph, SceneGraphNode},
    scene::{
        joint::{Joint, JointBuilder, JointParams},
        node::Node,
        rigidbody::RigidBody,
    },
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::{
    breakable_ball_joint::BreakableBallJoint, breakable_prismatic_joint::BreakablePrismaticJoint,
    events::JointBreakEvent,
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "c67d8138-7262-4825-bff1-d501d29c2483")]
#[visit(optional)]
pub struct ComponentJoint {
    // Add fields here.
    #[visit(optional)]
    ball_joint1: Handle<Joint>,
    #[visit(optional)]
    ball_joint2: Handle<Joint>,
    #[visit(optional)]
    prismatic_joint: Handle<Joint>,
    #[visit(optional)]
    pub connected_body1: Handle<RigidBody>, // connect to the second body of ball joint 1
    #[visit(optional)]
    pub connected_body2: Handle<RigidBody>, // connect to the second body of ball joint 2
}

impl ScriptTrait for ComponentJoint {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        context
            .message_dispatcher
            .subscribe_to::<JointBreakEvent>(context.handle);
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        {
            // There should be a logic that depends on other scripts in scene.
            // It is called right after **all** scripts were initialized.
            let current_node = context
                .scene
                .graph
                .try_get(context.handle)
                .expect("ComponentJoint must be attached to a Node");            
            let ball_joint1_script = context
                .scene
                .graph
                .try_get_script_of::<BreakableBallJoint>(self.ball_joint1.transmute())
                .expect("BreakableBallJoint script must be attached to the first ball joint");
            let ball_joint2_script = context
                .scene
                .graph
                .try_get_script_of::<BreakableBallJoint>(self.ball_joint2.transmute())
                .expect("BreakableBallJoint script must be attached to the second ball joint");
            let prismatic_joint_script = context
                .scene
                .graph
                .try_get_script_of::<BreakablePrismaticJoint>(self.prismatic_joint.transmute())
                .expect("BreakablePrismaticJoint script must be attached to the prismatic joint");
            ball_joint1_script.break_event.subscribe(context.handle);
            ball_joint2_script.break_event.subscribe(context.handle);
            prismatic_joint_script.break_event.subscribe(context.handle);
        }
        // add the connection
        // let connected_body1 = context.scene.graph.try_get_mut_of
        let mbc = context.scene.graph.begin_multi_borrow();
        let mut ball_joint1 = mbc
            .try_get_component_of_type_mut::<Joint>(self.ball_joint1.transmute())
            .unwrap();
        let mut ball_joint2 = mbc
            .try_get_component_of_type_mut::<Joint>(self.ball_joint2.transmute())
            .unwrap();
        ball_joint1.set_body2(self.connected_body1.transmute());
        ball_joint2.set_body2(self.connected_body2.transmute());
        println!("Successfully connected ball joints to the bodies");
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
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(joint_break_event) = message.downcast_ref::<JointBreakEvent>() {
            Log::set_verbosity(MessageKind::Warning);
            Log::warn(format!("Joint break event received: {}", joint_break_event.message));
            // println!("A joint breaks, deleting current node");
            // ctx.scene.graph.remove_node(ctx.handle);
        }
    }
}
