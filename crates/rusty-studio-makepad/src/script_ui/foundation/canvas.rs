use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.StudioGraphCanvasBase = #(StudioGraphCanvas::register_widget(vm))
    mod.widgets.StudioGraphCanvas = set_type_default() do StudioGraphCanvasBase{
        width: Fill
        height: 280.0
        draw_bg +: {
            draw_depth: 0.0
            color: #xf8fafc
        }
        draw_edge +: {
            draw_depth: 1.0
        }
        draw_node +: {
            draw_depth: 2.0
            color: #xffffffff
        }
        draw_text +: {
            draw_depth: 3.0
            color: #x17202a
            text_style.font_size: 10.0
        }
        bg_color: #xf8fafc
        node_color: #xffffffff
        node_selected_color: #xe7f1ff
        node_issue_color: #xfff4e5
        edge_color: #x64748b
        edge_selected_color: #x1d4ed8
        edge_issue_color: #xd97706
        border_color: #xcbd5e1
        selected_border_color: #x2563eb
        text_color: #x17202a
        issue_text_color: #x9a3412
    }
}
