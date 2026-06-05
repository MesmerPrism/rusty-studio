mod canvas;
mod editor;
mod project;
mod shell_preview;

use super::super::*;

pub(crate) fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    project::script_mod(vm);
    shell_preview::script_mod(vm);
    editor::script_mod(vm);
    canvas::script_mod(vm)
}
