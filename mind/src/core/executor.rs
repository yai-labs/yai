use crate::core::planner::Plan;
use crate::core::runtime::IceError;
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub command: crate::core::protocol::CommandId,
    pub response: String,
}

pub fn execute(plan: &Plan, ctx: &mut crate::core::runtime::RuntimeContext) -> Result<ExecutionResult, IceError> {
    let step = plan.steps.first().ok_or_else(|| {
        IceError::Execution("plan has no steps".to_string())
    })?;

    let response = ctx
        .scheduler
        .send_command_and_wait(step.command, std::time::Duration::from_secs(2))
        .map_err(IceError::Execution)?;

    Ok(ExecutionResult {
        command: step.command,
        response,
    })
}
