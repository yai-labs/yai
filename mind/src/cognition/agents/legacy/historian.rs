use crate::cognition::agents::{Agent, AgentOutput};
use crate::runtime::protocol::{AgentId, CommandId, RoutingDecision};
use crate::runtime::runtime::{Error, RuntimeContext};

pub struct HistorianAgent;

impl Agent for HistorianAgent {
    fn id(&self) -> AgentId {
        AgentId::Historian
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, Error> {
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
