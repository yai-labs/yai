use crate::agents::{get_agent, AgentOutput};
use crate::core::executor::{execute, ExecutionResult};
use crate::core::governance::{GovernanceEngine, RoutingEngine};
use crate::core::planner::{plan, Plan};
use crate::core::protocol::RoutingDecision;
use crate::core::scheduler::Scheduler;
use crate::llm::adapter::LlmClient;
use crate::memory::legacy::store::MemoryCore;
use crate::memory::legacy::types::EventKind;
use crate::rag::pipeline::build_prompt;
use std::fmt;

#[derive(Debug)]
pub enum IceError {
    Routing(String),
    Planning(String),
    Execution(String),
    Llm(String),
    Memory(String),
}

impl fmt::Display for IceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IceError::Routing(msg) => write!(f, "routing: {}", msg),
            IceError::Planning(msg) => write!(f, "planning: {}", msg),
            IceError::Execution(msg) => write!(f, "execution: {}", msg),
            IceError::Llm(msg) => write!(f, "llm: {}", msg),
            IceError::Memory(msg) => write!(f, "memory: {}", msg),
        }
    }
}

impl std::error::Error for IceError {}

pub struct RuntimeContext {
    pub scheduler: Scheduler,
    pub llm: Box<dyn LlmClient>,
    pub memory: MemoryCore,
    pub trace_id: String,
    pub workspace_id: String,
}

#[derive(Debug)]
pub struct TurnResult {
    pub decision: RoutingDecision,
    pub plan: Plan,
    pub execution: ExecutionResult,
    pub agent_output: AgentOutput,
    pub llm_response: Option<String>,
}

pub fn run_turn(user_text: &str, ctx: &mut RuntimeContext) -> Result<TurnResult, IceError> {
    let governance = GovernanceEngine::new();
    let decision = RoutingEngine::route_intent(user_text);

    let agent = get_agent(decision.agent).ok_or_else(|| {
        IceError::Routing(format!("No agent found for {:?}", decision.agent))
    })?;
    let agent_output = agent.handle(user_text, ctx)?;
    let _ = ctx
        .memory
        .put_event(&ctx.workspace_id, &ctx.trace_id, EventKind::User, user_text)
        .map_err(IceError::Memory);

    let llm_response = match &agent_output.llm_prompt {
        Some(prompt) => {
            let full_prompt = build_prompt(prompt, &ctx.memory, &ctx.workspace_id);
            Some(ctx.llm.complete(&full_prompt)?)
        }
        None => None,
    };

    let agent_payload = llm_response
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or(&agent_output.response_text);
    let _ = ctx
        .memory
        .put_event(&ctx.workspace_id, &ctx.trace_id, EventKind::Agent, agent_payload)
        .map_err(IceError::Memory);

    // Planner produces a plan from routing decision + input
    let plan = plan(agent_output.decision, user_text).map_err(IceError::Planning)?;

    // Execute plan via scheduler (Vault)
    let execution = execute(&plan, ctx)?;

    // Governance currently not enforcing here, but kept for future policy hooks
    let _ = governance.foundation_version;

    Ok(TurnResult {
        decision,
        plan,
        execution,
        agent_output,
        llm_response,
    })
}

#[cfg(test)]
mod tests {
    use crate::core::governance::RoutingEngine;
    use crate::core::planner::plan;
    use crate::core::protocol::AgentId;

    #[test]
    fn routing_code_fix_goes_to_code() {
        let d = RoutingEngine::route_intent("please fix this bug in code");
        assert_eq!(d.agent, AgentId::Code);
    }

    #[test]
    fn routing_audit_log_goes_to_historian() {
        let d = RoutingEngine::route_intent("audit the last log event");
        assert_eq!(d.agent, AgentId::Historian);
    }

    #[test]
    fn planner_produces_non_empty_plan() {
        let d = RoutingEngine::route_intent("ping");
        let p = plan(d, "ping").expect("plan");
        assert!(!p.steps.is_empty());
    }
}
