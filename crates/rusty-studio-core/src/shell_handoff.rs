use super::*;

pub fn shell_handoff_for_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellHandoffReport {
    let validation = validate_selected_shell_bundle(project, base_dir, graph_id, bundle_dir);
    let artifact_manifest_path = relative_output_path(bundle_dir, "shell-artifacts.json");
    let template_index_path = relative_output_path(bundle_dir, "shell-templates.json");
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_bundle_validation_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.shell_bundle_validation_failed".to_string());
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some(issue_code),
            "Selected shell bundle validation failed".to_string(),
            bundle_dir,
            String::new(),
            artifact_manifest_path.display().to_string(),
            template_index_path.display().to_string(),
            String::new(),
            Vec::new(),
            StudioShellTargetKind::Unknown,
            None,
            validation,
        );
    }

    let index = match load_shell_template_index(&template_index_path) {
        Ok(index) => index,
        Err(error) => {
            return shell_handoff_report(
                project,
                graph_id,
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_template_index_parse_failed".to_string()),
                error.to_string(),
                bundle_dir,
                String::new(),
                artifact_manifest_path.display().to_string(),
                template_index_path.display().to_string(),
                String::new(),
                Vec::new(),
                StudioShellTargetKind::Unknown,
                None,
                validation,
            );
        }
    };
    let Some(entry) = index
        .templates
        .iter()
        .find(|entry| entry.graph_id == graph_id)
        .or_else(|| index.templates.first())
    else {
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_template_missing".to_string()),
            "Shell template index does not declare a loadable template".to_string(),
            bundle_dir,
            String::new(),
            artifact_manifest_path.display().to_string(),
            template_index_path.display().to_string(),
            String::new(),
            Vec::new(),
            StudioShellTargetKind::Unknown,
            None,
            validation,
        );
    };

    let descriptor_path = relative_output_path(bundle_dir, &entry.descriptor_path);
    let template_manifest_path = relative_output_path(bundle_dir, &entry.template_path);
    let template_manifest = match load_shell_template_manifest(&template_manifest_path) {
        Ok(template_manifest) => template_manifest,
        Err(error) => {
            return shell_handoff_report(
                project,
                graph_id,
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_template_manifest_parse_failed".to_string()),
                error.to_string(),
                bundle_dir,
                descriptor_path.display().to_string(),
                artifact_manifest_path.display().to_string(),
                template_index_path.display().to_string(),
                template_manifest_path.display().to_string(),
                Vec::new(),
                entry.target_kind,
                None,
                validation,
            );
        }
    };

    shell_handoff_report(
        project,
        graph_id,
        StudioValidationStatus::Pass,
        None,
        format!(
            "{} shell handoff ready",
            shell_target_kind_label(entry.target_kind)
        ),
        bundle_dir,
        descriptor_path.display().to_string(),
        artifact_manifest_path.display().to_string(),
        template_index_path.display().to_string(),
        template_manifest_path.display().to_string(),
        vec![
            "--templates".to_string(),
            template_index_path.display().to_string(),
        ],
        entry.target_kind,
        Some(template_manifest.runtime_authority),
        validation,
    )
}

pub fn desktop_shell_handoff_for_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellHandoffReport {
    let report = shell_handoff_for_bundle(project, base_dir, graph_id, bundle_dir);
    if report.status == StudioValidationStatus::Pass
        && report.target_kind != StudioShellTargetKind::Desktop
    {
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_handoff_target_mismatch".to_string()),
            format!(
                "Selected shell bundle targets {}; desktop shell handoff requires desktop",
                shell_target_kind_label(report.target_kind)
            ),
            bundle_dir,
            report.descriptor_path,
            report.artifact_manifest_path,
            report.template_index_path,
            report.template_manifest_path,
            Vec::new(),
            report.target_kind,
            report.runtime_authority,
            report.validation,
        );
    }
    report
}

