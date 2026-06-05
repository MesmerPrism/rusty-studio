use super::super::*;

pub(crate) fn validation_issue_lines(model: &StudioViewModel) -> String {
    if model.validation_issues.is_empty() {
        return "none".to_string();
    }
    model
        .validation_issues
        .iter()
        .map(|issue| {
            let issue_code = issue.issue_code.as_deref().unwrap_or("unknown_issue");
            let mut lines = vec![format!("{} [{}]", issue.check_id, issue_code)];
            if let Some(graph_id) = issue.graph_id.as_deref() {
                let graph_label = if issue.targets_selected_graph {
                    "selected graph"
                } else {
                    "graph"
                };
                lines.push(format!("  {graph_label}: {graph_id}"));
            }
            if !issue.node_ids.is_empty() {
                lines.push(format!("  nodes: {}", issue.node_ids.join(", ")));
            }
            if !issue.edge_ids.is_empty() {
                lines.push(format!("  edges: {}", issue.edge_ids.join(", ")));
            }
            if !issue.reference_ids.is_empty() {
                lines.push(format!("  refs: {}", issue.reference_ids.join(", ")));
            }
            lines.push(format!("  {}", issue.evidence));
            lines.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
