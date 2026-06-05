use super::*;

pub fn shell_export_package_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellExportPackageReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    shell_export_package_for_manifest(&manifest)
}

pub fn shell_export_package_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellExportPackageReport {
    let runbook = shell_runbook_for_manifest(manifest);
    let entries = runbook
        .entries
        .iter()
        .map(shell_export_package_entry)
        .collect::<Vec<_>>();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
        .count();
    let rejected_count = if runbook.status == StudioShellRunbookStatus::Rejected {
        1
    } else {
        entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count()
    };
    let descriptor_count = entries
        .iter()
        .filter(|entry| entry.descriptor.is_some())
        .count();
    let template_manifest_count = entries
        .iter()
        .filter(|entry| entry.template_manifest.is_some())
        .count();
    let status = if runbook.status == StudioShellRunbookStatus::Rejected {
        StudioShellExportPackageStatus::Rejected
    } else if blocked_count > 0 || entries.is_empty() {
        StudioShellExportPackageStatus::Blocked
    } else {
        StudioShellExportPackageStatus::Ready
    };
    let issue_code = match status {
        StudioShellExportPackageStatus::Ready => None,
        StudioShellExportPackageStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellExportPackageStatus::Rejected => runbook.issue_code.clone(),
    };

    StudioShellExportPackageReport {
        schema_id: SHELL_EXPORT_PACKAGE_REPORT_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        source_runbook_schema: runbook.schema_id.clone(),
        package_id: format!("studio.shell_export_package.{}", manifest.project_id),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        bundle_root: manifest.bundle_root.clone(),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        review_owner: "rusty.hostess".to_string(),
        command_session_authority: manifest.runtime_authority.command_session_authority.clone(),
        install_launch_evidence_authority: manifest
            .runtime_authority
            .install_launch_evidence_authority
            .clone(),
        studio_role: manifest.runtime_authority.studio_role.clone(),
        ready_count,
        blocked_count,
        rejected_count,
        descriptor_count,
        template_manifest_count,
        runbook_entry_count: runbook.entries.len(),
        target_summaries: shell_export_package_target_summaries(&entries),
        prohibited_actions: runbook.prohibited_actions,
        entries,
    }
}

fn shell_export_package_entry(entry: &StudioShellRunbookEntry) -> StudioShellExportPackageEntry {
    let (descriptor, template_manifest, package_issue_code) =
        shell_export_package_artifact_refs(entry);
    let source_status = match entry.status {
        StudioShellRunbookStatus::Ready => StudioShellExportPackageStatus::Ready,
        StudioShellRunbookStatus::Blocked => StudioShellExportPackageStatus::Blocked,
        StudioShellRunbookStatus::Rejected => StudioShellExportPackageStatus::Rejected,
    };
    let status = if source_status == StudioShellExportPackageStatus::Ready
        && descriptor.is_some()
        && template_manifest.is_some()
    {
        StudioShellExportPackageStatus::Ready
    } else if source_status == StudioShellExportPackageStatus::Rejected {
        StudioShellExportPackageStatus::Rejected
    } else {
        StudioShellExportPackageStatus::Blocked
    };
    let issue_code = match status {
        StudioShellExportPackageStatus::Ready => None,
        StudioShellExportPackageStatus::Blocked => package_issue_code
            .or_else(|| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_export_package_blocked".to_string())),
        StudioShellExportPackageStatus::Rejected => entry.issue_code.clone(),
    };
    let responsible_owner = if status == StudioShellExportPackageStatus::Ready {
        entry.responsible_owner.clone()
    } else {
        "rusty.studio".to_string()
    };

    StudioShellExportPackageEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        status,
        issue_code,
        responsible_owner,
        execution_policy: "not_executed.review_only".to_string(),
        consumer_id: entry.consumer_id.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        next_required_action: "review_with_runtime_owner".to_string(),
        bundle_dir: entry.bundle_dir.clone(),
        descriptor,
        template_manifest,
        runbook_cli_request: if status == StudioShellExportPackageStatus::Ready {
            entry.cli_request.clone()
        } else {
            Vec::new()
        },
        host_routes: entry.host_routes.clone(),
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
    }
}

