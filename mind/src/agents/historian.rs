use crate::agents::{Agent, AgentOutput};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};
use crate::core::runtime::{IceError, RuntimeContext};

pub struct HistorianAgent;

impl Agent for HistorianAgent {
    fn id(&self) -> AgentId {
        AgentId::Historian
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, IceError> {
        Ok(AgentOutput {
            decision: RoutingDecision {
                agent: AgentId::Historian,
                command: CommandId::Noop,
            },
            response_text: "Historian engaged. Preparing audit record.".to_string(),
            llm_prompt: Some(format!("Summarize and audit: {}", input)),
        })
    }
}
