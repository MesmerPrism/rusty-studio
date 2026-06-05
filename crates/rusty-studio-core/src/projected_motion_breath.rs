use super::*;

pub(crate) const PROJECTED_MOTION_BREATH_PACKAGE_ID: &str = "package.projected_motion_breath";
pub(crate) const PROJECTED_MOTION_BREATH_MODULE_ID: &str = "module.breath.projected_motion";
pub(crate) const PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CHECK_SUFFIX: &str =
    "projected_motion_adapter_normalization";
pub(crate) const PROJECTED_MOTION_BREATH_REQUIRED_CHECK_SUFFIXES: [&str; 3] = [
    "projected_motion_contract",
    "projected_motion_profile_commands",
    "projected_motion_goldens",
];

pub fn package_evidence_intake_for_validation_report(
    report: &StudioManifoldPackageValidationReport,
    report_path: Option<&Path>,
    target_package_id: &str,
) -> StudioPackageEvidenceIntakeReport {
    let package_prefix = format!("validation.package.{target_package_id}.");
    let required_check_ids = projected_motion_breath_required_check_ids(target_package_id);
    let required_check_id_set = required_check_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<String>>();
    let target_package_checks = report
        .checks
        .iter()
        .filter(|check| check.check_id.starts_with(&package_prefix))
        .collect::<Vec<_>>();
    let target_package_check_count = target_package_checks.len();
    let target_package_supported = target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID;

    let mut entries = Vec::new();
    for required_check_id in &required_check_ids {
        if let Some(check) = target_package_checks
            .iter()
            .find(|check| check.check_id == *required_check_id)
        {
            entries.push(package_evidence_intake_entry(check, true));
        } else {
            entries.push(missing_package_evidence_intake_entry(required_check_id));
        }
    }
    for check in target_package_checks {
        if !required_check_id_set.contains(&check.check_id) {
            entries.push(package_evidence_intake_entry(check, false));
        }
    }

    let ready_required_check_count = entries
        .iter()
        .filter(|entry| {
            entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
        })
        .count();
    let blocked_required_check_count = required_check_ids.len() - ready_required_check_count;
    let observed_check_count = entries
        .iter()
        .filter(|entry| !entry.required_for_studio)
        .count();
    let failed_target_check_ids = entries
        .iter()
        .filter(|entry| entry.source_status == StudioValidationStatus::Fail)
        .map(|entry| entry.check_id.clone())
        .collect::<Vec<_>>();
    let missing_required_check_ids = entries
        .iter()
        .filter(|entry| {
            entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck
        })
        .map(|entry| entry.check_id.clone())
        .collect::<Vec<_>>();

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_schema",
        report.schema_id == MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA,
        "source Manifold package validation report schema is supported",
        "source Manifold package validation report schema is unsupported",
        "studio.issue.package_evidence_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_path",
        report_path.is_some(),
        "source Manifold package validation report has a durable path",
        "source Manifold package validation report path is missing",
        "studio.issue.package_evidence_source_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_status",
        report.status == StudioValidationStatus::Pass,
        "source Manifold package validation report passed",
        "source Manifold package validation report failed",
        "studio.issue.package_evidence_source_report_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_id",
        is_dotted_id(target_package_id),
        "target package id uses dotted-id grammar",
        "target package id is not a dotted id",
        "studio.issue.package_evidence_target_package_id",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_supported",
        target_package_supported,
        "target package is supported by this Studio intake",
        "target package is not supported by this Studio intake",
        "studio.issue.package_evidence_target_package_unsupported",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_present",
        target_package_check_count > 0,
        "source report contains target package checks",
        "source report does not contain target package checks",
        "studio.issue.package_evidence_target_package_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.required_checks_present",
        missing_required_check_ids.is_empty(),
        "source report contains all required projected-motion breath checks",
        &format!(
            "source report is missing required checks: {}",
            missing_required_check_ids.join(", ")
        ),
        "studio.issue.package_evidence_required_check_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.required_checks_pass",
        blocked_required_check_count == 0,
        "all required projected-motion breath checks pass",
        "one or more required projected-motion breath checks are blocked",
        "studio.issue.package_evidence_required_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_checks_pass",
        failed_target_check_ids.is_empty(),
        "all target package checks visible to Studio pass",
        &format!(
            "target package checks failed: {}",
            failed_target_check_ids.join(", ")
        ),
        "studio.issue.package_evidence_target_package_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.execution_policy",
        true,
        "Studio package evidence intake is review-only and not executed",
        "Studio package evidence intake attempted execution",
        "studio.issue.package_evidence_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.package_evidence_source_schema")
                    | Some("studio.issue.package_evidence_target_package_id")
                    | Some("studio.issue.package_evidence_target_package_unsupported")
            )
    });
    let status = if has_rejected_check {
        StudioPackageEvidenceIntakeStatus::Rejected
    } else if has_failed_check {
        StudioPackageEvidenceIntakeStatus::Blocked
    } else {
        StudioPackageEvidenceIntakeStatus::Ready
    };
    let issue_code = match status {
        StudioPackageEvidenceIntakeStatus::Ready => None,
        StudioPackageEvidenceIntakeStatus::Blocked
        | StudioPackageEvidenceIntakeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioPackageEvidenceIntakeReport {
        schema_id: PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA.to_string(),
        source_report_schema: report.schema_id.clone(),
        source_report_path: report_path.map(|path| path.display().to_string()),
        target_package_id: target_package_id.to_string(),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_report_status: report.status,
        source_check_count: report.checks.len(),
        target_package_check_count,
        required_check_count: required_check_ids.len(),
        ready_required_check_count,
        blocked_required_check_count,
        observed_check_count,
        entries,
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_required_check_ids(target_package_id: &str) -> Vec<String> {
    PROJECTED_MOTION_BREATH_REQUIRED_CHECK_SUFFIXES
        .iter()
        .map(|suffix| format!("validation.package.{target_package_id}.{suffix}"))
        .collect()
}

fn package_evidence_intake_entry(
    check: &StudioManifoldPackageValidationCheck,
    required_for_studio: bool,
) -> StudioPackageEvidenceIntakeEntry {
    let decision = match (check.status, required_for_studio) {
        (StudioValidationStatus::Pass, true) => StudioPackageEvidenceIntakeDecision::Ready,
        (StudioValidationStatus::Pass, false) => StudioPackageEvidenceIntakeDecision::Observed,
        (StudioValidationStatus::Fail, _) => {
            StudioPackageEvidenceIntakeDecision::BlockedByFailedCheck
        }
    };
    StudioPackageEvidenceIntakeEntry {
        check_id: check.check_id.clone(),
        source_status: check.status,
        evidence: check.evidence.clone(),
        required_for_studio,
        decision,
        next_required_action: package_evidence_next_action(decision).to_string(),
        issue_code: (check.status == StudioValidationStatus::Fail)
            .then(|| "studio.issue.package_evidence_source_check_failed".to_string()),
    }
}

fn missing_package_evidence_intake_entry(check_id: &str) -> StudioPackageEvidenceIntakeEntry {
    StudioPackageEvidenceIntakeEntry {
        check_id: check_id.to_string(),
        source_status: StudioValidationStatus::Fail,
        evidence: "required source check missing".to_string(),
        required_for_studio: true,
        decision: StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck,
        next_required_action: package_evidence_next_action(
            StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck,
        )
        .to_string(),
        issue_code: Some("studio.issue.package_evidence_required_check_missing".to_string()),
    }
}

fn package_evidence_next_action(decision: StudioPackageEvidenceIntakeDecision) -> &'static str {
    match decision {
        StudioPackageEvidenceIntakeDecision::Ready => "review_package_in_studio",
        StudioPackageEvidenceIntakeDecision::Observed => "observe_nonblocking_package_evidence",
        StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck => {
            "rerun_manifold_package_validation"
        }
        StudioPackageEvidenceIntakeDecision::BlockedByFailedCheck => {
            "repair_manifold_package_evidence"
        }
    }
}

fn package_evidence_intake_prohibited_actions() -> Vec<String> {
    [
        "build",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "start_runtime_package",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

pub fn projected_motion_breath_shell_handoff_review_for_evidence(
    evidence: &Value,
    evidence_path: Option<&Path>,
) -> StudioProjectedMotionBreathShellHandoffReviewReport {
    let source_evidence_schema = json_string(evidence, "$schema");
    let target_package_id = nested_json_string(evidence, "package", "package_id");
    let handoff_id = nested_json_string(evidence, "shell_handoff", "handoff_id");
    let target_host_profile = nested_json_string(evidence, "shell_handoff", "target_host_profile");
    let shell_app_id = nested_json_string(evidence, "shell_handoff", "shell_app_id");
    let command_ids = nested_json_string_array(evidence, "shell_handoff", "command_ids");
    let exported_stream_ids =
        nested_json_string_array(evidence, "package_contract", "exported_stream_ids");
    let feedback_sink_streams = nested_json_string_array(
        evidence,
        "package_contract",
        "feedback_sink_provides_streams",
    );
    let binding_pairs = projected_motion_breath_shell_binding_pairs(evidence);
    let required_bindings = projected_motion_breath_shell_required_bindings();
    let ready_required_binding_count = required_bindings
        .iter()
        .filter(|binding| binding_pairs.contains(*binding))
        .count();
    let transport_ids = projected_motion_breath_shell_transport_ids(evidence);
    let runtime_execution_performed =
        nested_json_bool(evidence, "execution", "runtime_execution_performed").unwrap_or(true);
    let platform_execution_performed =
        nested_json_bool(evidence, "execution", "platform_execution_performed").unwrap_or(true);
    let broker_transport_used =
        nested_json_bool(evidence, "execution", "broker_transport_used").unwrap_or(true);
    let downstream_shell_runtime_used =
        nested_json_bool(evidence, "execution", "downstream_shell_runtime_used").unwrap_or(true);
    let legacy_app_dependency_used =
        nested_json_bool(evidence, "execution", "legacy_app_dependency_used").unwrap_or(true);
    let legacy_rusty_xr_repo_used =
        nested_json_bool(evidence, "execution", "legacy_rusty_xr_repo_used").unwrap_or(true);
    let feedback_receipt_exported = exported_stream_ids
        .iter()
        .any(|stream_id| stream_id == "stream.breath.feedback_receipt");
    let feedback_sink_provides_receipt = feedback_sink_streams
        .iter()
        .any(|stream_id| stream_id == "stream.breath.feedback_receipt");
    let clean_execution_boundary = !runtime_execution_performed
        && !platform_execution_performed
        && !broker_transport_used
        && !downstream_shell_runtime_used
        && !legacy_app_dependency_used
        && !legacy_rusty_xr_repo_used;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_schema",
        source_evidence_schema.as_deref()
            == Some("rusty.hostess.projected_motion_breath.shell_handoff_validation_evidence.v1"),
        "source Hostess shell handoff evidence schema is supported",
        "source Hostess shell handoff evidence schema is unsupported",
        "studio.issue.projected_motion_breath_shell_handoff_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_path",
        evidence_path.is_some(),
        "source Hostess shell handoff evidence has a durable path",
        "source Hostess shell handoff evidence path is missing",
        "studio.issue.projected_motion_breath_shell_handoff_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_status",
        evidence.get("status").and_then(Value::as_str) == Some("pass")
            && evidence
                .get("scorecard")
                .and_then(|scorecard| scorecard.get("status"))
                .and_then(Value::as_str)
                == Some("pass"),
        "source Hostess shell handoff evidence and scorecard passed",
        "source Hostess shell handoff evidence or scorecard failed",
        "studio.issue.projected_motion_breath_shell_handoff_source_failed",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.target_package",
        target_package_id.as_deref() == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID),
        "source evidence targets projected-motion breath",
        "source evidence targets a different package",
        "studio.issue.projected_motion_breath_shell_handoff_package_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.handoff_id",
        handoff_id.as_deref().is_some_and(is_dotted_id),
        "source evidence declares a dotted shell handoff id",
        "source evidence is missing a dotted shell handoff id",
        "studio.issue.projected_motion_breath_shell_handoff_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.required_bindings",
        ready_required_binding_count == required_bindings.len(),
        "source evidence includes controller pose publish, feedback subscribe, and receipt publish bindings",
        "source evidence is missing one or more required PMB shell bindings",
        "studio.issue.projected_motion_breath_shell_handoff_required_bindings",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.feedback_receipt_export",
        feedback_receipt_exported && feedback_sink_provides_receipt,
        "source evidence proves feedback receipt export and feedback sink provisioning",
        "source evidence does not prove feedback receipt export and feedback sink provisioning",
        "studio.issue.projected_motion_breath_shell_handoff_feedback_receipt",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.command_status",
        command_ids
            .iter()
            .any(|command_id| command_id == "command.breath.status"),
        "source evidence exposes command.breath.status for read-only handoff checks",
        "source evidence does not expose command.breath.status",
        "studio.issue.projected_motion_breath_shell_handoff_command_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.transport_offer",
        !transport_ids.is_empty(),
        "source evidence includes a named transport offer for downstream shell wiring",
        "source evidence does not include a named transport offer",
        "studio.issue.projected_motion_breath_shell_handoff_transport_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.authority_boundary",
        clean_execution_boundary,
        "Studio review preserves Hostess runtime evidence ownership and avoids shell execution",
        "source evidence indicates runtime, transport, downstream shell, or legacy repo execution",
        "studio.issue.projected_motion_breath_shell_handoff_authority_mismatch",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_shell_handoff_source_schema")
                    | Some("studio.issue.projected_motion_breath_shell_handoff_package_mismatch")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Ready => None,
        StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
        | StudioProjectedMotionBreathShellHandoffReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathShellHandoffReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA.to_string(),
        source_evidence_schema,
        source_evidence_path: evidence_path.map(|path| path.display().to_string()),
        target_package_id,
        handoff_id,
        target_host_profile,
        shell_app_id,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed,
        platform_execution_performed,
        broker_transport_used,
        downstream_shell_runtime_used,
        legacy_app_dependency_used,
        required_binding_count: required_bindings.len(),
        ready_required_binding_count,
        stream_bindings: binding_pairs
            .iter()
            .map(|(stream_id, direction)| format!("{stream_id}:{direction}"))
            .collect(),
        command_ids,
        transport_ids,
        feedback_receipt_exported,
        feedback_sink_provides_receipt,
        proposal_kind: "review_shell_handoff_for_hostess_owner_execution".to_string(),
        prohibited_actions: projected_motion_breath_shell_handoff_review_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_shell_required_bindings() -> BTreeSet<(String, String)> {
    [
        ("stream.motion.object_pose", "publish"),
        ("stream.breath.feedback_state", "subscribe"),
        ("stream.breath.feedback_receipt", "publish"),
    ]
    .iter()
    .map(|(stream_id, direction)| (stream_id.to_string(), direction.to_string()))
    .collect()
}

fn projected_motion_breath_shell_binding_pairs(evidence: &Value) -> BTreeSet<(String, String)> {
    let mut bindings = BTreeSet::new();
    if let Some(shell_handoff) = evidence.get("shell_handoff") {
        for field in ["binding_pairs", "stream_bindings"] {
            if let Some(values) = shell_handoff.get(field).and_then(Value::as_array) {
                for value in values {
                    if let (Some(stream_id), Some(direction)) = (
                        value.get("stream_id").and_then(Value::as_str),
                        value.get("direction").and_then(Value::as_str),
                    ) {
                        bindings.insert((stream_id.to_string(), direction.to_string()));
                    }
                }
            }
        }
    }
    bindings
}

fn projected_motion_breath_shell_transport_ids(evidence: &Value) -> Vec<String> {
    evidence
        .get("shell_handoff")
        .and_then(|shell_handoff| shell_handoff.get("transport_offers"))
        .and_then(Value::as_array)
        .map(|offers| {
            offers
                .iter()
                .filter_map(|offer| offer.get("transport_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn projected_motion_breath_shell_handoff_review_prohibited_actions() -> Vec<String> {
    [
        "build",
        "install",
        "launch",
        "stage_shell_files",
        "launch_downstream_shell",
        "open_command_session",
        "collect_device_evidence",
        "start_runtime_package",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

pub fn projected_motion_breath_authoring_review_for_intake(
    intake: &StudioPackageEvidenceIntakeReport,
    intake_path: Option<&Path>,
    profile: &Value,
    profile_path: Option<&Path>,
) -> StudioProjectedMotionBreathAuthoringReviewReport {
    let required_package_checks =
        projected_motion_breath_required_check_ids(PROJECTED_MOTION_BREATH_PACKAGE_ID);
    let source_profile_schema = json_string(profile, "$schema");
    let profile_id = json_string(profile, "profile_id");
    let target_module_id = json_string(profile, "target_module_id");
    let input_kinds = json_string_array(profile, "input_kinds");
    let projection_mode = nested_json_string(profile, "projection", "mode");
    let fallback_projection_mode = nested_json_string(profile, "projection", "fallback_mode");
    let required_package_checks_ready = required_package_checks.iter().all(|required_check_id| {
        intake.entries.iter().any(|entry| {
            entry.check_id == *required_check_id
                && entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
        })
    });
    let authority_preserved = intake.runtime_authority == "rusty.manifold"
        && intake.authoring_authority == "rusty.studio"
        && intake.platform_validation_authority == "rusty.hostess"
        && !intake.runtime_execution_performed
        && !intake.platform_execution_performed;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.source_intake_schema",
        intake.schema_id == PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA,
        "source package evidence intake schema is supported",
        "source package evidence intake schema is unsupported",
        "studio.issue.package_evidence_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.source_intake_path",
        intake_path.is_some(),
        "source package evidence intake has a durable path",
        "source package evidence intake path is missing",
        "studio.issue.projected_motion_breath_intake_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.package_evidence_ready",
        intake.status == StudioPackageEvidenceIntakeStatus::Ready,
        "source package evidence intake is ready",
        "source package evidence intake is blocked or rejected",
        intake
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.package_evidence_intake_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.target_package",
        intake.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID,
        "source package evidence targets projected-motion breath",
        "source package evidence targets a different package",
        "studio.issue.projected_motion_breath_package_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.required_package_checks_ready",
        required_package_checks_ready,
        "all projected-motion breath package evidence checks are ready",
        "one or more projected-motion breath package evidence checks are not ready",
        "studio.issue.projected_motion_breath_package_evidence_not_ready",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.authority_boundary",
        authority_preserved,
        "Studio review preserves Manifold, Studio, and Hostess authorities",
        "Studio review authority boundary is not preserved",
        "studio.issue.projected_motion_breath_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_path",
        profile_path.is_some(),
        "source motion-breath profile has a durable path",
        "source motion-breath profile path is missing",
        "studio.issue.projected_motion_breath_profile_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_schema",
        source_profile_schema.as_deref() == Some(MOTION_BREATH_PROFILE_SCHEMA),
        "source profile schema is supported",
        "source profile schema is unsupported",
        "studio.issue.motion_breath_profile_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_id",
        profile_id.as_deref().is_some_and(is_dotted_id),
        "profile id uses dotted-id grammar",
        "profile id is missing or not a dotted id",
        "studio.issue.motion_breath_profile_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_target_module",
        target_module_id.as_deref() == Some("module.breath.projected_motion"),
        "profile targets the projected-motion breath module",
        "profile does not target the projected-motion breath module",
        "studio.issue.motion_breath_profile_target_module",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_input_kinds",
        ["pose", "vector3"]
            .iter()
            .all(|required| input_kinds.iter().any(|kind| kind == required)),
        "profile declares pose and vector3 input kinds",
        "profile does not declare both pose and vector3 input kinds",
        "studio.issue.motion_breath_profile_input_kinds",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_projection",
        projection_mode.is_some(),
        "profile declares a projection mode for review",
        "profile does not declare a projection mode",
        "studio.issue.motion_breath_profile_projection",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.execution_policy",
        true,
        "Studio projected-motion breath authoring review is proposal-only and not executed",
        "Studio projected-motion breath authoring review attempted execution",
        "studio.issue.projected_motion_breath_authoring_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.package_evidence_intake_schema")
                    | Some("studio.issue.motion_breath_profile_schema")
                    | Some("studio.issue.projected_motion_breath_package_mismatch")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathAuthoringReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathAuthoringReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathAuthoringReviewStatus::Ready => None,
        StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
        | StudioProjectedMotionBreathAuthoringReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathAuthoringReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        source_intake_path: intake_path.map(|path| path.display().to_string()),
        source_profile_schema,
        source_profile_path: profile_path.map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id,
        profile_id,
        status,
        issue_code,
        execution_policy: "not_executed.proposal_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        package_evidence_status: intake.status,
        package_required_check_count: intake.required_check_count,
        package_ready_required_check_count: intake.ready_required_check_count,
        package_blocked_required_check_count: intake.blocked_required_check_count,
        input_kinds,
        projection_mode,
        fallback_projection_mode,
        proposed_command_id: "command.breath.set_profile".to_string(),
        proposed_profile_operation: "propose_profile_for_runtime_owner_review".to_string(),
        required_package_checks,
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

pub fn projected_motion_breath_source_adapter_selection_review_for_authoring(
    authoring_review: &StudioProjectedMotionBreathAuthoringReviewReport,
    authoring_review_path: Option<&Path>,
    source_descriptors: &Value,
    source_descriptors_path: Option<&Path>,
    selected_adapter_id: &str,
) -> StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
    let descriptors = source_adapter_descriptor_array(source_descriptors);
    let selected_descriptor =
        find_source_adapter_descriptor(source_descriptors, selected_adapter_id);
    let selected_source_kind =
        selected_descriptor.and_then(|value| json_string(value, "source_kind"));
    let selected_input_kind =
        selected_descriptor.and_then(|value| json_string(value, "input_kind"));
    let selected_output_stream_id =
        selected_descriptor.and_then(|value| json_string(value, "output_stream_id"));
    let descriptor_schema = json_string(source_descriptors, "$schema");
    let descriptor_target_module = json_string(source_descriptors, "target_module_id");
    let selected_input_supported = selected_input_kind.as_ref().is_some_and(|kind| {
        authoring_review
            .input_kinds
            .iter()
            .any(|input| input == kind)
    });
    let selected_stream_supported = matches!(
        (
            selected_input_kind.as_deref(),
            selected_output_stream_id.as_deref()
        ),
        (Some("pose"), Some("stream.motion.object_pose"))
            | (Some("vector3"), Some("stream.motion.vector3"))
    );
    let descriptor_source_clean = source_descriptors
        .get("runtime_execution_performed")
        .and_then(Value::as_bool)
        == Some(false)
        && source_descriptors
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_descriptors
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);
    let selected_descriptor_clean = selected_descriptor.is_some_and(|descriptor| {
        descriptor.get("transport_kind").and_then(Value::as_str) == Some("descriptor_only")
            && descriptor
                .get("requires_platform_sdk")
                .and_then(Value::as_bool)
                == Some(false)
            && descriptor
                .get("requires_device_api")
                .and_then(Value::as_bool)
                == Some(false)
            && descriptor
                .get("runtime_adapter_included")
                .and_then(Value::as_bool)
                == Some(false)
    });
    let authority_preserved = authoring_review.runtime_authority == "rusty.manifold"
        && authoring_review.authoring_authority == "rusty.studio"
        && authoring_review.platform_validation_authority == "rusty.hostess"
        && !authoring_review.runtime_execution_performed
        && !authoring_review.platform_execution_performed;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.authoring_schema",
        authoring_review.schema_id == PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA,
        "source authoring review schema is supported",
        "source authoring review schema is unsupported",
        "studio.issue.projected_motion_breath_authoring_review_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.authoring_ready",
        authoring_review.status == StudioProjectedMotionBreathAuthoringReviewStatus::Ready,
        "source authoring review is ready",
        "source authoring review is blocked or rejected",
        authoring_review
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_authoring_review_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.descriptor_schema",
        descriptor_schema.as_deref()
            == Some(PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA),
        "source adapter descriptor schema is supported",
        "source adapter descriptor schema is unsupported",
        "studio.issue.projected_motion_breath_source_adapter_descriptor_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.target_contract",
        source_descriptors.get("package_id").and_then(Value::as_str)
            == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
            && descriptor_target_module.as_deref() == Some("module.breath.projected_motion")
            && authoring_review.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID
            && authoring_review.target_module_id.as_deref()
                == Some("module.breath.projected_motion"),
        "source adapter descriptors target projected-motion breath",
        "source adapter descriptor target package or module drifted",
        "studio.issue.projected_motion_breath_source_adapter_target",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.selected_adapter_id",
        is_dotted_id(selected_adapter_id),
        "selected source adapter id uses dotted-id grammar",
        "selected source adapter id is not a dotted id",
        "studio.issue.projected_motion_breath_source_adapter_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.selected_adapter_present",
        selected_descriptor.is_some(),
        "selected source adapter descriptor is present",
        "selected source adapter descriptor is missing",
        "studio.issue.projected_motion_breath_source_adapter_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.input_kind",
        selected_input_supported,
        "selected source adapter input kind is supported by the profile intent",
        "selected source adapter input kind is not supported by the profile intent",
        "studio.issue.projected_motion_breath_source_adapter_input_kind",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.stream_binding",
        selected_stream_supported,
        "selected source adapter maps to a supported pose/vector stream",
        "selected source adapter stream does not match its input kind",
        "studio.issue.projected_motion_breath_source_adapter_stream",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.non_executing",
        descriptor_source_clean && selected_descriptor_clean && authority_preserved,
        "source adapter selection is descriptor-only and preserves authority boundaries",
        "source adapter selection attempted runtime, platform, device, or authority drift",
        "studio.issue.projected_motion_breath_source_adapter_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_source_adapter_descriptor_schema")
                    | Some("studio.issue.projected_motion_breath_source_adapter_target")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready => None,
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
        | StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA.to_string(),
        source_authoring_review_schema: authoring_review.schema_id.clone(),
        source_authoring_review_path: authoring_review_path.map(|path| path.display().to_string()),
        source_descriptor_schema: descriptor_schema,
        source_descriptor_path: source_descriptors_path.map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id: authoring_review.target_module_id.clone(),
        profile_id: authoring_review.profile_id.clone(),
        selected_adapter_id: selected_adapter_id.to_string(),
        selected_source_kind,
        selected_input_kind,
        selected_output_stream_id,
        status,
        issue_code,
        execution_policy: "not_executed.proposal_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_authoring_review_status: authoring_review.status,
        source_descriptor_count: descriptors.len(),
        matching_descriptor_count: if selected_descriptor.is_some() { 1 } else { 0 },
        proposal_kind: "propose_source_adapter_for_runtime_owner_review".to_string(),
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

pub fn projected_motion_breath_adapter_normalization_evidence_review_for_selection(
    selection_review: &StudioProjectedMotionBreathSourceAdapterSelectionReviewReport,
    selection_review_path: Option<&Path>,
    package_report: &StudioManifoldPackageValidationReport,
    package_report_path: Option<&Path>,
    source_binding: &Value,
    source_binding_path: Option<&Path>,
    normalization_case: &Value,
    normalization_case_path: Option<&Path>,
) -> StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport {
    let adapter_normalization_check_id = projected_motion_breath_adapter_normalization_check_id();
    let adapter_normalization_check = package_report
        .checks
        .iter()
        .find(|check| check.check_id == adapter_normalization_check_id);
    let adapter_normalization_check_status = adapter_normalization_check.map(|check| check.status);
    let source_binding_schema = json_string(source_binding, "$schema");
    let source_normalization_case_schema = json_string(normalization_case, "$schema");
    let binding_id = json_string(source_binding, "binding_id");
    let normalization_case_id = json_string(normalization_case, "case_id");
    let source_payload_kind = json_string(normalization_case, "source_payload_kind");
    let expected_sample_kind = json_string(normalization_case, "expected_sample_kind");
    let binding_selected_adapter_id = json_string(source_binding, "selected_adapter_id");
    let binding_selected_input_kind = json_string(source_binding, "selected_input_kind");
    let binding_selected_output_stream_id =
        json_string(source_binding, "selected_output_stream_id");
    let normalization_binding_path = json_string(normalization_case, "binding_path");

    let source_binding_selected_adapter_match = binding_selected_adapter_id.as_deref()
        == Some(selection_review.selected_adapter_id.as_str());
    let source_binding_stream_match = binding_selected_input_kind
        == selection_review.selected_input_kind
        && binding_selected_output_stream_id == selection_review.selected_output_stream_id;
    let source_binding_target_match = source_binding.get("package_id").and_then(Value::as_str)
        == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
        && source_binding
            .get("target_module_id")
            .and_then(Value::as_str)
            == Some(PROJECTED_MOTION_BREATH_MODULE_ID)
        && source_binding.get("profile_id").and_then(Value::as_str)
            == selection_review.profile_id.as_deref();
    let source_binding_path_match = source_binding_path.is_some_and(|path| {
        normalization_binding_path
            .as_deref()
            .is_some_and(|binding_path| path_matches_reference_suffix(path, binding_path))
    });
    let normalization_payload_matches = adapter_normalization_payload_matches(
        selection_review.selected_source_kind.as_deref(),
        source_payload_kind.as_deref(),
        expected_sample_kind.as_deref(),
    );
    let deterministic_normalization_evidence = source_binding_path_match
        && normalization_payload_matches
        && source_binding_selected_adapter_match
        && source_binding_stream_match;
    let selection_authority_preserved = selection_review.runtime_authority == "rusty.manifold"
        && selection_review.authoring_authority == "rusty.studio"
        && selection_review.platform_validation_authority == "rusty.hostess"
        && !selection_review.runtime_execution_performed
        && !selection_review.platform_execution_performed;
    let source_binding_clean = source_binding
        .get("execution_policy")
        .and_then(Value::as_str)
        == Some("not_executed.schema_binding_only")
        && source_binding
            .get("runtime_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_binding
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_binding
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);
    let normalization_case_clean = normalization_case
        .get("execution_policy")
        .and_then(Value::as_str)
        == Some("not_executed.fixture_normalization_only")
        && normalization_case
            .get("runtime_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && normalization_case
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && normalization_case
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.selection_schema",
        selection_review.schema_id
            == PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA,
        "source adapter selection review schema is supported",
        "source adapter selection review schema is unsupported",
        "studio.issue.projected_motion_breath_source_adapter_selection_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.selection_ready",
        selection_review.status
            == StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready,
        "source adapter selection review is ready",
        "source adapter selection review is blocked or rejected",
        selection_review
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_source_adapter_selection_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_report_schema",
        package_report.schema_id == MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA,
        "source Manifold package validation report schema is supported",
        "source Manifold package validation report schema is unsupported",
        "studio.issue.package_evidence_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_check_present",
        adapter_normalization_check.is_some(),
        "source package report includes adapter-normalization evidence",
        "source package report is missing adapter-normalization evidence",
        "studio.issue.projected_motion_breath_adapter_normalization_check_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_check_pass",
        adapter_normalization_check_status == Some(StudioValidationStatus::Pass),
        "source package adapter-normalization evidence passed",
        "source package adapter-normalization evidence did not pass",
        "studio.issue.projected_motion_breath_adapter_normalization_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_schema",
        source_binding_schema.as_deref() == Some(PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA),
        "source binding schema is supported",
        "source binding schema is unsupported",
        "studio.issue.projected_motion_breath_source_binding_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_target",
        source_binding_target_match
            && selection_review.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID
            && selection_review.target_module_id.as_deref()
                == Some(PROJECTED_MOTION_BREATH_MODULE_ID),
        "source binding targets the selected projected-motion breath contract",
        "source binding target package, module, or profile drifted",
        "studio.issue.projected_motion_breath_source_binding_target",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_adapter",
        source_binding_selected_adapter_match,
        "source binding selected adapter matches Studio selection",
        "source binding selected adapter differs from Studio selection",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_adapter",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_stream",
        source_binding_stream_match,
        "source binding stream matches selected input kind and output stream",
        "source binding stream differs from selected input kind or output stream",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_stream",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.case_schema",
        source_normalization_case_schema.as_deref()
            == Some(PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA),
        "adapter-normalization case schema is supported",
        "adapter-normalization case schema is unsupported",
        "studio.issue.projected_motion_breath_adapter_normalization_case_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.case_binding_path",
        source_binding_path_match,
        "adapter-normalization case points at the selected source binding",
        "adapter-normalization case does not point at the selected source binding",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_path",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.payload_kind",
        normalization_payload_matches,
        "adapter-normalization case payload kind matches the selected source kind",
        "adapter-normalization case payload kind does not match the selected source kind",
        "studio.issue.projected_motion_breath_adapter_normalization_payload_kind",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.non_executing",
        selection_authority_preserved && source_binding_clean && normalization_case_clean,
        "adapter-normalization evidence is schema-only and preserves authority boundaries",
        "adapter-normalization evidence attempted runtime, platform, device, or authority drift",
        "studio.issue.projected_motion_breath_adapter_normalization_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check =
        checks.iter().any(|check| {
            check.status == StudioValidationStatus::Fail
                && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_source_adapter_selection_schema")
                    | Some("studio.issue.package_evidence_source_schema")
                    | Some("studio.issue.projected_motion_breath_source_binding_schema")
                    | Some("studio.issue.projected_motion_breath_source_binding_target")
                    | Some("studio.issue.projected_motion_breath_adapter_normalization_case_schema")
            )
        });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready => None,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
        | StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA.to_string(),
        source_selection_review_schema: selection_review.schema_id.clone(),
        source_selection_review_path: selection_review_path.map(|path| path.display().to_string()),
        source_package_report_schema: package_report.schema_id.clone(),
        source_package_report_path: package_report_path.map(|path| path.display().to_string()),
        source_binding_schema,
        source_binding_path: source_binding_path.map(|path| path.display().to_string()),
        source_normalization_case_schema,
        source_normalization_case_path: normalization_case_path
            .map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id: selection_review.target_module_id.clone(),
        profile_id: selection_review.profile_id.clone(),
        selected_adapter_id: selection_review.selected_adapter_id.clone(),
        selected_source_kind: selection_review.selected_source_kind.clone(),
        selected_input_kind: selection_review.selected_input_kind.clone(),
        selected_output_stream_id: selection_review.selected_output_stream_id.clone(),
        binding_id,
        normalization_case_id,
        source_payload_kind,
        expected_sample_kind,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_selection_status: selection_review.status,
        adapter_normalization_check_id,
        adapter_normalization_check_status,
        source_binding_selected_adapter_match,
        deterministic_normalization_evidence,
        proposal_kind: "review_adapter_normalization_for_runtime_owner".to_string(),
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_adapter_normalization_check_id() -> String {
    format!(
        "validation.package.{PROJECTED_MOTION_BREATH_PACKAGE_ID}.{PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CHECK_SUFFIX}"
    )
}

fn adapter_normalization_payload_matches(
    selected_source_kind: Option<&str>,
    source_payload_kind: Option<&str>,
    expected_sample_kind: Option<&str>,
) -> bool {
    matches!(
        (
            selected_source_kind,
            source_payload_kind,
            expected_sample_kind
        ),
        (
            Some("object_pose"),
            Some("object_pose"),
            Some("rigid_motion")
        ) | (
            Some("xr_controller_pose"),
            Some("object_pose"),
            Some("rigid_motion")
        ) | (
            Some("vector_motion"),
            Some("vector_motion"),
            Some("vector_motion")
        ) | (
            Some("wearable_acceleration"),
            Some("vector_motion"),
            Some("vector_motion")
        ) | (
            Some("external_patch_stream_bridge"),
            Some("external_patch_channels"),
            Some("vector_motion")
        )
    )
}

fn path_matches_reference_suffix(actual_path: &Path, reference_suffix: &str) -> bool {
    let actual = actual_path.display().to_string().replace('\\', "/");
    let expected = reference_suffix.replace('\\', "/");
    actual.ends_with(&expected)
        || actual_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|file_name| expected.ends_with(file_name))
}

fn source_adapter_descriptor_array(document: &Value) -> Vec<&Value> {
    document
        .get("source_adapters")
        .and_then(Value::as_array)
        .map(|values| values.iter().collect())
        .unwrap_or_default()
}

fn find_source_adapter_descriptor<'a>(
    document: &'a Value,
    selected_adapter_id: &str,
) -> Option<&'a Value> {
    source_adapter_descriptor_array(document)
        .into_iter()
        .find(|descriptor| {
            descriptor.get("adapter_id").and_then(Value::as_str) == Some(selected_adapter_id)
        })
}

fn json_string(document: &Value, field: &str) -> Option<String> {
    document
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn nested_json_string(document: &Value, object_field: &str, field: &str) -> Option<String> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn nested_json_bool(document: &Value, object_field: &str, field: &str) -> Option<bool> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_bool)
}

fn json_string_array(document: &Value, field: &str) -> Vec<String> {
    document
        .get(field)
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn nested_json_string_array(document: &Value, object_field: &str, field: &str) -> Vec<String> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}
