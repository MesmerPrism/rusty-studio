use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ProjectPanel = Panel{
        SectionTitle{text: "Project"}
        Row{FieldLabel{text: "source"} project_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "project"} project_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "revision"} project_revision := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} validation_status := FieldValue{text: ""}}
    }

    mod.widgets.DiagnosticsPanel = Panel{
        SectionTitle{text: "Validation Diagnostics"}
        ButtonRow{
            next_issue_button := ActionButton{text: "Next Issue"}
        }
        Row{FieldLabel{text: "issues"} validation_issues := SmallValue{text: ""}}
    }

    mod.widgets.GraphPanel = Panel{
        SectionTitle{text: "Graph"}
        ButtonRow{
            previous_graph_button := ActionButton{text: "Prev Graph"}
            next_graph_button := ActionButton{text: "Next Graph"}
        }
        Row{FieldLabel{text: "selected"} graph_selection := FieldValue{text: ""}}
        Row{FieldLabel{text: "graph"} graph_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "target host"} graph_target := FieldValue{text: ""}}
        Row{FieldLabel{text: "counts"} graph_counts := SmallValue{text: ""}}
    }
}
