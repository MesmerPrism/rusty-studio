mod foundation;
mod panels;
mod root;

use super::*;

pub(crate) fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    foundation::script_mod(vm);
    panels::script_mod(vm);
    root::script_mod(vm)
}
