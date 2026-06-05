use super::*;

fn projected_motion_package_validation_report() -> StudioManifoldPackageValidationReport {
    let mut checks = [
        "dotted_ids",
        "exports",
        "module_links",
        "stream_links",
        "graph_links",
        "deployment_links",
        "runtime_state_links",
        "timestamp_policy",
        "provider_processor_split",
        "command_rejections",
        "scorecards",
        "projected_motion_contract",
        "projected_motion_profile_commands",
        "projected_motion_goldens",
        "projected_motion_source_adapters",
        "projected_motion_source_bindings",
        "projected_motion_adapter_normalization",
    ]
    .iter()
    .map(|suffix| StudioManifoldPackageValidationCheck {
        check_id: format!("validation.package.{PROJECTED_MOTION_BREATH_PACKAGE_ID}.{suffix}"),
        status: StudioValidationStatus::Pass,
        evidence: format!("{suffix} passed"),
    })
    .collect::<Vec<_>>();
    checks.push(StudioManifoldPackageValidationCheck {
        check_id: "validation.package.package.other.dotted_ids".to_string(),
        status: StudioValidationStatus::Pass,
        evidence: "other package passed".to_string(),
    });
    StudioManifoldPackageValidationReport {
        schema_id: MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA.to_string(),
        status: StudioValidationStatus::Pass,
        checks,
    }
}

fn ready_projected_motion_package_evidence_intake() -> StudioPackageEvidenceIntakeReport {
    package_evidence_intake_for_validation_report(
        &projected_motion_package_validation_report(),
        Some(Path::new("target/manifold-package-validation.json")),
        PROJECTED_MOTION_BREATH_PACKAGE_ID,
    )
}

fn projected_motion_profile_document() -> Value {
    serde_json::json!({
        "$schema": MOTION_BREATH_PROFILE_SCHEMA,
        "profile_id": "profile.projected_motion_breath.synthetic_default",
        "target_module_id": "module.breath.projected_motion",
        "input_kinds": ["pose", "vector3"],
        "projection": {
            "mode": "principal_motion_axis",
            "fallback_mode": "fixed_axis"
        }
    })
}

fn ready_projected_motion_authoring_review() -> StudioProjectedMotionBreathAuthoringReviewReport {
    projected_motion_breath_authoring_review_for_intake(
        &ready_projected_motion_package_evidence_intake(),
        Some(Path::new("target/package-evidence-intake.json")),
        &projected_motion_profile_document(),
        Some(Path::new("target/profile-synthetic.json")),
    )
}

fn projected_motion_source_adapter_descriptors() -> Value {
    serde_json::json!({
        "$schema": PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA,
        "descriptor_set_id": "descriptor_set.projected_motion_breath.source_adapters.synthetic",
        "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
        "target_module_id": "module.breath.projected_motion",
        "execution_policy": "not_executed.schema_descriptors_only",
        "runtime_execution_performed": false,
        "platform_execution_performed": false,
        "device_required": false,
        "source_adapters": [
            {
                "adapter_id": "adapter.projected_motion_breath.object_pose_generic",
                "source_kind": "object_pose",
                "input_kind": "pose",
                "output_stream_id": "stream.motion.object_pose",
                "transport_kind": "descriptor_only",
                "requires_platform_sdk": false,
                "requires_device_api": false,
                "runtime_adapter_included": false
            },
            {
                "adapter_id": "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
                "source_kind": "external_patch_stream_bridge",
                "input_kind": "vector3",
                "output_stream_id": "stream.motion.vector3",
                "transport_kind": "descriptor_only",
                "requires_platform_sdk": false,
                "requires_device_api": false,
                "runtime_adapter_included": false
            }
        ]
    })
}

fn ready_projected_motion_source_adapter_selection(
) -> StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
    projected_motion_breath_source_adapter_selection_review_for_authoring(
        &ready_projected_motion_authoring_review(),
        Some(Path::new(
            "target/projected-motion-breath-authoring-review.json",
        )),
        &projected_motion_source_adapter_descriptors(),
        Some(Path::new("target/source-adapter-descriptors.json")),
        "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
    )
}

