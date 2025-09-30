use fyrox::{
    core::{
        algebra::Vector3, reflect::prelude::*, type_traits::prelude::*,
        variable::InheritableVariable, visitor::prelude::*,
    },
    event::Event,
    graph::SceneGraph,
    scene::joint::Joint,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::{events::JointBreakEvent, my_event::MyEvent};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "8689ab3b-7c7b-4efc-9859-792a07cd409b")]
#[visit(optional)]
pub struct BreakablePrismaticJoint {
    #[visit(skip)]
    #[reflect(hidden)]
    pub break_event: MyEvent<JointBreakEvent>,
    #[visit(optional)]
    pub target_position: InheritableVariable<f32>,
    #[visit(optional)]
    pub stiffness: InheritableVariable<f32>,
    #[visit(optional)]
    pub max_force: InheritableVariable<f32>,
    #[visit(optional)]
    pub damping: InheritableVariable<f32>,
    #[visit(optional)]
    pub threshold_displacement: InheritableVariable<f32>,
}

impl ScriptTrait for BreakablePrismaticJoint {
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
            .set_motor_target_position_as_prismatic(
                *self.target_position,
                *self.stiffness,
                *self.damping,
                *self.max_force,
            )
            .expect("The joint is supposed to be a ball joint");
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        let joint = context
            .scene
            .graph
            .try_get_mut_of_type::<Joint>(context.handle)
            .expect("BreakableBallJoint must be attached to a Joint node");
        joint
            .set_motor_target_position_as_prismatic(
                *self.target_position,
                *self.stiffness,
                *self.max_force,
                *self.damping,
            )
            .expect("The joint is supposed to be a ball joint");

        println!("Setting motor target position");
        // Put object logic here.
        let joint = context
            .scene
            .graph
            .try_get_of_type::<Joint>(context.handle)
            .expect("BreakableBallJoint must be attached to a Joint node");
        let body1 = context
            .scene
            .graph
            .try_get(joint.body1())
            .expect("Body1 of the ball joint is missing");
        let body2 = context
            .scene
            .graph
            .try_get(joint.body2())
            .expect("Body2 of the ball joint is missing");
        let body1_transform = body1.global_transform();
        let body2_transform = body2.global_transform();
        let position1: Vector3<f32> = body1_transform.fixed_view::<3, 1>(0, 3).into();
        let position2: Vector3<f32> = body2_transform.fixed_view::<3, 1>(0, 3).into();
        let displacement = (position2 - position1).norm();
        if displacement > *self.threshold_displacement {
            self.break_event.fire(
                JointBreakEvent::new(format!(
                    "prismatic joint breaks, displacement: {}",
                    displacement
                )),
                context,
            );
        }
    }
}
