use super::*;

pub(crate) fn layout_lines(graph: &StudioGraphView) -> String {
    let Some(layout) = graph.layout.as_ref() else {
        return "none".to_string();
    };
    let mut lines = vec![format!(
        "{} / {} / {} node(s) / {} edge(s)",
        layout.layout_id, layout.coordinate_space, layout.node_count, layout.edge_count
    )];
    for node in &layout.nodes {
        lines.push(format!(
            "{} @ {},{} {}x{}{}",
            node.node_id,
            node.x,
            node.y,
            node.width,
            node.height,
            issue_count_line(node.validation_issue_count)
        ));
    }
    for edge in &layout.edges {
        lines.push(format!(
            "{} route: {}{}",
            edge.edge_id,
            edge.route,
            issue_count_line(edge.validation_issue_count)
        ));
    }
    lines.join("\n")
}

pub(crate) fn node_lines(graph: &StudioGraphView) -> String {
    graph
        .node_rows
        .iter()
        .map(|node| {
            format!(
                "{} [{}]\n  ref: {}{}",
                node.label,
                node.kind,
                node.reference_id,
                issue_count_line(node.validation_issue_count)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn edge_lines(graph: &StudioGraphView) -> String {
    graph
        .edge_rows
        .iter()
        .map(|edge| {
            format!(
                "{} [{}]\n  {} -> {}{}",
                edge.edge_id,
                edge.kind,
                edge.source_node_id,
                edge.target_node_id,
                issue_count_line(edge.validation_issue_count)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn issue_count_line(count: usize) -> String {
    match count {
        0 => String::new(),
        1 => "\n  issues: 1".to_string(),
        _ => format!("\n  issues: {count}"),
    }
}
