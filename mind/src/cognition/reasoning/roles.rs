use crate::memory::graph::domains::activation::api::ActivationResult;
use crate::types::graph::{GraphEdge, GraphNode};
use anyhow::{bail, Result};
use serde_json::Value;

/// Bridge tra il Facade del grafo e l'algoritmo di Spreading Activation.
/// Questa funzione viene chiamata dal Facade per calcolare i pesi dei nodi.
pub fn run_activation_logic(
    _nodes: &[GraphNode],
    _edges: &[GraphEdge],
    _seeds: &[(String, f64)],
    _params: &Value,
) -> Result<ActivationResult> {
    // NOTA: Per un'implementazione completa, qui dovresti mappare
    // i GraphNode in una struttura che implementa il trait ActivationGraph.
    // Per ora lanciamo un errore controllato per far compilare il resto.

    bail!("Activation logic bridge not fully implemented yet: mapping from GraphNode to ActivationGraph required.")
}
