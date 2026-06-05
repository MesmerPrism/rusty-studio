use super::super::super::super::*;

pub(crate) fn next_shell_hostess_staging_acceptance_default_id(
    index: &StudioShellHostessStagingAcceptanceIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Shell Hostess staging acceptance index has no selectable entries".to_string());
    }
    let default_position = index
        .default_acceptance_id
        .as_deref()
        .and_then(|default_id| {
            index
                .entries
                .iter()
                .position(|entry| entry.acceptance_id == default_id)
        });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].acceptance_id.clone())
}