fn shell_export_package_artifact_refs(
    entry: &StudioShellRunbookEntry,
) -> (
    Option<StudioShellExportPackageDescriptorRef>,
    Option<StudioShellExportPackageTemplateRef>,
    Option<String>,
) {
    if entry.status != StudioShellRunbookStatus::Ready
        && entry.decision != StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    {
        return (None, None, entry.issue_code.clone());
    }

    let index = match load_shell_template_index(Path::new(&entry.template_index_path)) {
        Ok(index) => index,
        Err(_) => {
            return (
                None,
                None,
                Some("studio.issue.shell_export_package_template_index_load_failed".to_string()),
            );
        }
    };
    let Some(template_entry) = index
        .templates
        .iter()
        .find(|template| template.graph_id == entry.graph_id)
    else {
        return (
            None,
            None,
            Some("studio.issue.shell_export_package_template_missing".to_string()),
        );
    };

    let descriptor_path = relative_output_path(
        Path::new(&entry.bundle_dir),
        &template_entry.descriptor_path,
    );
    let template_manifest_path =
        relative_output_path(Path::new(&entry.bundle_dir), &template_entry.template_path);

    let (descriptor_ref, descriptor_issue_code) = match load_shell_descriptor(&descriptor_path) {
        Ok(descriptor)
            if descriptor.graph_id == entry.graph_id
                && descriptor.shell_id == template_entry.shell_id =>
        {
            (
                Some(shell_export_package_descriptor_ref(
                    &descriptor,
                    &descriptor_path,
                )),
                None,
            )
        }
        Ok(_) => (
            None,
            Some("studio.issue.shell_export_package_descriptor_mismatch".to_string()),
        ),
        Err(_) => (
            None,
            Some("studio.issue.shell_export_package_descriptor_load_failed".to_string()),
        ),
    };

    let (template_ref, template_issue_code) =
        match load_shell_template_manifest(&template_manifest_path) {
            Ok(template)
                if template.graph_id == entry.graph_id
                    && template.template_id == template_entry.template_id
                    && template.artifact_id == template_entry.artifact_id =>
            {
                (
                    Some(shell_export_package_template_ref(
                        &template,
                        &entry.template_index_path,
                        &template_manifest_path,
                    )),
                    None,
                )
            }
            Ok(_) => (
                None,
                Some("studio.issue.shell_export_package_template_mismatch".to_string()),
            ),
            Err(_) => (
                None,
                Some("studio.issue.shell_export_package_template_load_failed".to_string()),
            ),
        };

    (
        descriptor_ref,
        template_ref,
        descriptor_issue_code.or(template_issue_code),
    )
}

fn shell_export_package_descriptor_ref(
    descriptor: &StudioShellDescriptor,
    descriptor_path: &Path,
) -> StudioShellExportPackageDescriptorRef {
    StudioShellExportPackageDescriptorRef {
        descriptor_path: descriptor_path.display().to_string(),
        descriptor_id: descriptor.descriptor_id.clone(),
        graph_id: descriptor.graph_id.clone(),
        shell_id: descriptor.shell_id.clone(),
        target_host_profile: descriptor.target_host_profile.clone(),
        package_count: descriptor.package_ids.len(),
        module_count: descriptor.module_ids.len(),
        command_binding_count: descriptor.command_bindings.len(),
        stream_binding_count: descriptor.stream_bindings.len(),
        validation_slot_count: descriptor.validation_slot_ids.len(),
    }
}

fn shell_export_package_template_ref(
    template: &StudioShellTemplateManifest,
    template_index_path: &str,
    template_manifest_path: &Path,
) -> StudioShellExportPackageTemplateRef {
    StudioShellExportPackageTemplateRef {
        template_index_path: template_index_path.to_string(),
        template_manifest_path: template_manifest_path.display().to_string(),
        template_id: template.template_id.clone(),
        artifact_id: template.artifact_id.clone(),
        graph_id: template.graph_id.clone(),
        shell_id: template.shell_id.clone(),
        target_host_profile: template.target_host_profile.clone(),
        host_routes: template.host_routes.clone(),
        runtime_authority: template.runtime_authority.clone(),
    }
}

fn shell_export_package_target_summaries(
    entries: &[StudioShellExportPackageEntry],
) -> Vec<StudioShellExportPackageTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_export_package_target_summary(entries, *target_kind))
        .collect()
}

