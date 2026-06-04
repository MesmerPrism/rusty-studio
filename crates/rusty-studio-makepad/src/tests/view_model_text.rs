use super::*;

#[test]
fn palette_lines_render_catalog_and_host_profiles() {
    let root = temp_root("palette-lines");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let package_lines = catalog_package_lines(&model);
    assert!(package_lines.contains("package.synthetic [selected; 1 module(s)]"));
    assert!(package_lines.contains("module.synthetic_provider"));
    assert!(package_lines.contains("packages/synthetic/manifests/package.manifold.json"));

    let profile_lines = host_profile_lines(&model);
    assert!(profile_lines.contains("host_run.profile.desktop [target]"));
    assert!(profile_lines.contains("host: host.desktop"));
    assert!(profile_lines.contains("host_run.profile.headset [available]"));

    let shell_preview = shell_preview_lines(&model);
    assert!(shell_preview.contains("studio.graph.makepad_edit [exported]"));
    assert!(shell_preview.contains("Shell descriptor exported"));
    assert!(shell_preview.contains("descriptor: studio.shell_descriptor.studio.graph.makepad_edit"));
    assert!(shell_preview.contains("shell: shell.synthetic.operator / Shell"));
    assert!(shell_preview.contains("target: host_run.profile.desktop / desktop"));
    assert!(shell_preview
        .contains("graph: 1 package(s), 0 module(s), 0 stream binding(s), 0 command binding(s)"));
    assert!(shell_preview.contains("descriptor validation: pass"));

    let shell_routes = shell_route_lines(&model);
    assert!(shell_routes.contains("host: host.desktop"));
    assert!(shell_routes.contains("app: app.host_shell.desktop"));
    assert!(shell_routes.contains("install: install.local_process"));
    assert!(shell_routes.contains("launch: launch.local_process"));
    assert!(shell_routes.contains("command: bridge.local_cli"));
    assert!(shell_routes.contains("evidence: evidence.filesystem"));

    let shell_template = shell_template_lines(&model);
    assert!(shell_template.contains("template: studio.shell_template.studio.graph.makepad_edit"));
    assert!(shell_template
        .contains("path: shells/desktop/studio.graph.makepad_edit.shell-template.json"));
    assert!(shell_template
        .contains("descriptor: descriptors/studio.graph.makepad_edit.shell-descriptor.json"));
    assert!(shell_template
        .contains("authority: rusty.manifold / rusty.hostess / authoring.export_planning"));

    assert_eq!(selected_node_line(&model), "Package / package");
    let detail_lines = selected_node_detail_lines(&model);
    assert!(detail_lines.contains("node: node.package.synthetic"));
    assert!(detail_lines.contains("ref: package.synthetic [resolved]"));
    assert!(detail_lines.contains("module.synthetic_provider"));
    let layout = layout_lines(&model.graphs[0]);
    assert!(layout.contains("studio.layout.makepad_edit / studio.canvas.logical_2d"));
    assert!(layout.contains("node.shell.operator @ 320,40 180x72"));
    assert!(layout.contains("edge.shell_host route: direct"));
    let canvas = graph_canvas_model(&model, &model.graphs[0]);
    assert_eq!(canvas.layout_id, "studio.layout.makepad_edit");
    assert_eq!(canvas.nodes.len(), 3);
    assert_eq!(canvas.edges.len(), 1);
    assert!(canvas
        .nodes
        .iter()
        .any(|node| node.node_id == "node.package.synthetic" && node.selected));
    assert!(canvas
        .edges
        .iter()
        .any(|edge| edge.edge_id == "edge.shell_host" && edge.selected));
    let canvas_rect = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(840.0, 220.0),
    };
    let canvas_viewport = CanvasViewport::for_rect(
        canvas_rect,
        canvas.logical_bounds().expect("canvas logical bounds"),
    );
    let host_node = canvas
        .nodes
        .iter()
        .find(|node| node.node_id == "node.host.profile")
        .expect("host node");
    let shell_node = canvas
        .nodes
        .iter()
        .find(|node| node.node_id == "node.shell.operator")
        .expect("shell node");
    assert_eq!(
        canvas.hit_test_abs(canvas_rect, canvas_viewport.node_center(host_node)),
        Some(StudioGraphCanvasHit::Node("node.host.profile".to_string()))
    );
    let shell_center = canvas_viewport.node_center(shell_node);
    let host_center = canvas_viewport.node_center(host_node);
    let edge_midpoint = dvec2(
        (shell_center.x + host_center.x) * 0.5,
        (shell_center.y + host_center.y) * 0.5,
    );
    assert_eq!(
        canvas.hit_test_abs(canvas_rect, edge_midpoint),
        Some(StudioGraphCanvasHit::Edge("edge.shell_host".to_string()))
    );
    assert_eq!(
        selected_edge_line(&model),
        "edge.shell_host [shell_targets_host_profile]"
    );
    let edge_details = selected_edge_detail_lines(&model);
    assert!(edge_details.contains("status: endpoints_resolved"));
    assert!(edge_details.contains("source: node.shell.operator / operator_shell"));
    assert!(edge_details.contains("target: node.host.profile / host_profile"));
}