pub fn shell_handoff_readiness_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffReadinessReport {
    let plan = export_plan(project);
    let reference_index = reference_index_for_project(project, base_dir);
    let entries = project
        .graphs
        .iter()
        .zip(plan.bundles.iter())
        .map(|(graph, export_bundle)| {
            let bundle_dir = bundle_root.join(&graph.graph_id);
            let handoff = shell_handoff_for_bundle(project, base_dir, &graph.graph_id, &bundle_dir);
            let host_profile =
                shell_host_profile(&graph.target_host_profile, reference_index.as_ref());
            let intended_target_kind = shell_target_kind(host_profile.host_profile.as_deref());
            shell_handoff_readiness_entry(graph, export_bundle, handoff, intended_target_kind)
        })
        .collect::<Vec<_>>();
    let graph_count = entries.len();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = entries
        .iter()
        .filter(|entry| entry.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = entries
        .iter()
        .filter(|entry| {
            entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    let target_summaries = shell_handoff_readiness_target_summaries(&entries);
    let status = if entries.is_empty()
        || entries
            .iter()
            .any(|entry| entry.status == StudioValidationStatus::Fail)
    {
        StudioValidationStatus::Fail
    } else {
        StudioValidationStatus::Pass
    };
    StudioShellHandoffReadinessReport {
        schema_id: SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        bundle_root: bundle_root.display().to_string(),
        status,
        graph_count,
        ready_count,
        failed_count,
        missing_bundle_count,
        target_summaries,
        entries,
    }
}

pub fn shell_handoff_manifest_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffManifest {
    let readiness = shell_handoff_readiness_for_project(project, base_dir, bundle_root);
    shell_handoff_manifest_from_readiness(&readiness)
}

pub fn validate_shell_handoff_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellHandoffManifestValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.schema",
        manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA,
        "shell handoff manifest schema id is supported",
        "shell handoff manifest schema id is unsupported",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.manifest_id",
        is_dotted_id(&manifest.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.project_id",
        is_dotted_id(&manifest.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.project_revision",
        manifest.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.source_readiness_schema",
        manifest.source_readiness_schema == SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
        "source readiness schema id is supported",
        "source readiness schema id is unsupported",
        "studio.issue.shell_handoff_readiness_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.bundle_root",
        !manifest.bundle_root.trim().is_empty(),
        "bundle root is present",
        "bundle root must be present",
        "studio.issue.missing_bundle_root",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.handoffs_present",
        !manifest.handoffs.is_empty(),
        "manifest declares shell handoffs",
        "manifest must declare at least one shell handoff",
        "studio.issue.no_shell_handoffs",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.targets_present",
        !manifest.targets.is_empty(),
        "manifest declares target summaries",
        "manifest must declare at least one target summary",
        "studio.issue.no_target_summaries",
    );
    validate_shell_handoff_manifest_counts(manifest, &mut checks);
    validate_shell_handoff_manifest_authority(
        "studio.check.shell_handoff_manifest.runtime_authority",
        &manifest.runtime_authority,
        &mut checks,
    );
    validate_shell_handoff_manifest_target_coverage(manifest, &mut checks);
    for target in &manifest.targets {
        validate_shell_handoff_manifest_target(target, &manifest.handoffs, &mut checks);
    }
    for handoff in &manifest.handoffs {
        validate_shell_handoff_manifest_entry(handoff, &mut checks);
    }

    StudioShellHandoffManifestValidationReport {
        schema_id: SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA.to_string(),
        manifest_id: manifest.manifest_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

fn validate_shell_handoff_manifest_counts(
    manifest: &StudioShellHandoffManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let ready_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| {
            handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.graph_count",
        manifest.graph_count == manifest.handoffs.len(),
        "graph count matches declared handoffs",
        &format!(
            "graph count {} does not match {} handoffs",
            manifest.graph_count,
            manifest.handoffs.len()
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.ready_count",
        manifest.ready_count == ready_count,
        "ready count matches handoff statuses",
        &format!(
            "ready count {} does not match {ready_count} ready handoffs",
            manifest.ready_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.failed_count",
        manifest.failed_count == failed_count,
        "failed count matches handoff statuses",
        &format!(
            "failed count {} does not match {failed_count} failed handoffs",
            manifest.failed_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.missing_bundle_count",
        manifest.missing_bundle_count == missing_bundle_count,
        "missing-bundle count matches handoff issue codes",
        &format!(
            "missing-bundle count {} does not match {missing_bundle_count} missing handoffs",
            manifest.missing_bundle_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    let expected_status = shell_handoff_manifest_expected_status(&manifest.handoffs);
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.status",
        manifest.status == expected_status,
        "manifest status matches handoff statuses",
        "manifest status does not match handoff statuses",
        "studio.issue.shell_handoff_manifest_status_mismatch",
    );
}

fn validate_shell_handoff_manifest_target_coverage(
    manifest: &StudioShellHandoffManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    for target_kind in shell_target_kinds() {
        let handoff_count = manifest
            .handoffs
            .iter()
            .filter(|handoff| handoff.target_kind == target_kind)
            .count();
        let target_count = manifest
            .targets
            .iter()
            .filter(|target| target.target_kind == target_kind)
            .count();
        let label = shell_target_kind_label(target_kind);
        push_check(
            checks,
            &format!("studio.check.shell_handoff_manifest.target.{label}.unique_summary"),
            target_count <= 1,
            "target summary is unique",
            "target summary appears more than once",
            "studio.issue.duplicate_target_summary",
        );
        push_check(
            checks,
            &format!("studio.check.shell_handoff_manifest.target.{label}.summary_present"),
            handoff_count == 0 || target_count == 1,
            "target summary is present when handoffs target it",
            "target summary is missing for one or more handoffs",
            "studio.issue.missing_target_summary",
        );
    }
}

fn validate_shell_handoff_manifest_target(
    target: &StudioShellHandoffManifestTarget,
    handoffs: &[StudioShellHandoffManifestEntry],
    checks: &mut Vec<StudioValidationCheck>,
) {
    let target_handoffs = handoffs
        .iter()
        .filter(|handoff| handoff.target_kind == target.target_kind)
        .collect::<Vec<_>>();
    let label = shell_target_kind_label(target.target_kind);
    let ready_count = target_handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = target_handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = target_handoffs
        .iter()
        .filter(|handoff| {
            handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    let package_count = target_handoffs
        .iter()
        .map(|handoff| handoff.package_ids.len())
        .sum::<usize>();
    let module_count = target_handoffs
        .iter()
        .map(|handoff| handoff.module_ids.len())
        .sum::<usize>();
    let operator_shell_count = target_handoffs
        .iter()
        .map(|handoff| handoff.operator_shell_ids.len())
        .sum::<usize>();
    let graph_ids = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.graph_id.clone()),
    );
    let consumer_ids = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.consumer_id.clone()),
    );
    let issue_codes = unique_strings(
        target_handoffs
            .iter()
            .filter_map(|handoff| handoff.issue_code.clone()),
    );
    let bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let ready_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let failed_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let missing_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| {
                handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            })
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let template_index_paths = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.template_index_path.clone()),
    );

    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.graph_count"),
        target.graph_count == target_handoffs.len(),
        "target graph count matches handoffs",
        "target graph count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.ready_count"),
        target.ready_count == ready_count,
        "target ready count matches handoffs",
        "target ready count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.failed_count"),
        target.failed_count == failed_count,
        "target failed count matches handoffs",
        "target failed count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.missing_bundle_count"),
        target.missing_bundle_count == missing_bundle_count,
        "target missing-bundle count matches handoffs",
        "target missing-bundle count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.coverage_counts"),
        target.package_count == package_count
            && target.module_count == module_count
            && target.operator_shell_count == operator_shell_count,
        "target package/module/operator-shell counts match handoffs",
        "target package/module/operator-shell counts do not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.graph_ids"),
        same_unique_strings(&target.graph_ids, &graph_ids) && all_dotted_ids(&target.graph_ids),
        "target graph ids match handoffs and use dotted-id grammar",
        "target graph ids do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.consumer_ids"),
        same_unique_strings(&target.consumer_ids, &consumer_ids)
            && all_dotted_ids(&target.consumer_ids),
        "target consumer ids match handoffs and use dotted-id grammar",
        "target consumer ids do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_consumer_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.issue_codes"),
        same_unique_strings(&target.issue_codes, &issue_codes)
            && all_dotted_ids(&target.issue_codes),
        "target issue codes match handoffs and use dotted-id grammar",
        "target issue codes do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_issue_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.bundle_dirs"),
        same_unique_strings(&target.bundle_dirs, &bundle_dirs)
            && target
                .bundle_dirs
                .iter()
                .all(|path| !path.trim().is_empty()),
        "target bundle dirs match handoffs",
        "target bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.ready_bundle_dirs"),
        same_unique_strings(&target.ready_bundle_dirs, &ready_bundle_dirs),
        "target ready bundle dirs match handoffs",
        "target ready bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.failed_bundle_dirs"),
        same_unique_strings(&target.failed_bundle_dirs, &failed_bundle_dirs),
        "target failed bundle dirs match handoffs",
        "target failed bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.missing_bundle_dirs"),
        same_unique_strings(&target.missing_bundle_dirs, &missing_bundle_dirs),
        "target missing bundle dirs match handoffs",
        "target missing bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.template_index_paths"),
        same_unique_strings(&target.template_index_paths, &template_index_paths)
            && target
                .template_index_paths
                .iter()
                .all(|path| path_ends_with_shell_templates(path)),
        "target template-index paths match handoffs",
        "target template-index paths do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
}

