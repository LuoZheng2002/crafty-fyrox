use fyrox::script::ScriptMessagePayload;

#[derive(ScriptMessagePayload, Clone, Debug, Default)]
pub struct JointBreakEvent{
    pub message: String,
}

impl JointBreakEvent {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}