#[test]
fn canvas_selection_uses_shared_view_model_route() {
    let root = temp_root("canvas-selection-route");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let selected_node_model = canvas_selection_view_model_for_project_source(
        &project_path,
        &model,
        0,
        Some("node.host.profile"),
        model.selected_edge_id.as_deref(),
    )
    .expect("select host node through shared view model");
    assert_eq!(
        selected_node_model.requested_node_id.as_deref(),
        Some("node.host.profile")
    );
    assert_eq!(
        selected_node_model.selected_node_id.as_deref(),
        Some("node.host.profile")
    );
    assert_eq!(
        selected_node_model.selected_edge_id.as_deref(),
        Some("edge.shell_host")
    );

    let selected_edge_model = canvas_selection_view_model_for_project_source(
        &project_path,
        &selected_node_model,
        0,
        selected_node_model.selected_node_id.as_deref(),
        Some("edge.shell_host"),
    )
    .expect("select edge through shared view model");
    assert_eq!(
        selected_edge_model.requested_edge_id.as_deref(),
        Some("edge.shell_host")
    );
    assert_eq!(
        selected_edge_model.selected_node_id.as_deref(),
        Some("node.host.profile")
    );
    assert_eq!(
        selected_edge_model.selected_edge_id.as_deref(),
        Some("edge.shell_host")
    );
}

#[test]
fn validation_issue_lines_render_failed_checks() {
    let root = temp_root("validation-issue-lines");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    let mut project = editable_project();
    project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.missing".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.missing".to_string(),
        label: "Missing Module".to_string(),
    });
    save_project(&project_path, &project).expect("save invalid project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let issue_lines = validation_issue_lines(&model);
    assert!(issue_lines.contains("studio.check.graph.studio.graph.makepad_edit.package_refs"));
    assert!(issue_lines.contains("studio.check.graph.studio.graph.makepad_edit.module_refs"));
    assert!(issue_lines.contains("studio.issue.package_reference_missing"));
    assert!(issue_lines.contains("selected graph: studio.graph.makepad_edit"));
    assert!(issue_lines.contains("refs: package.missing"));
    assert!(issue_lines.contains("package references missing from catalog"));

    let focus_line = issue_focus_line(&model);
    assert!(focus_line.contains("studio.check.graph.studio.graph.makepad_edit.package_refs"));
    assert!(focus_line.contains("node: node.package.synthetic"));
    assert!(focus_line.contains("ref: package.missing"));
    assert_eq!(selected_node_line(&model), "issue: Package / package");
    let detail_lines = selected_node_detail_lines(&model);
    assert!(detail_lines.contains("ref: package.missing [missing]"));
    assert!(detail_lines.contains("issues: 1"));

    let node_lines = node_lines(&model.graphs[0]);
    assert!(node_lines.contains("Package [package]"));
    assert!(node_lines.contains("issues: 1"));

    assert_eq!(
        next_issue_check_id(&model),
        Some("studio.check.graph.studio.graph.makepad_edit.module_refs")
    );

    let requested_model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        Some("studio.check.graph.studio.graph.makepad_edit.module_refs"),
        None,
        None,
    )
    .expect("load requested issue view model");
    assert_eq!(requested_model.selected_issue_index, Some(1));
    let requested_focus_line = issue_focus_line(&requested_model);
    assert!(requested_focus_line
        .contains("#2 studio.check.graph.studio.graph.makepad_edit.module_refs"));
    assert!(requested_focus_line.contains("node: node.module.missing"));
    assert_eq!(
        selected_node_line(&requested_model),
        "issue: Missing Module / module"
    );
    assert_eq!(
        next_issue_check_id(&requested_model),
        Some("studio.check.graph.studio.graph.makepad_edit.package_refs")
    );

    let requested_node_model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        None,
        Some("node.host.profile"),
        None,
    )
    .expect("load requested node view model");
    let requested_node_details = selected_node_detail_lines(&requested_node_model);
    assert_eq!(
        selected_node_line(&requested_node_model),
        "Host / host_profile"
    );
    assert!(requested_node_details.contains("host: host.desktop"));
    assert!(requested_node_details.contains("routes: install.local_process / launch.local_process"));
    assert_eq!(
        next_node_id(&requested_node_model),
        Some("node.shell.operator")
    );

    let requested_edge_model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        None,
        None,
        Some("edge.shell_host"),
    )
    .expect("load requested edge view model");
    assert_eq!(
        selected_edge_line(&requested_edge_model),
        "edge.shell_host [shell_targets_host_profile]"
    );
    let requested_edge_details = selected_edge_detail_lines(&requested_edge_model);
    assert!(requested_edge_details.contains("status: endpoints_resolved"));
    assert!(requested_edge_details.contains("source: node.shell.operator / operator_shell"));
    assert!(requested_edge_details.contains("target: node.host.profile / host_profile"));
    assert_eq!(next_edge_id(&requested_edge_model), Some("edge.shell_host"));

    let mut generated_layout_model = requested_edge_model.clone();
    generated_layout_model.graphs[0].layout = None;
    let generated_canvas =
        graph_canvas_model(&generated_layout_model, &generated_layout_model.graphs[0]);
    assert_eq!(
        generated_canvas.layout_id,
        "studio.layout.generated_readonly"
    );
    assert_eq!(
        generated_canvas.coordinate_space,
        "studio.canvas.generated_2d"
    );
    assert_eq!(generated_canvas.nodes[0].x, 40);
    assert_eq!(generated_canvas.nodes[0].width, 220);
}
