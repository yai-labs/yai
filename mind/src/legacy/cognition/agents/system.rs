use crate::cognition::agents::{Agent, AgentOutput};
use crate::runtime::protocol::{AgentId, CommandId, RoutingDecision};
use crate::runtime::runtime::{Error, RuntimeContext};

pub struct SystemAgent;

impl Agent for SystemAgent {
    fn id(&self) -> AgentId {
        AgentId::System
    }

    fn handle(&self, _input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, Error> {
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
