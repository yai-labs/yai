use crate::agents::{Agent, AgentOutput};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};
use crate::core::runtime::{IceError, RuntimeContext};

pub struct CodeAgent;

impl Agent for CodeAgent {
    fn id(&self) -> AgentId {
        AgentId::Code
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, IceError> {
        Ok(AgentOutput {
            decision: RoutingDecision {
                agent: AgentId::Code,
                command: CommandId::Noop,
            },
            response_text: "Code agent engaged. Preparing analysis.".to_string(),
            llm_prompt: Some(format!("Code task: {}", input)),
        })
    }
}
