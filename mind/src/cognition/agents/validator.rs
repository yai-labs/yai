use crate::cognition::agents::{Agent, AgentOutput};
use crate::runtime::protocol::{AgentId, CommandId, RoutingDecision};
use crate::runtime::runtime::{Error, RuntimeContext};

pub struct ValidatorAgent;

impl Agent for ValidatorAgent {
    fn id(&self) -> AgentId {
        AgentId::Validator
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, Error> {
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
