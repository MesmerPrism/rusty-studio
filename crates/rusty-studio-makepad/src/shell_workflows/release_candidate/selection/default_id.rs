use super::super::super::*;

pub(crate) fn next_shell_release_candidate_default_id(
    index: &StudioShellReleaseCandidateReviewIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Shell release candidate index has no selectable entries".to_string());
    }
    let default_position = index
        .default_candidate_id
        .as_deref()
        .and_then(|default_id| {
            index
                .entries
                .iter()
                .position(|entry| entry.candidate_id == default_id)
        });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].candidate_id.clone())
}
