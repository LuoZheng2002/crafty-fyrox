use std::any::TypeId;

use fyrox::{
    core::{
        algebra::{Quaternion, UnitQuaternion},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    event::Event,
    graph::SceneGraph,
    scene::{joint::Joint, node::Node},
    script::{ScriptContext, ScriptDeinitContext, ScriptMessage, ScriptTrait},
};

use crate::{events::JointBreakEvent, my_event::MyEvent};

#[derive(Visit, Reflect, Clone, Debug, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "ecf85b97-9a16-4072-8740-a52d8e80178b")]
#[visit(optional)]
pub struct BreakableBallJoint {
    #[visit(skip)]
    #[reflect(hidden)]
    pub break_event: MyEvent<JointBreakEvent>,
    #[visit(optional)]
    pub stiffness: InheritableVariable<f32>,
    #[visit(optional)]
    pub max_torque: InheritableVariable<f32>,
    #[visit(optional)]
    pub damping: InheritableVariable<f32>,
    #[visit(optional)]
    pub threshold_angle: InheritableVariable<f32>,
}

impl BreakableBallJoint {}

impl ScriptTrait for BreakableBallJoint {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
        let joint = context
            .scene
            .graph
            .try_get_mut_of_type::<Joint>(context.handle)
            .expect("BreakableBallJoint must be attached to a Joint node");
        joint
            .set_motor_resistive_torque_as_ball(*self.stiffness, *self.damping, *self.max_torque)
            .expect("The joint is supposed to be a ball joint");
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        // let mbc = context.scene.graph.begin_multi_borrow();
        // let joint = mbc.try_get_component_of_type_mut::<Joint>(context.handle).expect("BreakableBallJoint must be attached to a Joint node");
        // let body1 = mbc.try_get_mut(joint.body1()).expect("Body1 of the ball joint is missing");
        // let body2 = mbc.typed_ref(joint.body2()).expect("Body2 of the ball joint is missing");
        let joint = context
            .scene
            .graph
            .try_get_of_type::<Joint>(context.handle)
            .expect("BreakableBallJoint must be attached to a Joint node");
        let body1 = context
            .scene
            .graph
            .typed_ref(joint.body1())
            .expect("Body1 of the ball joint is missing");
        let body2 = context
            .scene
            .graph
            .typed_ref(joint.body2())
            .expect("Body2 of the ball joint is missing");
        let body1_transform = body1.global_transform();
        let body2_transform = body2.global_transform();
        let angle1 = UnitQuaternion::from_matrix(&body1_transform.fixed_view::<3, 3>(0, 0).into());
        let angle2 = UnitQuaternion::from_matrix(&body2_transform.fixed_view::<3, 3>(0, 0).into());
        let relative_rotation = angle1.inverse() * angle2;
        let angle = relative_rotation.angle();
        if angle > *self.threshold_angle {
            self.break_event.fire(JointBreakEvent, context);
        }
    }
}
