use crate::cognition::agents::{Agent, AgentOutput};
use crate::runtime::protocol::{AgentId, CommandId, RoutingDecision};
use crate::runtime::runtime::{Error, RuntimeContext};

pub struct KnowledgeAgent;

impl Agent for KnowledgeAgent {
    fn id(&self) -> AgentId {
        AgentId::Knowledge
    }

    fn handle(&self, input: &str, _ctx: &mut RuntimeContext) -> Result<AgentOutput, Error> {
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