fn validate_shell_handoff_manifest_entry(
    handoff: &StudioShellHandoffManifestEntry,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = if handoff.graph_id.is_empty() {
        "unknown".to_string()
    } else {
        handoff.graph_id.clone()
    };
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.export_bundle_id"),
        is_dotted_id(&handoff.export_bundle_id)
            && handoff.export_bundle_id == format!("studio.export.{}", handoff.graph_id),
        "handoff export bundle id matches graph id",
        "handoff export bundle id is invalid or does not match graph id",
        "studio.issue.handoff_export_bundle_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.graph_id"),
        is_dotted_id(&handoff.graph_id),
        "handoff graph id uses dotted-id grammar",
        "handoff graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.target_host_profile"),
        is_dotted_id(&handoff.target_host_profile),
        "handoff target host profile uses dotted-id grammar",
        "handoff target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.consumer"),
        handoff.consumer_id == shell_handoff_consumer_id(handoff.target_kind)
            && is_dotted_id(&handoff.consumer_id),
        "handoff consumer matches target kind",
        "handoff consumer does not match target kind",
        "studio.issue.handoff_consumer_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.kind"),
        handoff.handoff_kind == shell_handoff_kind_for_target(handoff.target_kind),
        "handoff kind matches target kind",
        "handoff kind does not match target kind",
        "studio.issue.handoff_kind_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.ids"),
        all_dotted_ids(&handoff.package_ids)
            && all_dotted_ids(&handoff.module_ids)
            && all_dotted_ids(&handoff.operator_shell_ids),
        "handoff package, module, and operator-shell ids use dotted-id grammar",
        "one or more handoff package, module, or operator-shell ids are invalid",
        "studio.issue.handoff_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.bundle_dir"),
        !handoff.bundle_dir.trim().is_empty(),
        "handoff bundle dir is present",
        "handoff bundle dir must be present",
        "studio.issue.handoff_path_missing",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.template_index_path"),
        path_ends_with_shell_templates(&handoff.template_index_path),
        "handoff template-index path points to shell-templates.json",
        "handoff template-index path must point to shell-templates.json",
        "studio.issue.handoff_template_index_path_mismatch",
    );
    let pass_status_consistent = handoff.status != StudioValidationStatus::Pass
        || (handoff.issue_code.is_none()
            && handoff.validation_status == StudioValidationStatus::Pass
            && handoff.failed_check_count == 0);
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.pass_status"),
        pass_status_consistent,
        "passing handoff has no issue and no failed checks",
        "passing handoff carries an issue, failed validation, or failed checks",
        "studio.issue.handoff_status_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.fail_status"),
        handoff.status != StudioValidationStatus::Fail || handoff.issue_code.is_some(),
        "failing handoff carries an issue code",
        "failing handoff must carry an issue code",
        "studio.issue.handoff_status_mismatch",
    );
    let pass_args_consistent = handoff.status != StudioValidationStatus::Pass
        || (handoff.consumer_args.iter().any(|arg| arg == "--templates")
            && handoff
                .consumer_args
                .iter()
                .any(|arg| arg == &handoff.template_index_path));
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.consumer_args"),
        pass_args_consistent,
        "passing handoff carries template consumer args",
        "passing handoff must carry --templates and the template-index path",
        "studio.issue.handoff_consumer_args_mismatch",
    );
    let runtime_authority_consistent = match handoff.runtime_authority.as_ref() {
        Some(authority) => runtime_authority_matches(authority),
        None => handoff.status == StudioValidationStatus::Fail,
    };
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.runtime_authority"),
        runtime_authority_consistent,
        "handoff runtime authority preserves Manifold/Hostess/Studio boundaries",
        "handoff runtime authority is missing or does not preserve boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
}

