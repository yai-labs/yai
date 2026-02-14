// Tipi attesi dai file legacy (crate::cognition::agents::{Agent, AgentOutput})
#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub text: String,
}

pub trait Agent: Send + Sync {
    fn id(&self) -> &'static str;
}


