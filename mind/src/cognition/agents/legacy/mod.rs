use crate::runtime::protocol::{AgentId, RoutingDecision};
use crate::runtime::runtime::{Error, RuntimeContext};

pub mod code;
pub mod historian;
pub mod knowledge;
pub mod system;
pub mod validator;

#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub decision: RoutingDecision,
    pub response_text: String,
    pub llm_prompt: Option<String>,
}

#[allow(dead_code)]
pub trait Agent {
    fn id(&self) -> AgentId;
    fn handle(&self, input: &str, ctx: &mut RuntimeContext) -> Result<AgentOutput, Error>;
}

pub fn get_agent(agent_id: AgentId) -> Option<Box<dyn Agent>> {
    match agent_id {
        AgentId::System => Some(Box::new(system::SystemAgent)),
        AgentId::Code => Some(Box::new(code::CodeAgent)),
        AgentId::Knowledge => Some(Box::new(knowledge::KnowledgeAgent)),
        AgentId::Validator => Some(Box::new(validator::ValidatorAgent)),
        AgentId::Historian => Some(Box::new(historian::HistorianAgent)),
    }
}