fn projected_motion_source_binding_document() -> Value {
    serde_json::json!({
        "$schema": PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA,
        "binding_id": "binding.projected_motion_breath.synthetic.external_patch_stream",
        "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
        "target_module_id": "module.breath.projected_motion",
        "profile_id": "profile.projected_motion_breath.synthetic_default",
        "profile_path": "fixtures/valid/profile-synthetic.json",
        "descriptor_set_path": "fixtures/valid/source-adapter-descriptors.json",
        "selected_adapter_id": "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
        "selected_source_kind": "external_patch_stream_bridge",
        "selected_input_kind": "vector3",
        "selected_output_stream_id": "stream.motion.vector3",
        "source_stream_id": "stream.motion.vector3",
        "binding_policy": "descriptor_only.owner_review_required",
        "execution_policy": "not_executed.schema_binding_only",
        "runtime_execution_performed": false,
        "platform_execution_performed": false,
        "device_required": false
    })
}

fn projected_motion_adapter_normalization_case_document() -> Value {
    serde_json::json!({
        "$schema": PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA,
        "case_id": "case.projected_motion_breath.normalize.external_patch_vector",
        "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
        "binding_path": "fixtures/valid/source-binding-synthetic.json",
        "source_payload_kind": "external_patch_channels",
        "input": {
            "source_id": "source.synthetic.patch_stream",
            "sample_time_s": 12.0,
            "host_time_s": 12.011,
            "frame_id": "frame.synthetic.patch",
            "channel_values": {
                "breath_x": 0.11,
                "breath_y": 0.42,
                "breath_z": -0.03
            },
            "channel_map": {
                "x": "breath_x",
                "y": "breath_y",
                "z": "breath_z"
            },
            "units": "normalized_patch_units",
            "quality01": 0.91
        },
        "expected_sample_kind": "vector_motion",
        "expected": {
            "source_id": "source.synthetic.patch_stream",
            "sample_time_s": 12.0,
            "host_time_s": 12.011,
            "frame_id": "frame.synthetic.patch",
            "vector3": [0.11, 0.42, -0.03],
            "units": "normalized_patch_units",
            "quality01": 0.91
        },
        "execution_policy": "not_executed.fixture_normalization_only",
        "runtime_execution_performed": false,
        "platform_execution_performed": false,
        "device_required": false
    })
}

pub(super) fn projected_motion_shell_handoff_evidence() -> Value {
    serde_json::json!({
        "$schema": "rusty.hostess.projected_motion_breath.shell_handoff_validation_evidence.v1",
        "status": "pass",
        "package": {
            "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID
        },
        "execution": {
            "runtime_execution_performed": false,
            "platform_execution_performed": false,
            "broker_transport_used": false,
            "downstream_shell_runtime_used": false,
            "legacy_app_dependency_used": false,
            "legacy_rusty_xr_repo_used": false
        },
        "shell_handoff": {
            "handoff_id": "shell_handoff.projected_motion_breath.loopback",
            "target_host_profile": "host.headset",
            "shell_app_id": "app.downstream_shell",
            "binding_pairs": [
                {"stream_id": "stream.motion.object_pose", "direction": "publish"},
                {"stream_id": "stream.breath.feedback_state", "direction": "subscribe"},
                {"stream_id": "stream.breath.feedback_receipt", "direction": "publish"}
            ],
            "command_ids": ["command.breath.status"],
            "transport_offers": [
                {"transport_id": "transport.shell_loopback", "transport": "http"}
            ]
        },
        "package_contract": {
            "exported_stream_ids": [
                "stream.motion.object_pose",
                "stream.breath.feedback_state",
                "stream.breath.feedback_receipt"
            ],
            "feedback_sink_provides_streams": [
                "stream.breath.feedback_state",
                "stream.breath.feedback_receipt"
            ]
        },
        "scorecard": {
            "status": "pass"
        }
    })
}

