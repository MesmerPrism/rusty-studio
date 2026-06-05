use super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass.clear_color: #xf4f6f7
                window.inner_size: vec2(1180, 820)
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 0.0

                    SolidView{
                        width: Fill height: Fit
                        padding: Inset{left: 24.0 right: 24.0 top: 18.0 bottom: 16.0}
                        flow: Right
                        align: Align{y: 0.5}
                        draw_bg.color: #xfbfcf8

                        View{
                            width: Fill height: Fit
                            flow: Down
                            spacing: 3.0
                            PageTitle{text: "Rusty Studio"}
                            subtitle_label := Label{
                                text: "schema-first package/profile authoring surface"
                                draw_text.color: #x5d6875
                                draw_text.text_style.font_size: 12.0
                            }
                        }
                        mode_label := Label{
                            width: Fit height: Fit
                            text: "core-gated edits"
                            draw_text.color: #x2f6f5e
                            draw_text.text_style: theme.font_bold{font_size: 13.0}
                        }
                    }

                    Rule{}

                    ScrollYView{
                        width: Fill height: Fill
                        padding: 18.0
                        flow: Down
                        spacing: 12.0

                        ProjectPanel{}
                        DiagnosticsPanel{}
                        GraphPanel{}
                        ShellPreviewPanel{}
                        PalettePanel{}
                        EditPanel{}
                        CanvasPanel{}
                        InspectorPanel{}
                    }
                }
            }
        }
    }
}
