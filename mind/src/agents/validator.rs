use crate::agents::{Agent, AgentOutput};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};
use crate::core::runtime::{IceError, RuntimeContext};

pub struct ValidatorAgent;

impl Agent for ValidatorAgent {
    fn id(&self) -> AgentId {
        AgentId::Validator
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, IceError> {
        Ok(AgentOutput {
            decision: RoutingDecision {
                agent: AgentId::Validator,
                command: CommandId::Noop,
            },
            response_text: "Validator engaged. Running compliance pass.".to_string(),
            llm_prompt: Some(format!("Validate compliance and respond: {}", input)),
        })
    }
}