fn shell_export_package_target_summary(
    entries: &[StudioShellExportPackageEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellExportPackageTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellExportPackageTargetSummary {
        target_kind,
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count(),
        descriptor_count: target_entries
            .iter()
            .filter(|entry| entry.descriptor.is_some())
            .count(),
        template_manifest_count: target_entries
            .iter()
            .filter(|entry| entry.template_manifest.is_some())
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        responsible_owners: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.responsible_owner.clone()),
        ),
        issue_codes: unique_strings(
            target_entries
                .iter()
                .filter_map(|entry| entry.issue_code.clone()),
        ),
    })
}

pub fn shell_export_package_baseline_manifest_for_report(
    package: &StudioShellExportPackageReport,
    package_path: &Path,
    baseline_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellExportPackageBaselineManifest {
    let baseline_id = baseline_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_export_package_baseline_id(package));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_export_package_baseline_label(package));

    StudioShellExportPackageBaselineManifest {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA.to_string(),
        baseline_id,
        label,
        package_path: package_path.display().to_string(),
        package_schema: package.schema_id.clone(),
        package_id: package.package_id.clone(),
        manifest_id: package.manifest_id.clone(),
        project_id: package.project_id.clone(),
        project_revision: package.project_revision,
        status: package.status,
        issue_code: package.issue_code.clone(),
        execution_policy: package.execution_policy.clone(),
        review_owner: package.review_owner.clone(),
        command_session_authority: package.command_session_authority.clone(),
        install_launch_evidence_authority: package.install_launch_evidence_authority.clone(),
        studio_role: package.studio_role.clone(),
        ready_count: package.ready_count,
        blocked_count: package.blocked_count,
        rejected_count: package.rejected_count,
        descriptor_count: package.descriptor_count,
        template_manifest_count: package.template_manifest_count,
        runbook_entry_count: package.runbook_entry_count,
        target_count: package.target_summaries.len(),
        prohibited_actions: package.prohibited_actions.clone(),
    }
}

