use super::super::super::super::*;

pub(crate) fn shell_export_package_baseline_index_status(
    index: &StudioShellExportPackageBaselineIndex,
    index_path: &Path,
) -> String {
    let default = index.default_baseline_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let packages = if index.package_ids.is_empty() {
        "none".to_string()
    } else {
        index.package_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_export_package_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; package {}; manifest {}; issue {}",
                entry.baseline_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                entry.descriptor_count,
                entry.template_manifest_count,
                entry.package_path,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package baseline index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  packages: {}\n  manifests: {}\n  entries:\n  {}",
        index.baseline_count,
        default,
        index.ready_baseline_count,
        index.blocked_baseline_count,
        index.rejected_baseline_count,
        index_path.display(),
        projects,
        packages,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
