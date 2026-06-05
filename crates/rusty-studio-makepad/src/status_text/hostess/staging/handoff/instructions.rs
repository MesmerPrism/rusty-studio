use super::super::super::super::*;

pub(super) fn shell_hostess_staging_handoff_instruction_rows(
    report: &StudioShellHostessStagingHandoffEnvelope,
) -> String {
    report
        .owner_instructions
        .iter()
        .map(|instruction| {
            let instruction_status =
                shell_hostess_staging_handoff_instruction_status_label(instruction.status);
            let issue = instruction.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; kind {}; route {}; next {}; prohibited in Studio {}; issue {}",
                instruction.instruction_id,
                instruction_status,
                instruction.owner,
                instruction.instruction_kind,
                instruction.route_kind,
                instruction.next_required_action,
                if instruction.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
