// src/core/governance.rs
#![allow(dead_code)]
use crate::models::{IceMessage, MessageType};
use crate::core::protocol::{AgentId, CommandId, RoutingDecision};

/// Rappresenta il verdetto di conformità I-003
pub struct GovernanceVerdict {
    pub is_valid: bool,
    pub reason: String,
    pub authority_id: Option<String>,
}

pub struct GovernanceEngine {
    pub foundation_version: String,
}

impl GovernanceEngine {
    pub fn new() -> Self {
        Self {
            foundation_version: "1.0.0".to_string(),
        }
    }

    /// Valida un messaggio secondo i vincoli strutturali di GOVERNANCE.md
    pub fn validate_compliance(&self, message: &IceMessage) -> GovernanceVerdict {
        // Vincolo: Authority must be EXPLICIT (deve esserci un ID nel payload)
        let authority = message.payload.get("authority_id")
            .and_then(|v| v.as_str());

        if message.r#type == MessageType::Intent && authority.is_none() {
            return GovernanceVerdict {
                is_valid: false,
                reason: "Violation I-003: Authority is not explicit. Execution invalid.".to_string(),
                authority_id: None,
            };
        }

        // Vincolo: Traceability (deve esserci un riferimento al task originario)
        if !message.payload.contains_key("trace_id") {
             return GovernanceVerdict {
                is_valid: false,
                reason: "Violation I-003: Authority is not traceable.".to_string(),
                authority_id: authority.map(|s| s.to_string()),
            };
        }

        GovernanceVerdict {
            is_valid: true,
            reason: "Compliant with YAI Foundation v1.0.0".to_string(),
            authority_id: authority.map(|s| s.to_string()),
        }
    }
}

pub struct RoutingEngine;

impl RoutingEngine {
    pub fn route_intent(text: &str) -> RoutingDecision {
        let q = text.to_lowercase();
        if q.contains("ping") || q == "ping" {
            return RoutingDecision { agent: AgentId::System, command: CommandId::Ping };
        }
        if q == "ciao" || q == "hello" || q == "hi" || q == "hey" {
            return RoutingDecision { agent: AgentId::Knowledge, command: CommandId::Noop };
        }
        if q.contains("audit") || q.contains("log") {
            return RoutingDecision { agent: AgentId::Historian, command: CommandId::Noop };
        }
        if q.contains("validate") || q.contains("compliance") {
            return RoutingDecision { agent: AgentId::Validator, command: CommandId::Noop };
        }
        if q.contains("code") || q.contains("fix") || q.contains("patch") {
            return RoutingDecision { agent: AgentId::Code, command: CommandId::Noop };
        }
        RoutingDecision { agent: AgentId::Knowledge, command: CommandId::Noop }
    }
}