fn validate_shell_handoff_manifest_authority(
    check_id: &str,
    authority: &StudioShellRuntimeAuthority,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        check_id,
        runtime_authority_matches(authority),
        "runtime authority preserves Manifold/Hostess/Studio boundaries",
        "runtime authority does not preserve Manifold/Hostess/Studio boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
}

fn shell_handoff_manifest_expected_status(
    handoffs: &[StudioShellHandoffManifestEntry],
) -> StudioValidationStatus {
    if handoffs.is_empty()
        || handoffs
            .iter()
            .any(|handoff| handoff.status == StudioValidationStatus::Fail)
    {
        StudioValidationStatus::Fail
    } else {
        StudioValidationStatus::Pass
    }
}

pub fn shell_handoff_intake_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellHandoffIntakeReport {
    let validation = validate_shell_handoff_manifest(manifest);
    let authority = shell_runtime_authority();
    if validation.status == StudioValidationStatus::Fail {
        return StudioShellHandoffIntakeReport {
            schema_id: SHELL_HANDOFF_INTAKE_REPORT_SCHEMA.to_string(),
            manifest_id: manifest.manifest_id.clone(),
            project_id: manifest.project_id.clone(),
            project_revision: manifest.project_revision,
            status: StudioShellHandoffIntakeStatus::Rejected,
            issue_code: first_failed_validation_check_issue_code(&validation.checks),
            command_session_authority: authority.command_session_authority,
            install_launch_evidence_authority: authority.install_launch_evidence_authority,
            studio_role: authority.studio_role,
            accepted_count: 0,
            blocked_count: 0,
            target_summaries: Vec::new(),
            entries: Vec::new(),
            validation,
        };
    }

    let entries = manifest
        .handoffs
        .iter()
        .map(|handoff| shell_handoff_intake_entry(handoff, &authority))
        .collect::<Vec<_>>();
    let accepted_count = entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = entries.len() - accepted_count;
    let target_summaries = shell_handoff_intake_target_summaries(&entries);

    StudioShellHandoffIntakeReport {
        schema_id: SHELL_HANDOFF_INTAKE_REPORT_SCHEMA.to_string(),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status: StudioShellHandoffIntakeStatus::Accepted,
        issue_code: None,
        command_session_authority: authority.command_session_authority,
        install_launch_evidence_authority: authority.install_launch_evidence_authority,
        studio_role: authority.studio_role,
        accepted_count,
        blocked_count,
        target_summaries,
        entries,
        validation,
    }
}

