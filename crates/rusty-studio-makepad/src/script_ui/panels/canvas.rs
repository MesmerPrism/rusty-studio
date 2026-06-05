use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.CanvasPanel = Panel{
        SectionTitle{text: "Read-Only Graph Canvas"}
        graph_canvas := StudioGraphCanvas{}
        Rule{}
        Row{FieldLabel{text: "layout"} graph_layout := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "nodes"} graph_nodes := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "edges"} graph_edges := SmallValue{text: ""}}
    }

    mod.widgets.InspectorPanel = Panel{
        SectionTitle{text: "Inspector"}
        ButtonRow{
            next_node_button := ActionButton{text: "Next Node"}
            next_edge_button := ActionButton{text: "Next Edge"}
        }
        Row{FieldLabel{text: "selected node"} selected_node := FieldValue{text: ""}}
        Row{FieldLabel{text: "selected ref"} selected_reference := SmallValue{text: ""}}
        Row{FieldLabel{text: "details"} selected_node_details := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "selected edge"} selected_edge := FieldValue{text: ""}}
        Row{FieldLabel{text: "edge details"} selected_edge_details := SmallValue{text: ""}}
        Row{FieldLabel{text: "issue focus"} focused_issue := SmallValue{text: ""}}
        Row{FieldLabel{text: "authority"} authority_note := SmallValue{text: ""}}
    }
}