#[test]
fn package_evidence_intake_accepts_projected_motion_report() {
    let root = temp_root("package-evidence-intake");
    let report_path = root.join("manifold-package-validation.json");
    let report = projected_motion_package_validation_report();
    save_json(&report_path, &report).expect("save package validation report");

    let loaded =
        load_manifold_package_validation_report(&report_path).expect("load package report");
    let intake = package_evidence_intake_for_validation_report(
        &loaded,
        Some(&report_path),
        PROJECTED_MOTION_BREATH_PACKAGE_ID,
    );

    assert_eq!(intake.schema_id, PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA);
    assert_eq!(
        intake.source_report_schema,
        MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Ready);
    assert_eq!(intake.issue_code, None);
    assert_eq!(intake.execution_policy, "not_executed.review_only");
    assert_eq!(intake.runtime_authority, "rusty.manifold");
    assert_eq!(intake.authoring_authority, "rusty.studio");
    assert_eq!(intake.platform_validation_authority, "rusty.hostess");
    assert!(!intake.runtime_execution_performed);
    assert!(!intake.platform_execution_performed);
    assert_eq!(intake.required_check_count, 3);
    assert_eq!(intake.ready_required_check_count, 3);
    assert_eq!(intake.blocked_required_check_count, 0);
    assert_eq!(intake.target_package_check_count, 17);
    assert_eq!(intake.observed_check_count, 14);
    assert!(intake
        .prohibited_actions
        .iter()
        .any(|action| action == "start_runtime_package"));
    assert!(intake.entries.iter().any(|entry| {
        entry.check_id
            == "validation.package.package.projected_motion_breath.projected_motion_goldens"
            && entry.required_for_studio
            && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
    }));
    assert!(intake
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn package_evidence_intake_blocks_missing_required_projected_motion_check() {
    let mut report = projected_motion_package_validation_report();
    report.checks.retain(|check| {
        check.check_id
            != "validation.package.package.projected_motion_breath.projected_motion_goldens"
    });
    let intake = package_evidence_intake_for_validation_report(
        &report,
        Some(Path::new("target/manifold-package-validation.json")),
        PROJECTED_MOTION_BREATH_PACKAGE_ID,
    );

    assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Blocked);
    assert_eq!(
        intake.issue_code.as_deref(),
        Some("studio.issue.package_evidence_required_check_missing")
    );
    assert_eq!(intake.ready_required_check_count, 2);
    assert_eq!(intake.blocked_required_check_count, 1);
    assert!(intake.entries.iter().any(|entry| {
        entry.check_id
            == "validation.package.package.projected_motion_breath.projected_motion_goldens"
            && entry.required_for_studio
            && entry.decision == StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck
    }));
}

#[test]
fn package_evidence_intake_rejects_unsupported_source_schema() {
    let mut report = projected_motion_package_validation_report();
    report.schema_id = "rusty.manifold.package.validation_report.v0".to_string();
    let intake = package_evidence_intake_for_validation_report(
        &report,
        Some(Path::new("target/manifold-package-validation.json")),
        PROJECTED_MOTION_BREATH_PACKAGE_ID,
    );

    assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Rejected);
    assert_eq!(
        intake.issue_code.as_deref(),
        Some("studio.issue.package_evidence_source_schema")
    );
}