fn shell_handoff_report(
    project: &StudioProject,
    graph_id: &str,
    status: StudioValidationStatus,
    issue_code: Option<String>,
    message: String,
    bundle_dir: &Path,
    descriptor_path: String,
    artifact_manifest_path: String,
    template_index_path: String,
    template_manifest_path: String,
    consumer_args: Vec<String>,
    target_kind: StudioShellTargetKind,
    runtime_authority: Option<StudioShellRuntimeAuthority>,
    validation: StudioShellBundleValidationReport,
) -> StudioShellHandoffReport {
    StudioShellHandoffReport {
        schema_id: SHELL_HANDOFF_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status,
        issue_code,
        message,
        handoff_kind: shell_handoff_kind_for_target(target_kind),
        consumer_id: shell_handoff_consumer_id(target_kind).to_string(),
        target_kind,
        bundle_dir: bundle_dir.display().to_string(),
        descriptor_path,
        artifact_manifest_path,
        template_index_path,
        template_manifest_path,
        consumer_args,
        runtime_authority,
        validation,
    }
}

fn shell_handoff_readiness_entry(
    graph: &StudioGraph,
    export_bundle: &StudioExportBundle,
    handoff: StudioShellHandoffReport,
    intended_target_kind: StudioShellTargetKind,
) -> StudioShellHandoffReadinessEntry {
    let failed_check_count = handoff
        .validation
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let package_count = export_bundle.package_ids.len();
    let module_count = export_bundle.module_ids.len();
    let operator_shell_count = export_bundle.operator_shell_ids.len();
    let uses_intended_target = handoff.target_kind == StudioShellTargetKind::Unknown
        && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing");
    let target_kind = if uses_intended_target {
        intended_target_kind
    } else {
        handoff.target_kind
    };
    let handoff_kind = if uses_intended_target {
        shell_handoff_kind_for_target(target_kind)
    } else {
        handoff.handoff_kind
    };
    let consumer_id = if uses_intended_target {
        shell_handoff_consumer_id(target_kind).to_string()
    } else {
        handoff.consumer_id
    };
    StudioShellHandoffReadinessEntry {
        export_bundle_id: export_bundle.bundle_id.clone(),
        graph_id: graph.graph_id.clone(),
        display_name: graph.display_name.clone(),
        target_host_profile: export_bundle.target_host_profile.clone(),
        target_kind,
        package_ids: export_bundle.package_ids.clone(),
        module_ids: export_bundle.module_ids.clone(),
        operator_shell_ids: export_bundle.operator_shell_ids.clone(),
        package_count,
        module_count,
        operator_shell_count,
        status: handoff.status,
        issue_code: handoff.issue_code,
        message: handoff.message,
        handoff_kind,
        consumer_id,
        bundle_dir: handoff.bundle_dir,
        template_index_path: handoff.template_index_path,
        consumer_args: handoff.consumer_args,
        runtime_authority: handoff.runtime_authority,
        validation_status: handoff.validation.status,
        failed_check_count,
    }
}

