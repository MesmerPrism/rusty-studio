use super::super::super::*;

#[test]
fn shell_handoff_acceptance_baseline_writes_durable_artifact() {
    let root = temp_root("shell-handoff-acceptance-baseline");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, baseline, index, output_path, baseline_path, index_path, bundle_root) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");

    assert_eq!(
        output_path,
        shell_handoff_acceptance_checklist_output_path(&project_path)
    );
    assert!(output_path.is_file());
    assert_eq!(
        baseline_path,
        shell_handoff_acceptance_baseline_manifest_output_path(&project_path)
    );
    assert!(baseline_path.is_file());
    assert_eq!(
        index_path,
        shell_handoff_acceptance_baseline_index_output_path(&project_path)
    );
    assert!(index_path.is_file());
    assert_eq!(
        baseline.schema_id,
        "rusty.studio.shell_handoff_acceptance_baseline_manifest.v1"
    );
    assert_eq!(
        baseline.baseline_id,
        "studio.project.makepad_edit.rev1.ready"
    );
    assert_eq!(
        baseline.label,
        "studio.project.makepad_edit revision 1 ready acceptance baseline"
    );
    assert_eq!(baseline.checklist_path, output_path.display().to_string());
    assert_eq!(baseline.summary.project_id, "studio.project.makepad_edit");
    assert_eq!(baseline.summary.project_revision, 1);
    assert_eq!(
        report.schema_id,
        "rusty.studio.shell_handoff_acceptance_checklist.v1"
    );
    assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.blocked_count, 0);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(
        index.schema_id,
        "rusty.studio.shell_handoff_acceptance_baseline_index.v1"
    );
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(index.baseline_count, 1);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 0);
    assert_eq!(index.rejected_baseline_count, 0);
    assert_eq!(index.entries.len(), 1);
    assert_eq!(index.entries[0].baseline_id, baseline.baseline_id);
    assert_eq!(
        index.entries[0].baseline_manifest_path.as_deref(),
        Some(baseline_path.display().to_string().as_str())
    );
    assert_eq!(
        index.entries[0].checklist_path,
        output_path.display().to_string()
    );
    let written = std::fs::read_to_string(&output_path).expect("read acceptance baseline");
    assert!(written.contains("\"$schema\": \"rusty.studio.shell_handoff_acceptance_checklist.v1\""));
    let manifest_text = std::fs::read_to_string(&baseline_path).expect("read baseline identity");
    assert!(manifest_text
        .contains("\"$schema\": \"rusty.studio.shell_handoff_acceptance_baseline_manifest.v1\""));
    let index_text = std::fs::read_to_string(&index_path).expect("read baseline index");
    assert!(index_text
        .contains("\"$schema\": \"rusty.studio.shell_handoff_acceptance_baseline_index.v1\""));
    let status = shell_handoff_acceptance_baseline_status(
        &report,
        &baseline,
        &index,
        &output_path,
        &baseline_path,
        &index_path,
        &bundle_root,
    );
    assert!(status.contains("acceptance baseline written"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains(&format!("identity: {}", baseline_path.display())));
    assert!(status.contains(&format!("checklist: {}", output_path.display())));
    assert!(status.contains(&format!("index: {}", index_path.display())));
    assert!(status.contains(
            "baseline selection selected; requested none; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
    assert!(status.contains("selected yes; default yes"));
    assert!(
        status.contains("baseline index slots 1; default studio.project.makepad_edit.rev1.ready")
    );
    assert!(status.contains("handoff acceptance ready"));
    assert!(status.contains("ready 1; blocked 0; rejected 0"));
}
