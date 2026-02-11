use crate::agents::{Agent, AgentOutput};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};
use crate::core::runtime::{IceError, RuntimeContext};

pub struct KnowledgeAgent;

impl Agent for KnowledgeAgent {
    fn id(&self) -> AgentId {
        AgentId::Knowledge
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, IceError> {
        Ok(AgentOutput {
            decision: RoutingDecision {
                agent: AgentId::Knowledge,
                command: CommandId::Noop,
            },
            response_text: "Knowledge agent selected. Retrieving context.".to_string(),
            llm_prompt: Some(format!("Knowledge query: {}", input)),
        })
    }
}
