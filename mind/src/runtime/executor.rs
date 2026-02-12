use crate::runtime::planner::Plan;
use crate::runtime::runtime::Error;
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub command: crate::runtime::protocol::CommandId,
    pub response: String,
}

pub fn execute(
    plan: &Plan,
    ctx: &mut crate::runtime::runtime::RuntimeContext,
) -> Result<ExecutionResult, Error> {
    let step = plan
        .steps
        .first()
        .ok_or_else(|| Error::Execution("plan has no steps".to_string()))?;

    let response = ctx
        .scheduler
        .send_command_and_wait(step.command, std::time::Duration::from_secs(2))
        .map_err(Error::Execution)?;

    Ok(ExecutionResult {
        command: step.command,
        response,
    })
}
