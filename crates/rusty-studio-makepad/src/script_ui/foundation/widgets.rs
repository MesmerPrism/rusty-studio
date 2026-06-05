use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.PageTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x111827
        draw_text.text_style: theme.font_bold{font_size: 24.0}
    }

    mod.widgets.SectionTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x263238
        draw_text.text_style: theme.font_bold{font_size: 16.0}
    }

    mod.widgets.FieldLabel = Label{
        width: 150.0 height: Fit
        draw_text.color: #x5d6875
        draw_text.text_style.font_size: 12.0
    }

    mod.widgets.FieldValue = Label{
        width: Fill height: Fit
        draw_text.color: #x111827
        draw_text.text_style.font_size: 13.0
    }

    mod.widgets.SmallValue = Label{
        width: Fill height: Fit
        draw_text.color: #x3f4a54
        draw_text.text_style.font_size: 12.0
    }

    mod.widgets.Panel = RoundedView{
        width: Fill height: Fit
        flow: Down
        spacing: 8.0
        padding: 14.0
        draw_bg +: {
            color: #xffffff
            border_color: #xd8dde3
            border_size: 1.0
            border_radius: 8.0
        }
    }

    mod.widgets.Row = View{
        width: Fill height: Fit
        flow: Right
        spacing: 10.0
        align: Align{y: 0.5}
    }

    mod.widgets.ButtonRow = View{
        width: Fill height: Fit
        flow: Right
        spacing: 8.0
        align: Align{y: 0.5}
    }

    mod.widgets.ActionButton = Button{
        width: Fit height: 32.0
        padding: Inset{left: 12.0 right: 12.0 top: 7.0 bottom: 7.0}
        draw_bg +: {
            color: #xeaf0f6
            color_hover: #xdce8f6
            color_down: #xcbd9ea
            color_focus: #xe0edf9
            border_color: #xc7d0dc
            border_color_hover: #xb7c6d8
            border_color_down: #xa8b7ca
            border_color_focus: #x7aa0c8
            border_size: 1.0
            border_radius: 6.0
        }
        draw_text +: {
            color: #x111827
            color_hover: #x111827
            color_down: #x111827
            color_focus: #x111827
            text_style: theme.font_bold{font_size: 12.0}
        }
    }

    mod.widgets.Rule = SolidView{
        width: Fill height: 1.0
        draw_bg.color: #xe7ebef
    }
}
