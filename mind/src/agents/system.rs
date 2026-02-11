use crate::agents::{Agent, AgentOutput};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};
use crate::core::runtime::{IceError, RuntimeContext};

pub struct SystemAgent;

impl Agent for SystemAgent {
    fn id(&self) -> AgentId {
        AgentId::System
    }

    fn handle(&self, _input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, IceError> {
        Ok(AgentOutput {
            decision: RoutingDecision {
                agent: AgentId::System,
                command: CommandId::Ping,
            },
            response_text: "System online. Executing PING.".to_string(),
            llm_prompt: None,
        })
    }
}
