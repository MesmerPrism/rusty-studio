use super::*;

#[test]
fn default_project_working_copy_uses_ignored_sibling_dir() {
    let root = temp_root("default-project-working-copy");
    let source_path = root.join("examples/synthetic-studio-project.json");
    write_fixture(
        &source_path,
        r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
    );

    let working_path =
        default_project_working_copy_path(&source_path).expect("copy default project");

    assert_eq!(
        working_path,
        root.join("examples-working/synthetic-studio-project.json")
    );
    assert_eq!(
        std::fs::read_to_string(&working_path).expect("read working copy"),
        std::fs::read_to_string(&source_path).expect("read source")
    );
}

#[test]
fn requested_synthetic_example_uses_ignored_working_copy() {
    let root = temp_root("requested-synthetic-example-working-copy");
    let source_path = root.join("examples/synthetic-studio-project.json");
    write_fixture(
        &source_path,
        r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
    );

    let session_path =
        project_path_for_mutable_session(source_path.clone()).expect("resolve session path");

    assert_eq!(
        session_path,
        root.join("examples-working/synthetic-studio-project.json")
    );
    assert_eq!(
        std::fs::read_to_string(&session_path).expect("read working copy"),
        std::fs::read_to_string(&source_path).expect("read source")
    );
}

#[test]
fn requested_non_default_project_keeps_original_path() {
    let root = temp_root("requested-non-default-project");
    let project_path = root.join("project.json");
    write_fixture(
        &project_path,
        r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
    );

    let session_path =
        project_path_for_mutable_session(project_path.clone()).expect("resolve session path");

    assert_eq!(session_path, project_path);
}
