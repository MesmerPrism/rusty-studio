#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct StudioGraphCanvasModel {
    pub(crate) layout_id: String,
    pub(crate) coordinate_space: String,
    pub(crate) nodes: Vec<StudioGraphCanvasNode>,
    pub(crate) edges: Vec<StudioGraphCanvasEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StudioGraphCanvasNode {
    pub(crate) node_id: String,
    pub(crate) label: String,
    pub(crate) kind: String,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) validation_issue_count: usize,
    pub(crate) selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StudioGraphCanvasEdge {
    pub(crate) edge_id: String,
    pub(crate) source_node_id: String,
    pub(crate) target_node_id: String,
    pub(crate) route: String,
    pub(crate) validation_issue_count: usize,
    pub(crate) selected: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) enum StudioGraphCanvasAction {
    #[default]
    None,
    SelectNode(String),
    SelectEdge(String),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StudioGraphCanvasHit {
    Node(String),
    Edge(String),
}
