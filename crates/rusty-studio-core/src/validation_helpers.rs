use super::*;

pub(crate) fn push_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    push_contextual_check(
        checks,
        check_id,
        passed,
        pass_evidence,
        fail_evidence,
        issue_code,
        None,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
}

pub(crate) fn push_contextual_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
    graph_id: Option<&str>,
    node_ids: Vec<String>,
    edge_ids: Vec<String>,
    reference_ids: Vec<String>,
) {
    checks.push(StudioValidationCheck {
        check_id: check_id.to_string(),
        status: if passed {
            StudioValidationStatus::Pass
        } else {
            StudioValidationStatus::Fail
        },
        evidence: if passed { pass_evidence } else { fail_evidence }.to_string(),
        issue_code: (!passed).then(|| issue_code.to_string()),
        graph_id: graph_id.map(str::to_string),
        node_ids,
        edge_ids,
        reference_ids,
    });
}

pub(crate) fn first_failed_validation_check_issue_code(
    checks: &[StudioValidationCheck],
) -> Option<String> {
    checks.iter().find_map(failed_issue_code)
}

pub(crate) fn first_failed_issue_code(report: &StudioValidationReport) -> Option<String> {
    first_failed_validation_check_issue_code(&report.checks)
}

pub(crate) fn first_failed_check_issue_code(
    report: &StudioShellDescriptorValidationReport,
) -> Option<String> {
    first_failed_validation_check_issue_code(&report.checks)
}

pub(crate) fn first_failed_shell_artifact_manifest_issue_code(
    report: &StudioShellArtifactManifestValidationReport,
) -> Option<String> {
    first_failed_validation_check_issue_code(&report.checks)
}

pub(crate) fn first_failed_shell_template_index_issue_code(
    report: &StudioShellTemplateIndexValidationReport,
) -> Option<String> {
    first_failed_validation_check_issue_code(&report.checks)
}

pub(crate) fn first_failed_shell_bundle_validation_issue_code(
    report: &StudioShellBundleValidationReport,
) -> Option<String> {
    first_failed_validation_check_issue_code(&report.checks)
}

fn failed_issue_code(check: &StudioValidationCheck) -> Option<String> {
    (check.status == StudioValidationStatus::Fail)
        .then(|| check.issue_code.clone())
        .flatten()
}
