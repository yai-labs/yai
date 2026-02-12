use crate::runtime::protocol::{CommandId, RoutingDecision};

#[derive(Debug, Clone, Copy)]
pub enum Intent {
    Ping,
    Validate,
    Audit,
    Code,
    General,
}

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub command: CommandId,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub agent: crate::runtime::protocol::AgentId,
    pub intent: Intent,
    pub steps: Vec<PlanStep>,
    pub input: String,
}

pub fn plan(decision: RoutingDecision, input: &str) -> Result<Plan, String> {
    let intent = match decision.command {
        CommandId::Ping => Intent::Ping,
        CommandId::Noop => Intent::General,
        CommandId::None => Intent::General,
    };

    let mut steps = Vec::new();
    steps.push(PlanStep {
        command: decision.command,
        description: format!("execute {:?}", decision.command),
    });

    Ok(Plan {
        agent: decision.agent,
        intent,
        steps,
        input: input.to_string(),
    })
}