pub fn shell_export_package_baseline_index_for_manifests(
    baselines: Vec<(StudioShellExportPackageBaselineManifest, Option<PathBuf>)>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let entries = baselines
        .into_iter()
        .map(|(baseline, baseline_manifest_path)| {
            shell_export_package_baseline_index_entry_for_manifest(baseline, baseline_manifest_path)
        })
        .collect::<Vec<_>>();

    shell_export_package_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn append_shell_export_package_baseline_index_manifests(
    index: &StudioShellExportPackageBaselineIndex,
    baselines: Vec<(StudioShellExportPackageBaselineManifest, Option<PathBuf>)>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            baselines
                .into_iter()
                .map(|(baseline, baseline_manifest_path)| {
                    shell_export_package_baseline_index_entry_for_manifest(
                        baseline,
                        baseline_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id.or(index.default_baseline_id.as_deref());

    shell_export_package_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn promote_shell_export_package_baseline_index_default(
    index: &StudioShellExportPackageBaselineIndex,
    baseline_id: &str,
) -> Option<StudioShellExportPackageBaselineIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.baseline_id == baseline_id)
        .then(|| {
            shell_export_package_baseline_index_for_entries(
                index.entries.clone(),
                Some(baseline_id),
            )
        })
}

fn shell_export_package_baseline_index_entry_for_manifest(
    baseline: StudioShellExportPackageBaselineManifest,
    baseline_manifest_path: Option<PathBuf>,
) -> StudioShellExportPackageBaselineIndexEntry {
    StudioShellExportPackageBaselineIndexEntry {
        baseline_id: baseline.baseline_id,
        label: baseline.label,
        baseline_manifest_path: baseline_manifest_path.map(|path| path.display().to_string()),
        package_path: baseline.package_path,
        package_schema: baseline.package_schema,
        package_id: baseline.package_id,
        manifest_id: baseline.manifest_id,
        project_id: baseline.project_id,
        project_revision: baseline.project_revision,
        status: baseline.status,
        issue_code: baseline.issue_code,
        ready_count: baseline.ready_count,
        blocked_count: baseline.blocked_count,
        rejected_count: baseline.rejected_count,
        descriptor_count: baseline.descriptor_count,
        template_manifest_count: baseline.template_manifest_count,
        runbook_entry_count: baseline.runbook_entry_count,
        target_count: baseline.target_count,
    }
}

fn shell_export_package_baseline_index_for_entries(
    entries: Vec<StudioShellExportPackageBaselineIndexEntry>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.baseline_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id
        .filter(|baseline_id| {
            entries
                .iter()
                .any(|entry| entry.baseline_id == *baseline_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.baseline_id.clone()));

    StudioShellExportPackageBaselineIndex {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        package_ids: unique_strings(entries.iter().map(|entry| entry.package_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_baseline_id,
        baseline_count: entries.len(),
        ready_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
            .count(),
        blocked_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .count(),
        rejected_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_export_package_baseline_index_entry<'a>(
    index: &'a StudioShellExportPackageBaselineIndex,
    baseline_id: Option<&str>,
) -> Option<&'a StudioShellExportPackageBaselineIndexEntry> {
    let selected_id = baseline_id.or(index.default_baseline_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.baseline_id == selected_id)
        })
        .or_else(|| {
            baseline_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_export_package_baseline_index_selection(
    index: &StudioShellExportPackageBaselineIndex,
    index_path: Option<&Path>,
    requested_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineSelectionReport {
    let selected_entry =
        select_shell_export_package_baseline_index_entry(index, requested_baseline_id);
    let selected_baseline_id = selected_entry.map(|entry| entry.baseline_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellExportPackageBaselineSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellExportPackageBaselineSelectionStatus::Selected
    } else {
        StudioShellExportPackageBaselineSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellExportPackageBaselineSelectionStatus::Selected => None,
        StudioShellExportPackageBaselineSelectionStatus::Missing => {
            Some("studio.issue.shell_export_package_baseline_not_found".to_string())
        }
        StudioShellExportPackageBaselineSelectionStatus::Empty => {
            Some("studio.issue.shell_export_package_baseline_index_empty".to_string())
        }
    };

    StudioShellExportPackageBaselineSelectionReport {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_baseline_id: requested_baseline_id.map(str::to_string),
        default_baseline_id: index.default_baseline_id.clone(),
        selected_baseline_id: selected_baseline_id.clone(),
        status,
        issue_code,
        baseline_count: index.baseline_count,
        ready_baseline_count: index.ready_baseline_count,
        blocked_baseline_count: index.blocked_baseline_count,
        rejected_baseline_count: index.rejected_baseline_count,
        project_ids: index.project_ids.clone(),
        package_ids: index.package_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellExportPackageBaselineSelectionEntry {
                baseline_id: entry.baseline_id.clone(),
                label: entry.label.clone(),
                selected: selected_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                default: index.default_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                baseline_manifest_path: entry.baseline_manifest_path.clone(),
                package_path: entry.package_path.clone(),
                package_id: entry.package_id.clone(),
                manifest_id: entry.manifest_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_count: entry.ready_count,
                blocked_count: entry.blocked_count,
                rejected_count: entry.rejected_count,
                descriptor_count: entry.descriptor_count,
                template_manifest_count: entry.template_manifest_count,
                runbook_entry_count: entry.runbook_entry_count,
                target_count: entry.target_count,
            })
            .collect(),
    }
}

fn default_shell_export_package_baseline_id(package: &StudioShellExportPackageReport) -> String {
    format!(
        "{}.rev{}.{}",
        package.project_id,
        package.project_revision,
        shell_export_package_status_key(package.status)
    )
}

fn default_shell_export_package_baseline_label(package: &StudioShellExportPackageReport) -> String {
    format!(
        "{} revision {} {} export package baseline",
        package.project_id,
        package.project_revision,
        shell_export_package_status_key(package.status)
    )
}

fn shell_export_package_status_key(status: StudioShellExportPackageStatus) -> &'static str {
    match status {
        StudioShellExportPackageStatus::Ready => "ready",
        StudioShellExportPackageStatus::Blocked => "blocked",
        StudioShellExportPackageStatus::Rejected => "rejected",
    }
}

pub fn compare_shell_export_packages(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, None, None)
}

pub fn compare_shell_export_packages_against_baseline_manifest(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, Some(baseline_identity), None)
}

pub fn compare_shell_export_packages_against_baseline_index_entry(
    baseline_index: &StudioShellExportPackageBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_index_entry: &StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellExportPackageBaselineIndexComparisonContext {
            index: baseline_index,
            index_path: baseline_index_path,
            entry: baseline_index_entry,
            baseline_manifest_path,
        }),
    )
}

struct ShellExportPackageBaselineIndexComparisonContext<'a> {
    index: &'a StudioShellExportPackageBaselineIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&'a Path>,
}

fn compare_shell_export_packages_with_identity(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
    baseline_identity: Option<&StudioShellExportPackageBaselineManifest>,
    baseline_index: Option<ShellExportPackageBaselineIndexComparisonContext<'_>>,
) -> StudioShellExportPackageComparisonReport {
    let mut checks = shell_export_package_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_export_package_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(baseline_index) = baseline_index.as_ref() {
            checks.extend(shell_export_package_baseline_index_entry_checks(
                baseline_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_export_package_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };

    let ready_delta = count_delta(candidate.ready_count, baseline.ready_count);
    let blocked_delta = count_delta(candidate.blocked_count, baseline.blocked_count);
    let rejected_delta = count_delta(candidate.rejected_count, baseline.rejected_count);
    let descriptor_delta = count_delta(candidate.descriptor_count, baseline.descriptor_count);
    let template_manifest_delta = count_delta(
        candidate.template_manifest_count,
        baseline.template_manifest_count,
    );
    let runbook_entry_delta =
        count_delta(candidate.runbook_entry_count, baseline.runbook_entry_count);

    let status = if !comparable {
        StudioShellExportPackageComparisonStatus::Incomparable
    } else if export_package_status_score(candidate.status)
        < export_package_status_score(baseline.status)
        || ready_delta < 0
        || blocked_delta > 0
        || rejected_delta > 0
        || descriptor_delta < 0
        || template_manifest_delta < 0
        || runbook_entry_delta < 0
        || entries.iter().any(|entry| {
            matches!(
                entry.change,
                StudioShellExportPackageComparisonChange::Regressed
                    | StudioShellExportPackageComparisonChange::Removed
                    | StudioShellExportPackageComparisonChange::Changed
            )
        })
    {
        StudioShellExportPackageComparisonStatus::Regressed
    } else if export_package_status_score(candidate.status)
        > export_package_status_score(baseline.status)
        || ready_delta > 0
        || blocked_delta < 0
        || rejected_delta < 0
        || descriptor_delta > 0
        || template_manifest_delta > 0
        || runbook_entry_delta > 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellExportPackageComparisonChange::Improved)
    {
        StudioShellExportPackageComparisonStatus::Improved
    } else {
        StudioShellExportPackageComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellExportPackageComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellExportPackageComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| {
                matches!(
                    entry.change,
                    StudioShellExportPackageComparisonChange::Regressed
                        | StudioShellExportPackageComparisonChange::Removed
                        | StudioShellExportPackageComparisonChange::Changed
                )
            })
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonStatus::Improved
        | StudioShellExportPackageComparisonStatus::Unchanged => None,
    };

    StudioShellExportPackageComparisonReport {
        schema_id: SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_id: baseline_identity.map(|identity| identity.baseline_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_package_path: baseline_identity.map(|identity| identity.package_path.clone()),
        baseline_index_schema: baseline_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: baseline_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_baseline_id: baseline_index
            .as_ref()
            .and_then(|context| context.index.default_baseline_id.clone()),
        baseline_index_selected_baseline_id: baseline_index
            .as_ref()
            .map(|context| context.entry.baseline_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
        baseline_package_id: baseline.package_id.clone(),
        candidate_package_id: candidate.package_id.clone(),
        baseline_manifest_id: baseline.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest_id.clone(),
        baseline_project_id: baseline.project_id.clone(),
        candidate_project_id: candidate.project_id.clone(),
        baseline_project_revision: baseline.project_revision,
        candidate_project_revision: candidate.project_revision,
        baseline_status: baseline.status,
        candidate_status: candidate.status,
        status,
        issue_code,
        baseline_ready_count: baseline.ready_count,
        candidate_ready_count: candidate.ready_count,
        ready_delta,
        baseline_blocked_count: baseline.blocked_count,
        candidate_blocked_count: candidate.blocked_count,
        blocked_delta,
        baseline_rejected_count: baseline.rejected_count,
        candidate_rejected_count: candidate.rejected_count,
        rejected_delta,
        baseline_descriptor_count: baseline.descriptor_count,
        candidate_descriptor_count: candidate.descriptor_count,
        descriptor_delta,
        baseline_template_manifest_count: baseline.template_manifest_count,
        candidate_template_manifest_count: candidate.template_manifest_count,
        template_manifest_delta,
        baseline_runbook_entry_count: baseline.runbook_entry_count,
        candidate_runbook_entry_count: candidate.runbook_entry_count,
        runbook_entry_delta,
        checks,
        entries,
    }
}

fn shell_export_package_comparison_checks(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_schema",
        baseline.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "baseline export-package schema id is supported",
        "baseline export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_schema",
        candidate.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "candidate export-package schema id is supported",
        "candidate export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_source_schemas",
        baseline.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && baseline.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "baseline source schemas are supported",
        "baseline source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_source_schemas",
        candidate.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && candidate.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "candidate source schemas are supported",
        "candidate source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.project_id",
        baseline.project_id == candidate.project_id,
        "baseline and candidate project ids match",
        "baseline and candidate project ids differ",
        "studio.issue.shell_export_package_project_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.execution_policy",
        baseline.execution_policy == candidate.execution_policy
            && baseline.execution_policy == "not_executed.review_only",
        "baseline and candidate use review-only execution policy",
        "baseline and candidate execution policies differ or are executable",
        "studio.issue.shell_export_package_execution_policy_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.authority",
        baseline.command_session_authority == candidate.command_session_authority
            && baseline.command_session_authority == "rusty.manifold"
            && baseline.install_launch_evidence_authority
                == candidate.install_launch_evidence_authority
            && baseline.install_launch_evidence_authority == "rusty.hostess"
            && baseline.studio_role == candidate.studio_role
            && baseline.studio_role == "authoring.export_planning",
        "baseline and candidate keep Manifold/Hostess/Studio authority",
        "baseline and candidate authority fields differ or drifted",
        "studio.issue.shell_export_package_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate prohibited actions match",
        "baseline and candidate prohibited actions differ",
        "studio.issue.shell_export_package_prohibited_actions_mismatch",
    );
    checks
}

fn shell_export_package_baseline_identity_checks(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_export_package_baseline_identity_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package_schema",
        baseline_identity.package_schema == baseline.schema_id,
        "baseline identity names the loaded package schema",
        "baseline identity does not name the loaded package schema",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package",
        baseline_identity.package_id == baseline.package_id
            && baseline_identity.manifest_id == baseline.manifest_id,
        "baseline identity package ids match the loaded package",
        "baseline identity package ids differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_project",
        baseline_identity.project_id == baseline.project_id
            && baseline_identity.project_revision == baseline.project_revision,
        "baseline identity project metadata matches the loaded package",
        "baseline identity project metadata differs from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_status_counts",
        baseline_identity.status == baseline.status
            && baseline_identity.ready_count == baseline.ready_count
            && baseline_identity.blocked_count == baseline.blocked_count
            && baseline_identity.rejected_count == baseline.rejected_count
            && baseline_identity.descriptor_count == baseline.descriptor_count
            && baseline_identity.template_manifest_count == baseline.template_manifest_count
            && baseline_identity.runbook_entry_count == baseline.runbook_entry_count
            && baseline_identity.target_count == baseline.target_summaries.len(),
        "baseline identity review counts match the loaded package",
        "baseline identity review counts differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_authority",
        baseline_identity.execution_policy == baseline.execution_policy
            && baseline_identity.review_owner == baseline.review_owner
            && baseline_identity.command_session_authority == baseline.command_session_authority
            && baseline_identity.install_launch_evidence_authority
                == baseline.install_launch_evidence_authority
            && baseline_identity.studio_role == baseline.studio_role
            && string_set(&baseline_identity.prohibited_actions)
                == string_set(&baseline.prohibited_actions),
        "baseline identity authority fields match the loaded package",
        "baseline identity authority fields differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    checks
}

fn shell_export_package_baseline_index_entry_checks(
    context: &ShellExportPackageBaselineIndexComparisonContext<'_>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let expected_manifest_path = context
        .baseline_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.baseline_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA,
        "baseline index schema id is supported",
        "baseline index schema id is unsupported",
        "studio.issue.shell_export_package_baseline_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_selected_baseline",
        entry.baseline_id == baseline_identity.baseline_id,
        "baseline index selected entry matches the loaded baseline identity",
        "baseline index selected entry differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline index entry records the selected baseline manifest path",
        "baseline index entry is missing or mismatches the selected manifest path",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package_path",
        entry.package_path == baseline_identity.package_path,
        "baseline index entry package path matches the loaded baseline identity",
        "baseline index entry package path differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package",
        entry.package_schema == baseline_identity.package_schema
            && entry.package_id == baseline_identity.package_id
            && entry.manifest_id == baseline_identity.manifest_id,
        "baseline index entry package ids match the loaded baseline identity",
        "baseline index entry package ids differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_project",
        entry.project_id == baseline_identity.project_id
            && entry.project_revision == baseline_identity.project_revision,
        "baseline index entry project metadata matches the loaded baseline identity",
        "baseline index entry project metadata differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_status_counts",
        entry.status == baseline_identity.status
            && entry.ready_count == baseline_identity.ready_count
            && entry.blocked_count == baseline_identity.blocked_count
            && entry.rejected_count == baseline_identity.rejected_count
            && entry.descriptor_count == baseline_identity.descriptor_count
            && entry.template_manifest_count == baseline_identity.template_manifest_count
            && entry.runbook_entry_count == baseline_identity.runbook_entry_count
            && entry.target_count == baseline_identity.target_count,
        "baseline index entry review counts match the loaded baseline identity",
        "baseline index entry review counts differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    checks
}

fn shell_export_package_comparison_entries(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioShellExportPackageComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let graph_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|graph_id| (*graph_id).to_string())
        .collect::<BTreeSet<_>>();

    graph_ids
        .into_iter()
        .map(|graph_id| {
            shell_export_package_comparison_entry(
                &graph_id,
                baseline_entries.get(graph_id.as_str()).copied(),
                candidate_entries.get(graph_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_export_package_comparison_entry(
    graph_id: &str,
    baseline: Option<&StudioShellExportPackageEntry>,
    candidate: Option<&StudioShellExportPackageEntry>,
) -> StudioShellExportPackageComparisonEntry {
    let baseline_score = baseline.map(|entry| export_package_status_score(entry.status));
    let candidate_score = candidate.map(|entry| export_package_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellExportPackageComparisonChange::Added,
        (Some(_), None) => StudioShellExportPackageComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => StudioShellExportPackageComparisonChange::Improved,
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellExportPackageComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.consumer_id != candidate.consumer_id
                || baseline.issue_code != candidate.issue_code
                || baseline.descriptor.is_some() != candidate.descriptor.is_some()
                || baseline.template_manifest.is_some()
                    != candidate.template_manifest.is_some()
                || baseline.runbook_cli_request.is_empty()
                    != candidate.runbook_cli_request.is_empty() =>
        {
            StudioShellExportPackageComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellExportPackageComparisonChange::Unchanged,
        (None, None) => StudioShellExportPackageComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellExportPackageComparisonChange::Regressed
        | StudioShellExportPackageComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonChange::Added
        | StudioShellExportPackageComparisonChange::Improved
        | StudioShellExportPackageComparisonChange::Unchanged
        | StudioShellExportPackageComparisonChange::Changed => None,
    };

    StudioShellExportPackageComparisonEntry {
        graph_id: graph_id.to_string(),
        target_kind: candidate
            .map(|entry| entry.target_kind)
            .or_else(|| baseline.map(|entry| entry.target_kind)),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_consumer_id: baseline.map(|entry| entry.consumer_id.clone()),
        candidate_consumer_id: candidate.map(|entry| entry.consumer_id.clone()),
        baseline_descriptor_present: baseline
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        candidate_descriptor_present: candidate
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        baseline_template_manifest_present: baseline
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        candidate_template_manifest_present: candidate
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        baseline_runbook_cli_request_present: baseline
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        candidate_runbook_cli_request_present: candidate
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn export_package_status_score(status: StudioShellExportPackageStatus) -> isize {
    match status {
        StudioShellExportPackageStatus::Rejected => 0,
        StudioShellExportPackageStatus::Blocked => 1,
        StudioShellExportPackageStatus::Ready => 2,
    }
}
