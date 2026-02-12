use crate::interface::config::RuntimeConfig;
use crate::interface::tui::app::{ActivationState, AppState, SubgraphState};
use crate::interface::tui::datasource::DataSource;
use crate::memory::graph::facade::{ActivationPolicy, GraphFacade, GraphScope, NeighborFilters};
use anyhow::Result;

pub struct GraphSource;

impl DataSource for GraphSource {
    fn tick(&mut self, _cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let scope = GraphScope::Workspace(state.ws.clone());
        let stats = GraphFacade::stats(scope.clone())?;
        state.graph.stats_nodes = stats.nodes;
        state.graph.stats_edges = stats.edges;
        state.graph.backend = stats.backend;

        let mut nodes = GraphFacade::list_nodes(scope.clone())?;
        nodes.sort_by(|a, b| b.last_seen.cmp(&a.last_seen).then_with(|| a.id.cmp(&b.id)));
        state.graph.node_list = nodes.iter().take(200).map(|n| n.id.clone()).collect();
        if state.graph.selected_node.is_none() {
            state.graph.selected_node = state.graph.node_list.first().cloned();
        }
        if state.graph.selected_node.is_none() && !state.graph.node_list.is_empty() {
            state.graph.selected_node = Some(state.graph.node_list[state.graph.selected_index].clone());
        }

        if let Some(id) = state.graph.selected_node.clone() {
            if let Some(node) = GraphFacade::get_node(scope.clone(), &id)? {
                state.graph.selected_node_kind = Some(node.kind);
                state.graph.selected_node_meta = node.meta;
                state.graph.selected_node_last_seen = node.last_seen;
            }
            let sg = GraphFacade::neighbors(
                scope.clone(),
                &id,
                state.graph.depth.max(1),
                NeighborFilters::default(),
            )?;
            state.graph.last_subgraph = Some(SubgraphState {
                nodes: sg.nodes.len(),
                edges: sg.edges.len(),
            });
            state.graph.neighbors_preview = sg
                .edges
                .iter()
                .take(128)
                .map(|e| format!("{} -{}:{:.2}-> {}", e.src, e.rel, e.weight, e.dst))
                .collect();
            if state.graph.activate_requested {
                let act = GraphFacade::activate(scope, &[(id, 1.0)], ActivationPolicy::default())?;
                state.graph.activation_top = act
                    .scores
                    .iter()
                    .take(16)
                    .map(|(k, v)| format!("{k}={v:.3}"))
                    .collect();
                state.graph.last_activation = Some(ActivationState {
                    nodes: act.nodes.len(),
                    edges: act.edges.len(),
                    top_scores: act.scores,
                });
                state.graph.activate_requested = false;
            }
        }
        Ok(())
    }
}
