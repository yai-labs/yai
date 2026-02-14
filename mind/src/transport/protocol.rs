pub type AgentId = String;
pub type CommandId = String;

#[derive(Debug, Clone)]
pub enum RoutingDecision {
    Allow,
    Deny,
}
