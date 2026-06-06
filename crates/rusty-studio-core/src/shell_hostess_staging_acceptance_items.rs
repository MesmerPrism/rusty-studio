use super::*;

#[derive(Clone, Debug)]
pub(crate) struct StagingAcceptanceItemSpec {
    pub(crate) item_id: &'static str,
    pub(crate) owner: &'static str,
    pub(crate) item_kind: &'static str,
    pub(crate) route_kind: &'static str,
    pub(crate) source: &'static str,
    pub(crate) evidence: String,
    pub(crate) next_required_action: &'static str,
    pub(crate) prohibited_in_studio: bool,
    pub(crate) expected_input_path: Option<String>,
}

pub(crate) fn shell_hostess_staging_acceptance_item_specs(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    handoff_path: Option<&Path>,
) -> Vec<StagingAcceptanceItemSpec> {
    let handoff_path = handoff_path.map(|path| path.display().to_string());
    let file_plan_path = handoff.file_plan_path.clone();
    vec![
        StagingAcceptanceItemSpec {
            item_id: "hostess.accept_staging_handoff",
            owner: "rusty.hostess",
            item_kind: "hostess_acceptance_gate",
            route_kind: "hostess.accept.staging_handoff",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "handoff envelope {} is ready for Hostess acceptance",
                handoff.envelope_id
            ),
            next_required_action: "accept_or_reject_handoff_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.verify_staging_file_plan_checksum",
            owner: "rusty.hostess",
            item_kind: "hostess_checksum_gate",
            route_kind: "hostess.verify.staging_file_plan_checksum",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} checksum {}",
                handoff.provenance.checksum_algorithm, handoff.provenance.plan_checksum
            ),
            next_required_action: "verify_file_plan_checksum_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.review_staging_file_requests",
            owner: "rusty.hostess",
            item_kind: "hostess_file_plan_review_gate",
            route_kind: "hostess.review.staging_file_requests",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} ready requests over {} planned files",
                handoff.ready_request_count, handoff.planned_file_count
            ),
            next_required_action: "review_shared_and_target_requests_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.copy_staging_files",
            owner: "rusty.hostess",
            item_kind: "hostess_file_copy_request",
            route_kind: "hostess.stage.files_from_plan",
            source: "hostess_staging_file_plan",
            evidence: "file copy remains an external Hostess action".to_string(),
            next_required_action: "copy_stage_files_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "manifold.review_command_session_contract",
            owner: "rusty.manifold",
            item_kind: "manifold_contract_review",
            route_kind: "manifold.review.command_session_contract",
            source: "hostess_staging_handoff_envelope",
            evidence: "Manifold remains command/session authority".to_string(),
            next_required_action: "review_command_session_contract_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.collect_install_launch_evidence",
            owner: "rusty.hostess",
            item_kind: "hostess_evidence_collection_request",
            route_kind: "hostess.collect.install_launch_evidence",
            source: "hostess_staging_handoff_envelope",
            evidence: "install/launch evidence remains an external Hostess action".to_string(),
            next_required_action: "collect_install_launch_evidence_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path,
        },
    ]
}

pub(crate) fn shell_hostess_staging_acceptance_entries(
    specs: Vec<StagingAcceptanceItemSpec>,
    checklist_status: StudioShellHostessStagingAcceptanceStatus,
    checklist_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingAcceptanceChecklistEntry> {
    specs
        .into_iter()
        .map(|spec| {
            let status = if checklist_status == StudioShellHostessStagingAcceptanceStatus::Ready {
                StudioShellHostessStagingAcceptanceStatus::Ready
            } else {
                StudioShellHostessStagingAcceptanceStatus::Blocked
            };
            StudioShellHostessStagingAcceptanceChecklistEntry {
                item_id: spec.item_id.to_string(),
                owner: spec.owner.to_string(),
                status,
                issue_code: (status != StudioShellHostessStagingAcceptanceStatus::Ready).then(
                    || {
                        checklist_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_acceptance_blocked")
                            .to_string()
                    },
                ),
                item_kind: spec.item_kind.to_string(),
                route_kind: spec.route_kind.to_string(),
                source: spec.source.to_string(),
                evidence: spec.evidence,
                next_required_action: spec.next_required_action.to_string(),
                prohibited_in_studio: spec.prohibited_in_studio,
                expected_input_path: spec.expected_input_path,
            }
        })
        .collect()
}