#[test]
fn projected_motion_breath_shell_handoff_review_accepts_hostess_evidence() {
    let root = temp_root("projected-motion-shell-handoff-review");
    let evidence_path = root.join("pmb-shell-handoff.json");
    let evidence = projected_motion_shell_handoff_evidence();
    save_json(&evidence_path, &evidence).expect("save shell handoff evidence");
    let loaded = load_projected_motion_breath_shell_handoff_evidence(&evidence_path)
        .expect("load shell handoff evidence");

    let review =
        projected_motion_breath_shell_handoff_review_for_evidence(&loaded, Some(&evidence_path));

    assert_eq!(
        review.schema_id,
        PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA
    );
    assert_eq!(
        review.status,
        StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
    );
    assert_eq!(review.issue_code, None);
    assert_eq!(
        review.target_package_id.as_deref(),
        Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
    );
    assert_eq!(
        review.handoff_id.as_deref(),
        Some("shell_handoff.projected_motion_breath.loopback")
    );
    assert_eq!(review.required_binding_count, 3);
    assert_eq!(review.ready_required_binding_count, 3);
    assert!(review
        .stream_bindings
        .contains(&"stream.breath.feedback_receipt:publish".to_string()));
    assert!(review
        .command_ids
        .contains(&"command.breath.status".to_string()));
    assert!(review
        .transport_ids
        .contains(&"transport.shell_loopback".to_string()));
    assert!(review.feedback_receipt_exported);
    assert!(review.feedback_sink_provides_receipt);
    assert_eq!(review.runtime_authority, "rusty.manifold");
    assert_eq!(review.authoring_authority, "rusty.studio");
    assert_eq!(review.platform_validation_authority, "rusty.hostess");
    assert!(!review.runtime_execution_performed);
    assert!(!review.platform_execution_performed);
    assert!(!review.broker_transport_used);
    assert!(!review.downstream_shell_runtime_used);
    assert!(!review.legacy_app_dependency_used);
    assert!(review
        .prohibited_actions
        .iter()
        .any(|action| action == "launch_downstream_shell"));
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn projected_motion_breath_shell_handoff_review_blocks_missing_receipt_binding() {
    let mut evidence = projected_motion_shell_handoff_evidence();
    let bindings = evidence["shell_handoff"]["binding_pairs"]
        .as_array_mut()
        .expect("binding pairs");
    bindings.retain(|binding| {
        binding.get("stream_id").and_then(Value::as_str) != Some("stream.breath.feedback_receipt")
    });

    let review = projected_motion_breath_shell_handoff_review_for_evidence(
        &evidence,
        Some(Path::new("target/pmb-shell-handoff.json")),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
    );
    assert_eq!(review.ready_required_binding_count, 2);
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_shell_handoff_required_bindings")
    );
    assert!(review.checks.iter().any(|check| {
        check.check_id == "studio.check.projected_motion_breath_shell_handoff.required_bindings"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn projected_motion_breath_authoring_review_accepts_ready_intake_and_profile() {
    let root = temp_root("projected-motion-authoring-review");
    let intake_path = root.join("package-evidence-intake.json");
    let profile_path = root.join("profile-synthetic.json");
    let intake = ready_projected_motion_package_evidence_intake();
    let profile = projected_motion_profile_document();
    save_json(&intake_path, &intake).expect("save intake");
    save_json(&profile_path, &profile).expect("save profile");
    let loaded_intake =
        load_package_evidence_intake_report(&intake_path).expect("load package intake");
    let loaded_profile =
        load_motion_breath_profile_document(&profile_path).expect("load motion profile");

    let review = projected_motion_breath_authoring_review_for_intake(
        &loaded_intake,
        Some(&intake_path),
        &loaded_profile,
        Some(&profile_path),
    );

    assert_eq!(
        review.schema_id,
        PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA
    );
    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAuthoringReviewStatus::Ready
    );
    assert_eq!(review.issue_code, None);
    assert_eq!(
        review.profile_id.as_deref(),
        Some("profile.projected_motion_breath.synthetic_default")
    );
    assert_eq!(
        review.target_module_id.as_deref(),
        Some("module.breath.projected_motion")
    );
    assert_eq!(
        review.projection_mode.as_deref(),
        Some("principal_motion_axis")
    );
    assert_eq!(review.proposed_command_id, "command.breath.set_profile");
    assert_eq!(
        review.proposed_profile_operation,
        "propose_profile_for_runtime_owner_review"
    );
    assert_eq!(review.runtime_authority, "rusty.manifold");
    assert_eq!(review.authoring_authority, "rusty.studio");
    assert_eq!(review.platform_validation_authority, "rusty.hostess");
    assert!(!review.runtime_execution_performed);
    assert!(!review.platform_execution_performed);
    assert_eq!(review.package_ready_required_check_count, 3);
    assert_eq!(review.package_blocked_required_check_count, 0);
    assert_eq!(review.required_package_checks.len(), 3);
    assert!(review.input_kinds.contains(&"pose".to_string()));
    assert!(review.input_kinds.contains(&"vector3".to_string()));
    assert!(review
        .prohibited_actions
        .iter()
        .any(|action| action == "start_runtime_package"));
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn projected_motion_breath_authoring_review_blocks_unready_package_evidence() {
    let mut report = projected_motion_package_validation_report();
    report.checks.retain(|check| {
        check.check_id
            != "validation.package.package.projected_motion_breath.projected_motion_goldens"
    });
    let intake = package_evidence_intake_for_validation_report(
        &report,
        Some(Path::new("target/manifold-package-validation.json")),
        PROJECTED_MOTION_BREATH_PACKAGE_ID,
    );
    let review = projected_motion_breath_authoring_review_for_intake(
        &intake,
        Some(Path::new("target/package-evidence-intake.json")),
        &projected_motion_profile_document(),
        Some(Path::new("target/profile-synthetic.json")),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.package_evidence_required_check_missing")
    );
    assert_eq!(review.package_ready_required_check_count, 2);
    assert_eq!(review.package_blocked_required_check_count, 1);
}

#[test]
fn projected_motion_breath_authoring_review_rejects_unsupported_profile_schema() {
    let intake = ready_projected_motion_package_evidence_intake();
    let mut profile = projected_motion_profile_document();
    profile["$schema"] = serde_json::json!("rusty.motion_breath_profile.v0");
    let review = projected_motion_breath_authoring_review_for_intake(
        &intake,
        Some(Path::new("target/package-evidence-intake.json")),
        &profile,
        Some(Path::new("target/profile-synthetic.json")),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAuthoringReviewStatus::Rejected
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.motion_breath_profile_schema")
    );
}

#[test]
fn projected_motion_breath_source_adapter_selection_accepts_descriptor() {
    let review = ready_projected_motion_authoring_review();
    let descriptors = projected_motion_source_adapter_descriptors();

    let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
        &review,
        Some(Path::new(
            "target/projected-motion-breath-authoring-review.json",
        )),
        &descriptors,
        Some(Path::new("target/source-adapter-descriptors.json")),
        "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
    );

    assert_eq!(
        selection.schema_id,
        PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready
    );
    assert_eq!(selection.issue_code, None);
    assert_eq!(
        selection.selected_source_kind.as_deref(),
        Some("external_patch_stream_bridge")
    );
    assert_eq!(selection.selected_input_kind.as_deref(), Some("vector3"));
    assert_eq!(
        selection.selected_output_stream_id.as_deref(),
        Some("stream.motion.vector3")
    );
    assert_eq!(selection.source_descriptor_count, 2);
    assert_eq!(selection.matching_descriptor_count, 1);
    assert_eq!(selection.execution_policy, "not_executed.proposal_only");
    assert_eq!(selection.runtime_authority, "rusty.manifold");
    assert_eq!(selection.authoring_authority, "rusty.studio");
    assert_eq!(selection.platform_validation_authority, "rusty.hostess");
    assert!(!selection.runtime_execution_performed);
    assert!(!selection.platform_execution_performed);
    assert_eq!(
        selection.proposal_kind,
        "propose_source_adapter_for_runtime_owner_review"
    );
    assert!(selection
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn projected_motion_breath_source_adapter_selection_blocks_missing_adapter() {
    let review = ready_projected_motion_authoring_review();
    let descriptors = projected_motion_source_adapter_descriptors();

    let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
        &review,
        Some(Path::new(
            "target/projected-motion-breath-authoring-review.json",
        )),
        &descriptors,
        Some(Path::new("target/source-adapter-descriptors.json")),
        "adapter.projected_motion_breath.missing",
    );

    assert_eq!(
        selection.status,
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_source_adapter_missing")
    );
    assert_eq!(selection.matching_descriptor_count, 0);
}

#[test]
fn projected_motion_breath_source_adapter_selection_rejects_bad_descriptor_schema() {
    let review = ready_projected_motion_authoring_review();
    let mut descriptors = projected_motion_source_adapter_descriptors();
    descriptors["$schema"] = serde_json::json!("rusty.manifold.source_adapter.v0");

    let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
        &review,
        Some(Path::new(
            "target/projected-motion-breath-authoring-review.json",
        )),
        &descriptors,
        Some(Path::new("target/source-adapter-descriptors.json")),
        "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
    );

    assert_eq!(
        selection.status,
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_source_adapter_descriptor_schema")
    );
}

#[test]
fn projected_motion_breath_adapter_normalization_review_accepts_selected_evidence() {
    let root = temp_root("projected-motion-adapter-normalization-review");
    let selection_path = root.join("source-adapter-selection-review.json");
    let package_report_path = root.join("manifold-package-validation.json");
    let source_binding_path = root
        .join("fixtures")
        .join("valid")
        .join("source-binding-synthetic.json");
    let normalization_case_path = root.join("adapter-normalization-external-patch-vector.json");
    let selection = ready_projected_motion_source_adapter_selection();
    let package_report = projected_motion_package_validation_report();
    let source_binding = projected_motion_source_binding_document();
    let normalization_case = projected_motion_adapter_normalization_case_document();
    save_json(&selection_path, &selection).expect("save selection review");
    save_json(&package_report_path, &package_report).expect("save package report");
    save_json(&source_binding_path, &source_binding).expect("save source binding");
    save_json(&normalization_case_path, &normalization_case).expect("save normalization case");

    let loaded_selection =
        load_projected_motion_breath_source_adapter_selection_review_report(&selection_path)
            .expect("load selection review");
    let loaded_package =
        load_manifold_package_validation_report(&package_report_path).expect("load package");
    let loaded_binding = load_projected_motion_breath_source_binding_document(&source_binding_path)
        .expect("load source binding");
    let loaded_case =
        load_projected_motion_breath_adapter_normalization_case_document(&normalization_case_path)
            .expect("load normalization case");

    let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
        &loaded_selection,
        Some(&selection_path),
        &loaded_package,
        Some(&package_report_path),
        &loaded_binding,
        Some(&source_binding_path),
        &loaded_case,
        Some(&normalization_case_path),
    );

    assert_eq!(
        review.schema_id,
        PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA
    );
    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready
    );
    assert_eq!(review.issue_code, None);
    assert_eq!(
        review.selected_adapter_id,
        "adapter.projected_motion_breath.external_patch_stream_bridge_shape"
    );
    assert_eq!(
        review.binding_id.as_deref(),
        Some("binding.projected_motion_breath.synthetic.external_patch_stream")
    );
    assert_eq!(
        review.normalization_case_id.as_deref(),
        Some("case.projected_motion_breath.normalize.external_patch_vector")
    );
    assert_eq!(
        review.source_payload_kind.as_deref(),
        Some("external_patch_channels")
    );
    assert_eq!(
        review.expected_sample_kind.as_deref(),
        Some("vector_motion")
    );
    assert_eq!(
        review.adapter_normalization_check_status,
        Some(StudioValidationStatus::Pass)
    );
    assert!(review.source_binding_selected_adapter_match);
    assert!(review.deterministic_normalization_evidence);
    assert_eq!(review.execution_policy, "not_executed.review_only");
    assert_eq!(review.runtime_authority, "rusty.manifold");
    assert_eq!(review.authoring_authority, "rusty.studio");
    assert_eq!(review.platform_validation_authority, "rusty.hostess");
    assert!(!review.runtime_execution_performed);
    assert!(!review.platform_execution_performed);
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn projected_motion_breath_adapter_normalization_review_blocks_missing_package_check() {
    let mut package_report = projected_motion_package_validation_report();
    package_report.checks.retain(|check| {
        check.check_id
            != "validation.package.package.projected_motion_breath.projected_motion_adapter_normalization"
    });
    let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
        &ready_projected_motion_source_adapter_selection(),
        Some(Path::new("target/source-adapter-selection-review.json")),
        &package_report,
        Some(Path::new("target/manifold-package-validation.json")),
        &projected_motion_source_binding_document(),
        Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
        &projected_motion_adapter_normalization_case_document(),
        Some(Path::new(
            "fixtures/valid/adapter-normalization-external-patch-vector.json",
        )),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_adapter_normalization_check_missing")
    );
    assert_eq!(review.adapter_normalization_check_status, None);
}

#[test]
fn projected_motion_breath_adapter_normalization_review_blocks_binding_adapter_drift() {
    let mut source_binding = projected_motion_source_binding_document();
    source_binding["selected_adapter_id"] =
        serde_json::json!("adapter.projected_motion_breath.vector_motion_generic");

    let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
        &ready_projected_motion_source_adapter_selection(),
        Some(Path::new("target/source-adapter-selection-review.json")),
        &projected_motion_package_validation_report(),
        Some(Path::new("target/manifold-package-validation.json")),
        &source_binding,
        Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
        &projected_motion_adapter_normalization_case_document(),
        Some(Path::new(
            "fixtures/valid/adapter-normalization-external-patch-vector.json",
        )),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_adapter_normalization_binding_adapter")
    );
    assert!(!review.source_binding_selected_adapter_match);
    assert!(!review.deterministic_normalization_evidence);
}

#[test]
fn projected_motion_breath_adapter_normalization_review_rejects_bad_case_schema() {
    let mut normalization_case = projected_motion_adapter_normalization_case_document();
    normalization_case["$schema"] =
        serde_json::json!("rusty.manifold.projected_motion_breath.adapter_normalization_case.v0");

    let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
        &ready_projected_motion_source_adapter_selection(),
        Some(Path::new("target/source-adapter-selection-review.json")),
        &projected_motion_package_validation_report(),
        Some(Path::new("target/manifold-package-validation.json")),
        &projected_motion_source_binding_document(),
        Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
        &normalization_case,
        Some(Path::new(
            "fixtures/valid/adapter-normalization-external-patch-vector.json",
        )),
    );

    assert_eq!(
        review.status,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_adapter_normalization_case_schema")
    );
}