fn shell_handoff_readiness_target_summaries(
    entries: &[StudioShellHandoffReadinessEntry],
) -> Vec<StudioShellHandoffReadinessTargetSummary> {
    [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
        StudioShellTargetKind::Unknown,
    ]
    .iter()
    .filter_map(|target_kind| shell_handoff_readiness_target_summary(entries, *target_kind))
    .collect()
}

fn shell_handoff_readiness_target_summary(
    entries: &[StudioShellHandoffReadinessEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffReadinessTargetSummary> {
    let mut graph_count = 0;
    let mut ready_count = 0;
    let mut failed_count = 0;
    let mut missing_bundle_count = 0;
    let mut package_count = 0;
    let mut module_count = 0;
    let mut operator_shell_count = 0;
    let mut graph_ids = Vec::new();
    let mut consumer_ids = Vec::new();
    let mut issue_codes = Vec::new();
    let mut bundle_dirs = Vec::new();
    let mut ready_bundle_dirs = Vec::new();
    let mut failed_bundle_dirs = Vec::new();
    let mut missing_bundle_dirs = Vec::new();
    let mut template_index_paths = Vec::new();

    for entry in entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
    {
        graph_count += 1;
        if entry.status == StudioValidationStatus::Pass {
            ready_count += 1;
        }
        if entry.status == StudioValidationStatus::Fail {
            failed_count += 1;
        }
        if entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing") {
            missing_bundle_count += 1;
        }
        package_count += entry.package_count;
        module_count += entry.module_count;
        operator_shell_count += entry.operator_shell_count;
        graph_ids.push(entry.graph_id.clone());
        if !bundle_dirs.contains(&entry.bundle_dir) {
            bundle_dirs.push(entry.bundle_dir.clone());
        }
        if !template_index_paths.contains(&entry.template_index_path) {
            template_index_paths.push(entry.template_index_path.clone());
        }
        if entry.status == StudioValidationStatus::Pass
            && !ready_bundle_dirs.contains(&entry.bundle_dir)
        {
            ready_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if entry.status == StudioValidationStatus::Fail
            && !failed_bundle_dirs.contains(&entry.bundle_dir)
        {
            failed_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && !missing_bundle_dirs.contains(&entry.bundle_dir)
        {
            missing_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if !consumer_ids.contains(&entry.consumer_id) {
            consumer_ids.push(entry.consumer_id.clone());
        }
        if let Some(issue_code) = entry.issue_code.as_ref() {
            if !issue_codes.contains(issue_code) {
                issue_codes.push(issue_code.clone());
            }
        }
    }

    (graph_count > 0).then(|| StudioShellHandoffReadinessTargetSummary {
        target_kind,
        graph_count,
        ready_count,
        failed_count,
        missing_bundle_count,
        package_count,
        module_count,
        operator_shell_count,
        graph_ids,
        consumer_ids,
        issue_codes,
        bundle_dirs,
        ready_bundle_dirs,
        failed_bundle_dirs,
        missing_bundle_dirs,
        template_index_paths,
    })
}

fn shell_handoff_manifest_from_readiness(
    readiness: &StudioShellHandoffReadinessReport,
) -> StudioShellHandoffManifest {
    StudioShellHandoffManifest {
        schema_id: SHELL_HANDOFF_MANIFEST_SCHEMA.to_string(),
        manifest_id: shell_handoff_manifest_id(&readiness.project_id),
        project_id: readiness.project_id.clone(),
        project_revision: readiness.revision,
        source_readiness_schema: readiness.schema_id.to_string(),
        bundle_root: readiness.bundle_root.clone(),
        status: readiness.status,
        graph_count: readiness.graph_count,
        ready_count: readiness.ready_count,
        failed_count: readiness.failed_count,
        missing_bundle_count: readiness.missing_bundle_count,
        runtime_authority: shell_runtime_authority(),
        targets: readiness
            .target_summaries
            .iter()
            .map(shell_handoff_manifest_target)
            .collect(),
        handoffs: readiness
            .entries
            .iter()
            .map(shell_handoff_manifest_entry)
            .collect(),
    }
}

fn shell_handoff_manifest_target(
    summary: &StudioShellHandoffReadinessTargetSummary,
) -> StudioShellHandoffManifestTarget {
    StudioShellHandoffManifestTarget {
        target_kind: summary.target_kind,
        graph_count: summary.graph_count,
        ready_count: summary.ready_count,
        failed_count: summary.failed_count,
        missing_bundle_count: summary.missing_bundle_count,
        package_count: summary.package_count,
        module_count: summary.module_count,
        operator_shell_count: summary.operator_shell_count,
        graph_ids: summary.graph_ids.clone(),
        consumer_ids: summary.consumer_ids.clone(),
        issue_codes: summary.issue_codes.clone(),
        bundle_dirs: summary.bundle_dirs.clone(),
        ready_bundle_dirs: summary.ready_bundle_dirs.clone(),
        failed_bundle_dirs: summary.failed_bundle_dirs.clone(),
        missing_bundle_dirs: summary.missing_bundle_dirs.clone(),
        template_index_paths: summary.template_index_paths.clone(),
    }
}

fn shell_handoff_manifest_entry(
    entry: &StudioShellHandoffReadinessEntry,
) -> StudioShellHandoffManifestEntry {
    StudioShellHandoffManifestEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        status: entry.status,
        issue_code: entry.issue_code.clone(),
        message: entry.message.clone(),
        handoff_kind: entry.handoff_kind,
        consumer_id: entry.consumer_id.clone(),
        bundle_dir: entry.bundle_dir.clone(),
        template_index_path: entry.template_index_path.clone(),
        consumer_args: entry.consumer_args.clone(),
        runtime_authority: entry.runtime_authority.clone(),
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
        validation_status: entry.validation_status,
        failed_check_count: entry.failed_check_count,
    }
}

fn shell_handoff_intake_entry(
    handoff: &StudioShellHandoffManifestEntry,
    authority: &StudioShellRuntimeAuthority,
) -> StudioShellHandoffIntakeEntry {
    let decision = if handoff.status == StudioValidationStatus::Pass {
        StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    } else {
        StudioShellHandoffIntakeDecision::BlockedByHandoffIssue
    };
    StudioShellHandoffIntakeEntry {
        export_bundle_id: handoff.export_bundle_id.clone(),
        graph_id: handoff.graph_id.clone(),
        display_name: handoff.display_name.clone(),
        target_host_profile: handoff.target_host_profile.clone(),
        target_kind: handoff.target_kind,
        handoff_kind: handoff.handoff_kind,
        consumer_id: handoff.consumer_id.clone(),
        handoff_status: handoff.status,
        issue_code: handoff.issue_code.clone(),
        decision,
        handoff_request_kind: "operator_shell_handoff".to_string(),
        runtime_route_kind: format!(
            "{}_operator_shell",
            shell_target_kind_label(handoff.target_kind)
        ),
        next_required_action: shell_handoff_intake_next_action(decision).to_string(),
        bundle_dir: handoff.bundle_dir.clone(),
        template_index_path: handoff.template_index_path.clone(),
        consumer_args: handoff.consumer_args.clone(),
        command_session_authority: authority.command_session_authority.clone(),
        install_launch_evidence_authority: authority.install_launch_evidence_authority.clone(),
        studio_role: authority.studio_role.clone(),
        package_ids: handoff.package_ids.clone(),
        module_ids: handoff.module_ids.clone(),
        operator_shell_ids: handoff.operator_shell_ids.clone(),
    }
}

fn shell_handoff_intake_next_action(decision: StudioShellHandoffIntakeDecision) -> &'static str {
    match decision {
        StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner => "stage_with_runtime_owner",
        StudioShellHandoffIntakeDecision::BlockedByManifestIssue => "repair_handoff_manifest",
        StudioShellHandoffIntakeDecision::BlockedByHandoffIssue => "repair_export_bundle",
    }
}

fn shell_handoff_intake_target_summaries(
    entries: &[StudioShellHandoffIntakeEntry],
) -> Vec<StudioShellHandoffIntakeTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_handoff_intake_target_summary(entries, *target_kind))
        .collect()
}

fn shell_handoff_intake_target_summary(
    entries: &[StudioShellHandoffIntakeEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffIntakeTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    let accepted_count = target_entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = target_entries.len() - accepted_count;
    Some(StudioShellHandoffIntakeTargetSummary {
        target_kind,
        accepted_count,
        blocked_count,
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        bundle_dirs: unique_strings(target_entries.iter().map(|entry| entry.bundle_dir.clone())),
        template_index_paths: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.template_index_path.clone()),
        ),
    })
}
