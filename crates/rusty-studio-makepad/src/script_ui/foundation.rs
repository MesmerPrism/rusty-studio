mod canvas;
mod widgets;

use super::super::*;

pub(crate) fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    widgets::script_mod(vm);
    canvas::script_mod(vm)
}
