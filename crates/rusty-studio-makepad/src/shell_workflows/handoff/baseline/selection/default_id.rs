use super::super::super::super::*;

pub(crate) fn next_shell_handoff_acceptance_baseline_default_id(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Baseline acceptance index has no selectable entries".to_string());
    }
    let default_position = index.default_baseline_id.as_deref().and_then(|default_id| {
        index
            .entries
            .iter()
            .position(|entry| entry.baseline_id == default_id)
    });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].baseline_id.clone())
}
