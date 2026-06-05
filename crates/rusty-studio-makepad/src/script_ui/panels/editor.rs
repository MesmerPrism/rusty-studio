use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.PalettePanel = Panel{
        SectionTitle{text: "Reference Palette"}
        ButtonRow{
            add_palette_module_button := ActionButton{text: "Add Module From Package"}
        }
        Row{FieldLabel{text: "packages"} catalog_packages := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "profiles"} host_profiles := SmallValue{text: ""}}
    }

    mod.widgets.EditPanel = Panel{
        SectionTitle{text: "Edit Report"}
        ButtonRow{
            target_desktop_button := ActionButton{text: "Target Desktop"}
            target_headset_button := ActionButton{text: "Target Headset"}
        }
        ButtonRow{
            remove_selected_module_button := ActionButton{text: "Remove Selected Module"}
            add_command_binding_button := ActionButton{text: "Add Command To Selected"}
            remove_selected_binding_button := ActionButton{text: "Remove Selected Binding"}
        }
        Row{FieldLabel{text: "status"} edit_status := FieldValue{text: "no edits requested"}}
        Row{FieldLabel{text: "message"} edit_message := SmallValue{text: ""}}
        Row{FieldLabel{text: "changed"} edit_changed_fields := SmallValue{text: ""}}
        Row{FieldLabel{text: "validation"} edit_validation := SmallValue{text: ""}}
    }
}
