$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name,
        [Parameter(Mandatory=$true)]
        [string]$File,
        [string[]]$Arguments = @()
    )

    & $File @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $RepoRoot
try {
    $EditOutput = Join-Path $RepoRoot "target\studio-edit-retarget-headset.json"
    $DiagnosticProjectOutput = Join-Path $RepoRoot "target\studio-view-model-diagnostic-invalid-project.json"
    $LayoutDiagnosticProjectOutput = Join-Path $RepoRoot "target\studio-layout-diagnostic-project.json"
    $AddModuleOutput = Join-Path $RepoRoot "target\studio-edit-add-module.json"
    $AddPaletteModuleOutput = Join-Path $RepoRoot "target\studio-edit-add-palette-module.json"
    $AddSelectedPackageModuleOutput = Join-Path $RepoRoot "target\studio-edit-add-selected-package-module.json"
    $RemoveModuleOutput = Join-Path $RepoRoot "target\studio-edit-remove-module.json"
    $AddBindingOutput = Join-Path $RepoRoot "target\studio-edit-add-binding.json"
    $RemoveBindingOutput = Join-Path $RepoRoot "target\studio-edit-remove-binding.json"
    $ShellOutput = Join-Path $RepoRoot "target\studio-shell-descriptor-desktop.json"
    $ShellArtifactsDir = Join-Path $RepoRoot "target\studio-shells"
    $ShellTemplatesDir = Join-Path $RepoRoot "target\studio-shell-templates"
    $SelectedShellBundleRoot = Join-Path $RepoRoot "target\studio-selected-shell"
    $SelectedShellBundleDir = Join-Path $SelectedShellBundleRoot "studio.graph.synthetic_wave_desktop"
    $SelectedPhoneShellBundleDir = Join-Path $SelectedShellBundleRoot "studio.graph.synthetic_wave_phone"
    $SelectedQuestShellBundleDir = Join-Path $SelectedShellBundleRoot "studio.graph.synthetic_wave_headset"
    New-Item -ItemType Directory -Path (Split-Path $EditOutput) -Force | Out-Null
    foreach ($GeneratedOutput in @($EditOutput, $DiagnosticProjectOutput, $LayoutDiagnosticProjectOutput, $AddModuleOutput, $AddPaletteModuleOutput, $AddSelectedPackageModuleOutput, $RemoveModuleOutput, $AddBindingOutput, $RemoveBindingOutput, $ShellOutput)) {
        if (Test-Path $GeneratedOutput) {
            Remove-Item -LiteralPath $GeneratedOutput
        }
    }
    foreach ($GeneratedDir in @($ShellArtifactsDir, $ShellTemplatesDir, $SelectedShellBundleDir, $SelectedPhoneShellBundleDir, $SelectedQuestShellBundleDir)) {
        if (Test-Path $GeneratedDir) {
            Remove-Item -Recurse -Force -LiteralPath $GeneratedDir
        }
    }

    Invoke-Checked "cargo fmt" "cargo" @(
        "fmt",
        "-p",
        "rusty-studio-model",
        "-p",
        "rusty-studio-core",
        "-p",
        "rusty-studio-cli",
        "-p",
        "rusty-studio-makepad",
        "-p",
        "rusty-studio-desktop-shell",
        "--check"
    )
    Invoke-Checked "cargo test" "cargo" @("test", "--workspace")
    Invoke-Checked "makepad viewer check" "cargo" @(
        "check",
        "-p",
        "rusty-studio-makepad"
    )
    Invoke-Checked "desktop shell check" "cargo" @(
        "check",
        "-p",
        "rusty-studio-desktop-shell"
    )
    Invoke-Checked "studio validate" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        "examples\synthetic-studio-project.json"
    )
    Invoke-Checked "studio resolve" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "resolve",
        "--project",
        "examples\synthetic-studio-project.json"
    )
    Invoke-Checked "studio export plan" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "export-plan",
        "--project",
        "examples\synthetic-studio-project.json"
    )
    Invoke-Checked "studio view model" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "view-model",
        "--project",
        "examples\synthetic-studio-project.json"
    )
    $ViewModelOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop"
    if ($LASTEXITCODE -ne 0) {
        throw "studio view model palette check failed with exit code $LASTEXITCODE"
    }
    $ViewModelText = $ViewModelOutput -join [Environment]::NewLine
    $ViewModel = $ViewModelText | ConvertFrom-Json
    if ($ViewModel.'$schema' -ne "rusty.studio.view_model.v1") {
        throw "view model schema mismatch"
    }
    if ($ViewModel.validation_issues.Count -ne 0) {
        throw "valid view model should not expose validation issues"
    }
    if ($null -ne $ViewModel.focused_issue) {
        throw "valid view model should not expose focused issue"
    }
    if ($null -ne $ViewModel.requested_issue_check_id) {
        throw "valid view model should not expose requested issue"
    }
    if ($null -ne $ViewModel.selected_issue_check_id) {
        throw "valid view model should not expose selected issue"
    }
    if ($null -ne $ViewModel.issue_selection_code) {
        throw "valid view model should not expose issue selection code"
    }
    if ($null -eq $ViewModel.selected_node) {
        throw "valid view model should expose selected node inspector"
    }
    if ($ViewModel.selected_node_id -ne "node.package.synthetic_wave") {
        throw "valid view model selected node id mismatch"
    }
    if ($ViewModel.selected_node.reference_status -ne "resolved") {
        throw "valid view model selected package should resolve"
    }
    if (@($ViewModel.selected_node.package_module_ids) -notcontains "module.synthetic_wave_provider") {
        throw "valid view model selected package missing provider module detail"
    }
    if ($null -eq $ViewModel.selected_edge) {
        throw "valid view model should expose selected edge inspector"
    }
    if ($ViewModel.selected_edge_id -ne "edge.package_provider") {
        throw "valid view model selected edge id mismatch"
    }
    if ($ViewModel.selected_edge.endpoint_status -ne "endpoints_resolved") {
        throw "valid view model selected edge endpoints should resolve"
    }
    if ($ViewModel.selected_edge.source_reference_id -ne "package.synthetic_wave") {
        throw "valid view model selected edge source reference mismatch"
    }
    if ($ViewModel.selected_edge.target_reference_id -ne "module.synthetic_wave_provider") {
        throw "valid view model selected edge target reference mismatch"
    }
    if ($null -eq $ViewModel.shell_preview) {
        throw "valid view model should expose selected graph shell preview"
    }
    if ($ViewModel.shell_preview.status -ne "exported") {
        throw "valid view model shell preview should be exported"
    }
    if ($null -ne $ViewModel.shell_preview.issue_code) {
        throw "valid view model shell preview should not expose an issue code"
    }
    if ($ViewModel.shell_preview.descriptor_id -ne "studio.shell_descriptor.studio.graph.synthetic_wave_desktop") {
        throw "valid view model shell preview descriptor id mismatch"
    }
    if ($ViewModel.shell_preview.descriptor_path -ne "descriptors/studio.graph.synthetic_wave_desktop.shell-descriptor.json") {
        throw "valid view model shell preview descriptor path mismatch"
    }
    if ($ViewModel.shell_preview.shell_id -ne "shell.synthetic_wave.desktop_operator") {
        throw "valid view model shell preview shell id mismatch"
    }
    if ($ViewModel.shell_preview.target_host_profile -ne "host_run.profile.desktop") {
        throw "valid view model shell preview target host mismatch"
    }
    if ($ViewModel.shell_preview.target_kind -ne "desktop") {
        throw "valid view model shell preview target kind mismatch"
    }
    if ($ViewModel.shell_preview.package_count -ne 1) {
        throw "valid view model shell preview package count mismatch"
    }
    if ($ViewModel.shell_preview.module_count -ne 2) {
        throw "valid view model shell preview module count mismatch"
    }
    if ($ViewModel.shell_preview.stream_binding_count -ne 1) {
        throw "valid view model shell preview stream binding count mismatch"
    }
    if ($ViewModel.shell_preview.command_binding_count -ne 0) {
        throw "valid view model shell preview command binding count mismatch"
    }
    if ($ViewModel.shell_preview.descriptor_validation_status -ne "pass") {
        throw "valid view model shell preview descriptor validation mismatch"
    }
    if ($ViewModel.shell_preview.template_id -ne "studio.shell_template.studio.graph.synthetic_wave_desktop") {
        throw "valid view model shell preview template id mismatch"
    }
    if ($ViewModel.shell_preview.template_path -ne "shells/desktop/studio.graph.synthetic_wave_desktop.shell-template.json") {
        throw "valid view model shell preview template path mismatch"
    }
    if ($ViewModel.shell_preview.template_descriptor_path -ne "descriptors/studio.graph.synthetic_wave_desktop.shell-descriptor.json") {
        throw "valid view model shell preview template descriptor path mismatch"
    }
    if ($ViewModel.shell_preview.runtime_command_authority -ne "rusty.manifold") {
        throw "valid view model shell preview command authority mismatch"
    }
    if ($ViewModel.shell_preview.runtime_host_authority -ne "rusty.hostess") {
        throw "valid view model shell preview host authority mismatch"
    }
    if ($ViewModel.shell_preview.studio_role -ne "authoring.export_planning") {
        throw "valid view model shell preview Studio role mismatch"
    }
    $ViewModelDesktopGraph = $ViewModel.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $ViewModelDesktopGraph) {
        throw "view model missing desktop graph row"
    }
    if ($ViewModelDesktopGraph.validation_issue_count -ne 0) {
        throw "valid desktop graph row should have no validation issues"
    }
    if ($null -eq $ViewModelDesktopGraph.layout) {
        throw "valid desktop graph row should expose graph layout"
    }
    if ($ViewModelDesktopGraph.layout.layout_id -ne "studio.layout.synthetic_wave_desktop") {
        throw "valid desktop graph layout id mismatch"
    }
    if ($ViewModelDesktopGraph.layout.coordinate_space -ne "studio.canvas.logical_2d") {
        throw "valid desktop graph layout coordinate space mismatch"
    }
    if ($ViewModelDesktopGraph.layout.node_count -ne 5) {
        throw "valid desktop graph layout node count mismatch"
    }
    if ($ViewModelDesktopGraph.layout.edge_count -ne 4) {
        throw "valid desktop graph layout edge count mismatch"
    }
    $LayoutProvider = $ViewModelDesktopGraph.layout.nodes | Where-Object { $_.node_id -eq "node.module.synthetic_wave_provider" } | Select-Object -First 1
    if ($null -eq $LayoutProvider) {
        throw "valid desktop graph layout missing provider node"
    }
    if ($LayoutProvider.x -ne 320 -or $LayoutProvider.y -ne 24 -or $LayoutProvider.width -ne 220 -or $LayoutProvider.height -ne 72) {
        throw "valid desktop graph layout provider box mismatch"
    }
    $LayoutShellEdge = $ViewModelDesktopGraph.layout.edges | Where-Object { $_.edge_id -eq "edge.shell_host" } | Select-Object -First 1
    if ($null -eq $LayoutShellEdge) {
        throw "valid desktop graph layout missing shell edge"
    }
    if ($LayoutShellEdge.route -ne "orthogonal") {
        throw "valid desktop graph layout shell edge route mismatch"
    }
    if ($ViewModel.catalog_package_count -lt 4) {
        throw "view model should expose at least four catalog packages"
    }
    if ($ViewModel.host_profile_count -ne 3) {
        throw "view model should expose desktop, phone, and headset profiles"
    }
    $SyntheticPackage = $ViewModel.catalog_packages | Where-Object { $_.package_id -eq "package.synthetic_wave" } | Select-Object -First 1
    if ($null -eq $SyntheticPackage) {
        throw "view model missing synthetic wave package palette row"
    }
    if (-not $SyntheticPackage.in_selected_graph) {
        throw "view model should mark synthetic wave package as selected"
    }
    $SyntheticModules = @($SyntheticPackage.module_ids)
    if ($SyntheticModules -notcontains "module.synthetic_wave_provider") {
        throw "view model missing synthetic wave provider module export"
    }
    $DesktopProfile = $ViewModel.host_profiles | Where-Object { $_.profile_id -eq "host_run.profile.desktop" } | Select-Object -First 1
    if ($null -eq $DesktopProfile) {
        throw "view model missing desktop host profile row"
    }
    if (-not $DesktopProfile.targets_selected_graph) {
        throw "view model should mark desktop host profile as selected target"
    }
    $HeadsetProfile = $ViewModel.host_profiles | Where-Object { $_.profile_id -eq "host_run.profile.headset" } | Select-Object -First 1
    if ($null -eq $HeadsetProfile) {
        throw "view model missing headset host profile row"
    }
    if ($HeadsetProfile.targets_selected_graph) {
        throw "view model should not mark headset profile as desktop target"
    }
    $RequestedNodeViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --node "node.host_profile.desktop"
    if ($LASTEXITCODE -ne 0) {
        throw "studio requested node view model failed with exit code $LASTEXITCODE"
    }
    $RequestedNodeViewText = $RequestedNodeViewOutput -join [Environment]::NewLine
    $RequestedNodeView = $RequestedNodeViewText | ConvertFrom-Json
    if ($RequestedNodeView.requested_node_id -ne "node.host_profile.desktop") {
        throw "requested node view model requested node mismatch"
    }
    if ($RequestedNodeView.selected_node_id -ne "node.host_profile.desktop") {
        throw "requested node view model selected node mismatch"
    }
    if ($RequestedNodeView.selected_node.kind -ne "host_profile") {
        throw "requested node view model selected node kind mismatch"
    }
    if ($RequestedNodeView.selected_node.host_profile.profile_id -ne "host_run.profile.desktop") {
        throw "requested node view model host profile id mismatch"
    }
    if ($RequestedNodeView.selected_node.host_profile.install_route -ne "install.local_process") {
        throw "requested node view model install route mismatch"
    }
    $MissingRequestedNodeViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --node "node.missing"
    if ($LASTEXITCODE -ne 0) {
        throw "studio missing requested node view model failed with exit code $LASTEXITCODE"
    }
    $MissingRequestedNodeViewText = $MissingRequestedNodeViewOutput -join [Environment]::NewLine
    $MissingRequestedNodeView = $MissingRequestedNodeViewText | ConvertFrom-Json
    if ($MissingRequestedNodeView.node_selection_code -ne "studio.issue.node_selection_missing") {
        throw "missing requested node view model should expose node selection code"
    }
    if ($MissingRequestedNodeView.selected_node_id -ne "node.package.synthetic_wave") {
        throw "missing requested node view model should fall back to deterministic selected node"
    }
    $RequestedEdgeViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --edge "edge.provider_to_processor"
    if ($LASTEXITCODE -ne 0) {
        throw "studio requested edge view model failed with exit code $LASTEXITCODE"
    }
    $RequestedEdgeViewText = $RequestedEdgeViewOutput -join [Environment]::NewLine
    $RequestedEdgeView = $RequestedEdgeViewText | ConvertFrom-Json
    if ($RequestedEdgeView.requested_edge_id -ne "edge.provider_to_processor") {
        throw "requested edge view model requested edge mismatch"
    }
    if ($RequestedEdgeView.selected_edge_id -ne "edge.provider_to_processor") {
        throw "requested edge view model selected edge mismatch"
    }
    if ($RequestedEdgeView.selected_edge.kind -ne "stream_binding") {
        throw "requested edge view model selected edge kind mismatch"
    }
    if ($RequestedEdgeView.selected_edge.binding_kind -ne "stream") {
        throw "requested edge view model binding kind mismatch"
    }
    if ($RequestedEdgeView.selected_edge.source_reference_id -ne "module.synthetic_wave_provider") {
        throw "requested edge view model source reference mismatch"
    }
    if ($RequestedEdgeView.selected_edge.target_reference_id -ne "module.synthetic_wave_processor") {
        throw "requested edge view model target reference mismatch"
    }
    $MissingRequestedEdgeViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --edge "edge.missing"
    if ($LASTEXITCODE -ne 0) {
        throw "studio missing requested edge view model failed with exit code $LASTEXITCODE"
    }
    $MissingRequestedEdgeViewText = $MissingRequestedEdgeViewOutput -join [Environment]::NewLine
    $MissingRequestedEdgeView = $MissingRequestedEdgeViewText | ConvertFrom-Json
    if ($MissingRequestedEdgeView.edge_selection_code -ne "studio.issue.edge_selection_missing") {
        throw "missing requested edge view model should expose edge selection code"
    }
    if ($MissingRequestedEdgeView.selected_edge_id -ne "edge.package_provider") {
        throw "missing requested edge view model should fall back to deterministic selected edge"
    }
    $DiagnosticProject = Get-Content -Raw -Path "examples\synthetic-studio-project.json" | ConvertFrom-Json
    $DiagnosticProject.graphs[0].nodes[0].reference_id = "package.missing"
    [System.IO.File]::WriteAllText(
        $DiagnosticProjectOutput,
        ($DiagnosticProject | ConvertTo-Json -Depth 100),
        [System.Text.UTF8Encoding]::new($false)
    )
    $DiagnosticViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project $DiagnosticProjectOutput --graph "studio.graph.synthetic_wave_desktop"
    if ($LASTEXITCODE -ne 0) {
        throw "studio diagnostic view model failed with exit code $LASTEXITCODE"
    }
    $DiagnosticViewText = $DiagnosticViewOutput -join [Environment]::NewLine
    $DiagnosticView = $DiagnosticViewText | ConvertFrom-Json
    if ($DiagnosticView.validation_status -ne "fail") {
        throw "diagnostic view model should fail validation"
    }
    if ($DiagnosticView.validation_issues.Count -lt 1) {
        throw "diagnostic view model should expose failed validation issues"
    }
    $PackageReferenceIssue = $DiagnosticView.validation_issues | Where-Object { $_.issue_code -eq "studio.issue.package_reference_missing" } | Select-Object -First 1
    if ($null -eq $PackageReferenceIssue) {
        throw "diagnostic view model missing package reference issue"
    }
    if ($PackageReferenceIssue.check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "diagnostic view model package issue check id mismatch"
    }
    if ($PackageReferenceIssue.graph_id -ne "studio.graph.synthetic_wave_desktop") {
        throw "diagnostic view model package issue graph id mismatch"
    }
    if (-not $PackageReferenceIssue.targets_selected_graph) {
        throw "diagnostic view model package issue should target selected graph"
    }
    if (@($PackageReferenceIssue.reference_ids) -notcontains "package.missing") {
        throw "diagnostic view model package issue missing affected reference id"
    }
    if ($PackageReferenceIssue.evidence -notlike "*package references missing from catalog*") {
        throw "diagnostic view model package issue evidence mismatch"
    }
    if ($null -eq $DiagnosticView.focused_issue) {
        throw "diagnostic view model missing focused issue"
    }
    if ($DiagnosticView.focused_issue.check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "diagnostic focused issue check id mismatch"
    }
    if ($DiagnosticView.focused_issue.graph_id -ne "studio.graph.synthetic_wave_desktop") {
        throw "diagnostic focused issue graph id mismatch"
    }
    if ($DiagnosticView.focused_issue.node_id -ne "node.package.synthetic_wave") {
        throw "diagnostic focused issue node id mismatch"
    }
    if ($DiagnosticView.focused_issue.reference_id -ne "package.missing") {
        throw "diagnostic focused issue reference id mismatch"
    }
    if ($DiagnosticView.selected_node_id -ne "node.package.synthetic_wave") {
        throw "diagnostic selected node should follow focused issue"
    }
    if ($DiagnosticView.selected_node.reference_status -ne "missing") {
        throw "diagnostic selected node should expose missing reference status"
    }
    if ($DiagnosticView.selected_node.validation_issue_count -lt 1) {
        throw "diagnostic selected node should expose validation issue count"
    }
    if ($DiagnosticView.selected_issue_check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "diagnostic selected issue check id mismatch"
    }
    if ($DiagnosticView.selected_issue_index -ne 0) {
        throw "diagnostic selected issue index mismatch"
    }
    $DiagnosticGraph = $DiagnosticView.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $DiagnosticGraph) {
        throw "diagnostic view model missing desktop graph row"
    }
    if ($DiagnosticGraph.validation_issue_count -lt 1) {
        throw "diagnostic desktop graph row should expose validation issue count"
    }
    $DiagnosticPackageNode = $DiagnosticGraph.node_rows | Where-Object { $_.node_id -eq "node.package.synthetic_wave" } | Select-Object -First 1
    if ($null -eq $DiagnosticPackageNode) {
        throw "diagnostic view model missing package node row"
    }
    if ($DiagnosticPackageNode.validation_issue_count -lt 1) {
        throw "diagnostic package node row should expose validation issue count"
    }
    $LayoutDiagnosticProject = Get-Content -Raw -Path "examples\synthetic-studio-project.json" | ConvertFrom-Json
    $LayoutDiagnosticProject.graphs[0].layout.nodes[0].node_id = "node.layout_missing"
    $LayoutDiagnosticProject.graphs[0].layout.nodes[1].width = 0
    $LayoutDiagnosticProject.graphs[0].layout.edges[0].edge_id = "edge.layout_missing"
    [System.IO.File]::WriteAllText(
        $LayoutDiagnosticProjectOutput,
        ($LayoutDiagnosticProject | ConvertTo-Json -Depth 100),
        [System.Text.UTF8Encoding]::new($false)
    )
    $LayoutDiagnosticViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project $LayoutDiagnosticProjectOutput --graph "studio.graph.synthetic_wave_desktop"
    if ($LASTEXITCODE -ne 0) {
        throw "studio layout diagnostic view model failed with exit code $LASTEXITCODE"
    }
    $LayoutDiagnosticViewText = $LayoutDiagnosticViewOutput -join [Environment]::NewLine
    $LayoutDiagnosticView = $LayoutDiagnosticViewText | ConvertFrom-Json
    if ($LayoutDiagnosticView.validation_status -ne "fail") {
        throw "layout diagnostic view model should fail validation"
    }
    $MissingLayoutNodeIssue = $LayoutDiagnosticView.validation_issues | Where-Object { $_.issue_code -eq "studio.issue.layout_node_missing" } | Select-Object -First 1
    if ($null -eq $MissingLayoutNodeIssue) {
        throw "layout diagnostic view model should expose missing layout node issue"
    }
    $MissingLayoutEdgeIssue = $LayoutDiagnosticView.validation_issues | Where-Object { $_.issue_code -eq "studio.issue.layout_edge_missing" } | Select-Object -First 1
    if ($null -eq $MissingLayoutEdgeIssue) {
        throw "layout diagnostic view model should expose missing layout edge issue"
    }
    $InvalidLayoutBoxIssue = $LayoutDiagnosticView.validation_issues | Where-Object { $_.issue_code -eq "studio.issue.invalid_layout_node_box" } | Select-Object -First 1
    if ($null -eq $InvalidLayoutBoxIssue) {
        throw "layout diagnostic view model should expose invalid layout box issue"
    }
    $LayoutDiagnosticGraph = $LayoutDiagnosticView.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $LayoutDiagnosticGraph.layout) {
        throw "layout diagnostic graph should still expose layout view"
    }
    $MissingLayoutNode = $LayoutDiagnosticGraph.layout.nodes | Where-Object { $_.node_id -eq "node.layout_missing" } | Select-Object -First 1
    if ($null -eq $MissingLayoutNode -or $MissingLayoutNode.validation_issue_count -lt 1) {
        throw "layout diagnostic missing node should carry issue count"
    }
    $MissingLayoutEdge = $LayoutDiagnosticGraph.layout.edges | Where-Object { $_.edge_id -eq "edge.layout_missing" } | Select-Object -First 1
    if ($null -eq $MissingLayoutEdge -or $MissingLayoutEdge.validation_issue_count -lt 1) {
        throw "layout diagnostic missing edge should carry issue count"
    }
    $RequestedDiagnosticViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project $DiagnosticProjectOutput --graph "studio.graph.synthetic_wave_desktop" --issue "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs"
    if ($LASTEXITCODE -ne 0) {
        throw "studio requested diagnostic view model failed with exit code $LASTEXITCODE"
    }
    $RequestedDiagnosticViewText = $RequestedDiagnosticViewOutput -join [Environment]::NewLine
    $RequestedDiagnosticView = $RequestedDiagnosticViewText | ConvertFrom-Json
    if ($RequestedDiagnosticView.requested_issue_check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "requested diagnostic view model requested issue mismatch"
    }
    if ($RequestedDiagnosticView.selected_issue_check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "requested diagnostic view model selected issue mismatch"
    }
    if ($null -ne $RequestedDiagnosticView.issue_selection_code) {
        throw "requested diagnostic view model should not expose issue selection code"
    }
    $MissingRequestedIssueViewOutput = & cargo run --quiet -p rusty-studio-cli -- view-model --project $DiagnosticProjectOutput --graph "studio.graph.synthetic_wave_desktop" --issue "studio.check.graph.studio.graph.synthetic_wave_desktop.missing"
    if ($LASTEXITCODE -ne 0) {
        throw "studio missing requested issue view model failed with exit code $LASTEXITCODE"
    }
    $MissingRequestedIssueViewText = $MissingRequestedIssueViewOutput -join [Environment]::NewLine
    $MissingRequestedIssueView = $MissingRequestedIssueViewText | ConvertFrom-Json
    if ($MissingRequestedIssueView.issue_selection_code -ne "studio.issue.validation_issue_selection_missing") {
        throw "missing requested issue view model should expose issue selection code"
    }
    if ($MissingRequestedIssueView.selected_issue_check_id -ne "studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs") {
        throw "missing requested issue view model should fall back to deterministic focused issue"
    }
    Invoke-Checked "studio view model selected graph" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "view-model",
        "--project",
        "examples\synthetic-studio-project.json",
        "--graph",
        "studio.graph.synthetic_wave_headset"
    )
    Invoke-Checked "studio view model missing graph diagnostic" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "view-model",
        "--project",
        "examples\synthetic-studio-project.json",
        "--graph",
        "studio.graph.missing"
    )
    Invoke-Checked "studio retarget host" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "retarget-host",
        "--project",
        "examples\synthetic-studio-project.json",
        "--graph",
        "studio.graph.synthetic_wave_desktop",
        "--host-profile",
        "host_run.profile.headset",
        "--output",
        $EditOutput
    )
    Invoke-Checked "studio validate retargeted host output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $EditOutput
    )
    $AddModuleReportOutput = & cargo run --quiet -p rusty-studio-cli -- add-module --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --package "package.biosignal_sensor" --module "module.biosignal_sensor.provider" --label "Biosignal Provider" --output $AddModuleOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio add module failed with exit code $LASTEXITCODE"
    }
    $AddModuleReportText = $AddModuleReportOutput -join [Environment]::NewLine
    $AddModuleReport = $AddModuleReportText | ConvertFrom-Json
    if ($AddModuleReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "add module edit report schema mismatch"
    }
    if ($AddModuleReport.operation -ne "add_module") {
        throw "add module edit report operation mismatch"
    }
    if ($AddModuleReport.status -ne "applied") {
        throw "add module edit report did not apply"
    }
    if ($AddModuleReport.requested_reference_id -ne "module.biosignal_sensor.provider") {
        throw "add module edit report requested reference mismatch"
    }
    Invoke-Checked "studio validate add-module output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $AddModuleOutput
    )
    $AddModuleProject = Get-Content -Raw -Path $AddModuleOutput | ConvertFrom-Json
    if ($AddModuleProject.revision -ne 2) {
        throw "add module output should bump project revision"
    }
    $AddedGraph = $AddModuleProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $AddedGraph) {
        throw "add module output graph missing"
    }
    $AddedPackage = $AddedGraph.nodes | Where-Object { $_.kind -eq "package" -and $_.reference_id -eq "package.biosignal_sensor" } | Select-Object -First 1
    if ($null -eq $AddedPackage) {
        throw "add module output package node missing"
    }
    $AddedModule = $AddedGraph.nodes | Where-Object { $_.kind -eq "module" -and $_.reference_id -eq "module.biosignal_sensor.provider" } | Select-Object -First 1
    if ($null -eq $AddedModule) {
        throw "add module output module node missing"
    }
    $AddedEdge = $AddedGraph.edges | Where-Object { $_.kind -eq "package_provides_module" -and $_.source_node_id -eq $AddedPackage.node_id -and $_.target_node_id -eq $AddedModule.node_id } | Select-Object -First 1
    if ($null -eq $AddedEdge) {
        throw "add module output package/module edge missing"
    }
    $AddPaletteModuleReportOutput = & cargo run --quiet -p rusty-studio-cli -- add-palette-module --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --output $AddPaletteModuleOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio add palette module failed with exit code $LASTEXITCODE"
    }
    $AddPaletteModuleReportText = $AddPaletteModuleReportOutput -join [Environment]::NewLine
    $AddPaletteModuleReport = $AddPaletteModuleReportText | ConvertFrom-Json
    if ($AddPaletteModuleReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "add palette module edit report schema mismatch"
    }
    if ($AddPaletteModuleReport.operation -ne "add_module") {
        throw "add palette module edit report operation mismatch"
    }
    if ($AddPaletteModuleReport.status -ne "applied") {
        throw "add palette module edit report did not apply"
    }
    if ($AddPaletteModuleReport.requested_reference_id -ne "module.biosignal_sensor.provider") {
        throw "add palette module should choose the first provider module not already in the graph"
    }
    Invoke-Checked "studio validate add-palette-module output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $AddPaletteModuleOutput
    )
    $AddPaletteModuleProject = Get-Content -Raw -Path $AddPaletteModuleOutput | ConvertFrom-Json
    if ($AddPaletteModuleProject.revision -ne 2) {
        throw "add palette module output should bump project revision"
    }
    $PaletteGraph = $AddPaletteModuleProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $PaletteGraph) {
        throw "add palette module output graph missing"
    }
    $PalettePackage = $PaletteGraph.nodes | Where-Object { $_.kind -eq "package" -and $_.reference_id -eq "package.biosignal_sensor" } | Select-Object -First 1
    if ($null -eq $PalettePackage) {
        throw "add palette module output package node missing"
    }
    $PaletteModule = $PaletteGraph.nodes | Where-Object { $_.kind -eq "module" -and $_.reference_id -eq "module.biosignal_sensor.provider" } | Select-Object -First 1
    if ($null -eq $PaletteModule) {
        throw "add palette module output selected module node missing"
    }
    $PaletteEdge = $PaletteGraph.edges | Where-Object { $_.kind -eq "package_provides_module" -and $_.source_node_id -eq $PalettePackage.node_id -and $_.target_node_id -eq $PaletteModule.node_id } | Select-Object -First 1
    if ($null -eq $PaletteEdge) {
        throw "add palette module output package/module edge missing"
    }
    $AddSelectedPackageModuleReportOutput = & cargo run --quiet -p rusty-studio-cli -- add-palette-module --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --package "package.hand_animation" --output $AddSelectedPackageModuleOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio add selected package module failed with exit code $LASTEXITCODE"
    }
    $AddSelectedPackageModuleReportText = $AddSelectedPackageModuleReportOutput -join [Environment]::NewLine
    $AddSelectedPackageModuleReport = $AddSelectedPackageModuleReportText | ConvertFrom-Json
    if ($AddSelectedPackageModuleReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "add selected package module edit report schema mismatch"
    }
    if ($AddSelectedPackageModuleReport.operation -ne "add_module") {
        throw "add selected package module edit report operation mismatch"
    }
    if ($AddSelectedPackageModuleReport.status -ne "applied") {
        throw "add selected package module edit report did not apply"
    }
    if ($AddSelectedPackageModuleReport.requested_reference_id -ne "module.hand_motion.provider") {
        throw "add selected package module should choose a module from the requested package"
    }
    Invoke-Checked "studio validate selected-package-module output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $AddSelectedPackageModuleOutput
    )
    $AddSelectedPackageModuleProject = Get-Content -Raw -Path $AddSelectedPackageModuleOutput | ConvertFrom-Json
    if ($AddSelectedPackageModuleProject.revision -ne 2) {
        throw "add selected package module output should bump project revision"
    }
    $SelectedPackageGraph = $AddSelectedPackageModuleProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $SelectedPackageGraph) {
        throw "add selected package module output graph missing"
    }
    $SelectedPackageNode = $SelectedPackageGraph.nodes | Where-Object { $_.kind -eq "package" -and $_.reference_id -eq "package.hand_animation" } | Select-Object -First 1
    if ($null -eq $SelectedPackageNode) {
        throw "add selected package module output package node missing"
    }
    $SelectedPackageModule = $SelectedPackageGraph.nodes | Where-Object { $_.kind -eq "module" -and $_.reference_id -eq "module.hand_motion.provider" } | Select-Object -First 1
    if ($null -eq $SelectedPackageModule) {
        throw "add selected package module output module node missing"
    }
    $SelectedPackageEdge = $SelectedPackageGraph.edges | Where-Object { $_.kind -eq "package_provides_module" -and $_.source_node_id -eq $SelectedPackageNode.node_id -and $_.target_node_id -eq $SelectedPackageModule.node_id } | Select-Object -First 1
    if ($null -eq $SelectedPackageEdge) {
        throw "add selected package module output package/module edge missing"
    }
    $RemoveModuleReportOutput = & cargo run --quiet -p rusty-studio-cli -- remove-module --project $AddModuleOutput --graph "studio.graph.synthetic_wave_desktop" --module "module.biosignal_sensor.provider" --output $RemoveModuleOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio remove module failed with exit code $LASTEXITCODE"
    }
    $RemoveModuleReportText = $RemoveModuleReportOutput -join [Environment]::NewLine
    $RemoveModuleReport = $RemoveModuleReportText | ConvertFrom-Json
    if ($RemoveModuleReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "remove module edit report schema mismatch"
    }
    if ($RemoveModuleReport.operation -ne "remove_module") {
        throw "remove module edit report operation mismatch"
    }
    if ($RemoveModuleReport.status -ne "applied") {
        throw "remove module edit report did not apply"
    }
    if ($RemoveModuleReport.requested_reference_id -ne "module.biosignal_sensor.provider") {
        throw "remove module edit report requested reference mismatch"
    }
    Invoke-Checked "studio validate remove-module output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $RemoveModuleOutput
    )
    $RemoveModuleProject = Get-Content -Raw -Path $RemoveModuleOutput | ConvertFrom-Json
    if ($RemoveModuleProject.revision -ne 3) {
        throw "remove module output should bump project revision from add-module output"
    }
    $RemovedGraph = $RemoveModuleProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $RemovedGraph) {
        throw "remove module output graph missing"
    }
    $RemovedModule = $RemovedGraph.nodes | Where-Object { $_.kind -eq "module" -and $_.reference_id -eq "module.biosignal_sensor.provider" } | Select-Object -First 1
    if ($null -ne $RemovedModule) {
        throw "remove module output still contains removed module node"
    }
    $DanglingRemovedEdge = $RemovedGraph.edges | Where-Object { $_.source_node_id -eq $AddedModule.node_id -or $_.target_node_id -eq $AddedModule.node_id } | Select-Object -First 1
    if ($null -ne $DanglingRemovedEdge) {
        throw "remove module output still contains an edge incident to the removed module"
    }
    $AddBindingReportOutput = & cargo run --quiet -p rusty-studio-cli -- add-binding --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --kind "command" --source-node "node.shell.operator" --target-node "node.module.synthetic_wave_provider" --output $AddBindingOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio add binding failed with exit code $LASTEXITCODE"
    }
    $AddBindingReportText = $AddBindingReportOutput -join [Environment]::NewLine
    $AddBindingReport = $AddBindingReportText | ConvertFrom-Json
    if ($AddBindingReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "add binding edit report schema mismatch"
    }
    if ($AddBindingReport.operation -ne "add_binding") {
        throw "add binding edit report operation mismatch"
    }
    if ($AddBindingReport.status -ne "applied") {
        throw "add binding edit report did not apply"
    }
    $ExpectedBindingId = "edge.command_binding.node.shell.operator.node.module.synthetic_wave_provider"
    if ($AddBindingReport.requested_reference_id -ne $ExpectedBindingId) {
        throw "add binding edit report requested reference mismatch"
    }
    Invoke-Checked "studio validate add-binding output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $AddBindingOutput
    )
    $AddBindingProject = Get-Content -Raw -Path $AddBindingOutput | ConvertFrom-Json
    if ($AddBindingProject.revision -ne 2) {
        throw "add binding output should bump project revision"
    }
    $BindingGraph = $AddBindingProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $BindingGraph) {
        throw "add binding output graph missing"
    }
    $AddedBindingEdge = $BindingGraph.edges | Where-Object { $_.kind -eq "command_binding" -and $_.source_node_id -eq "node.shell.operator" -and $_.target_node_id -eq "node.module.synthetic_wave_provider" } | Select-Object -First 1
    if ($null -eq $AddedBindingEdge) {
        throw "add binding output command edge missing"
    }
    $RemoveBindingReportOutput = & cargo run --quiet -p rusty-studio-cli -- remove-binding --project $AddBindingOutput --graph "studio.graph.synthetic_wave_desktop" --kind "command" --source-node "node.shell.operator" --target-node "node.module.synthetic_wave_provider" --output $RemoveBindingOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio remove binding failed with exit code $LASTEXITCODE"
    }
    $RemoveBindingReportText = $RemoveBindingReportOutput -join [Environment]::NewLine
    $RemoveBindingReport = $RemoveBindingReportText | ConvertFrom-Json
    if ($RemoveBindingReport.'$schema' -ne "rusty.studio.edit_report.v1") {
        throw "remove binding edit report schema mismatch"
    }
    if ($RemoveBindingReport.operation -ne "remove_binding") {
        throw "remove binding edit report operation mismatch"
    }
    if ($RemoveBindingReport.status -ne "applied") {
        throw "remove binding edit report did not apply"
    }
    if ($RemoveBindingReport.requested_reference_id -ne $ExpectedBindingId) {
        throw "remove binding edit report requested reference mismatch"
    }
    Invoke-Checked "studio validate remove-binding output" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "validate",
        "--project",
        $RemoveBindingOutput
    )
    $RemoveBindingProject = Get-Content -Raw -Path $RemoveBindingOutput | ConvertFrom-Json
    if ($RemoveBindingProject.revision -ne 3) {
        throw "remove binding output should bump project revision from add-binding output"
    }
    $RemovedBindingGraph = $RemoveBindingProject.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $RemovedBindingGraph) {
        throw "remove binding output graph missing"
    }
    $RemovedBindingEdge = $RemovedBindingGraph.edges | Where-Object { $_.kind -eq "command_binding" -and $_.source_node_id -eq "node.shell.operator" -and $_.target_node_id -eq "node.module.synthetic_wave_provider" } | Select-Object -First 1
    if ($null -ne $RemovedBindingEdge) {
        throw "remove binding output still contains removed command edge"
    }
    Invoke-Checked "studio shell descriptor" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "shell-descriptor",
        "--project",
        "examples\synthetic-studio-project.json",
        "--graph",
        "studio.graph.synthetic_wave_desktop",
        "--output",
        $ShellOutput
    )
    $ShellDescriptor = Get-Content -Raw -Path $ShellOutput | ConvertFrom-Json
    if ($ShellDescriptor.'$schema' -ne "rusty.studio.shell_descriptor.v1") {
        throw "shell descriptor schema mismatch"
    }
    if ($ShellDescriptor.target_host_profile -ne "host_run.profile.desktop") {
        throw "shell descriptor target host mismatch"
    }
    if ($ShellDescriptor.shell_id -ne "shell.synthetic_wave.desktop_operator") {
        throw "shell descriptor operator shell mismatch"
    }
    $ShellValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-descriptor --descriptor $ShellOutput
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate shell descriptor failed with exit code $LASTEXITCODE"
    }
    $ShellValidationText = $ShellValidationOutput -join [Environment]::NewLine
    $ShellValidation = $ShellValidationText | ConvertFrom-Json
    if ($ShellValidation.'$schema' -ne "rusty.studio.shell_descriptor_validation_report.v1") {
        throw "shell descriptor validation schema mismatch"
    }
    if ($ShellValidation.status -ne "pass") {
        throw "shell descriptor validation did not pass"
    }
    Invoke-Checked "studio shell artifacts" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "shell-artifacts",
        "--project",
        "examples\synthetic-studio-project.json",
        "--output-dir",
        $ShellArtifactsDir
    )
    $ShellArtifactsManifestPath = Join-Path $ShellArtifactsDir "shell-artifacts.json"
    if (-not (Test-Path $ShellArtifactsManifestPath)) {
        throw "shell artifacts manifest was not written"
    }
    $ShellArtifactsManifest = Get-Content -Raw -Path $ShellArtifactsManifestPath | ConvertFrom-Json
    if ($ShellArtifactsManifest.'$schema' -ne "rusty.studio.shell_artifact_manifest.v1") {
        throw "shell artifacts manifest schema mismatch"
    }
    if ($ShellArtifactsManifest.artifacts.Count -ne 3) {
        throw "shell artifacts manifest should contain desktop, phone, and quest artifacts"
    }
    $TargetKinds = @($ShellArtifactsManifest.artifacts | ForEach-Object { $_.target_kind })
    foreach ($RequiredTargetKind in @("desktop", "phone", "quest")) {
        if ($TargetKinds -notcontains $RequiredTargetKind) {
            throw "shell artifacts manifest missing target kind $RequiredTargetKind"
        }
    }
    foreach ($Artifact in $ShellArtifactsManifest.artifacts) {
        $RelativeDescriptorPath = $Artifact.descriptor_path -replace '/', [System.IO.Path]::DirectorySeparatorChar
        $DescriptorPath = Join-Path $ShellArtifactsDir $RelativeDescriptorPath
        if (-not (Test-Path $DescriptorPath)) {
            throw "shell artifact descriptor missing: $($Artifact.descriptor_path)"
        }
        $DescriptorValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-descriptor --descriptor $DescriptorPath
        if ($LASTEXITCODE -ne 0) {
            throw "studio validate shell artifact descriptor failed with exit code $LASTEXITCODE"
        }
        $DescriptorValidationText = $DescriptorValidationOutput -join [Environment]::NewLine
        $DescriptorValidation = $DescriptorValidationText | ConvertFrom-Json
        if ($DescriptorValidation.status -ne "pass") {
            throw "shell artifact descriptor validation did not pass: $($Artifact.descriptor_path)"
        }
    }
    $ShellArtifactsValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-artifacts --manifest $ShellArtifactsManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate shell artifacts failed with exit code $LASTEXITCODE"
    }
    $ShellArtifactsValidationText = $ShellArtifactsValidationOutput -join [Environment]::NewLine
    $ShellArtifactsValidation = $ShellArtifactsValidationText | ConvertFrom-Json
    if ($ShellArtifactsValidation.'$schema' -ne "rusty.studio.shell_artifact_manifest_validation_report.v1") {
        throw "shell artifacts validation schema mismatch"
    }
    if ($ShellArtifactsValidation.status -ne "pass") {
        throw "shell artifacts validation did not pass"
    }
    Invoke-Checked "studio shell templates" "cargo" @(
        "run",
        "-p",
        "rusty-studio-cli",
        "--",
        "shell-templates",
        "--manifest",
        $ShellArtifactsManifestPath,
        "--output-dir",
        $ShellTemplatesDir
    )
    $ShellTemplatesIndexPath = Join-Path $ShellTemplatesDir "shell-templates.json"
    if (-not (Test-Path $ShellTemplatesIndexPath)) {
        throw "shell templates index was not written"
    }
    $ShellTemplatesIndex = Get-Content -Raw -Path $ShellTemplatesIndexPath | ConvertFrom-Json
    if ($ShellTemplatesIndex.'$schema' -ne "rusty.studio.shell_template_index.v1") {
        throw "shell templates index schema mismatch"
    }
    if ($ShellTemplatesIndex.templates.Count -ne 3) {
        throw "shell templates index should contain desktop, phone, and quest templates"
    }
    $ShellTemplatesValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-templates --index $ShellTemplatesIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate shell templates failed with exit code $LASTEXITCODE"
    }
    $ShellTemplatesValidationText = $ShellTemplatesValidationOutput -join [Environment]::NewLine
    $ShellTemplatesValidation = $ShellTemplatesValidationText | ConvertFrom-Json
    if ($ShellTemplatesValidation.'$schema' -ne "rusty.studio.shell_template_index_validation_report.v1") {
        throw "shell templates validation schema mismatch"
    }
    if ($ShellTemplatesValidation.status -ne "pass") {
        throw "shell templates validation did not pass"
    }
    $TemplateTargetKinds = @($ShellTemplatesIndex.templates | ForEach-Object { $_.target_kind })
    foreach ($RequiredTargetKind in @("desktop", "phone", "quest")) {
        if ($TemplateTargetKinds -notcontains $RequiredTargetKind) {
            throw "shell templates index missing target kind $RequiredTargetKind"
        }
    }
    foreach ($TemplateEntry in $ShellTemplatesIndex.templates) {
        $RelativeTemplatePath = $TemplateEntry.template_path -replace '/', [System.IO.Path]::DirectorySeparatorChar
        $TemplatePath = Join-Path $ShellTemplatesDir $RelativeTemplatePath
        if (-not (Test-Path $TemplatePath)) {
            throw "shell template manifest missing: $($TemplateEntry.template_path)"
        }
        $TemplateManifest = Get-Content -Raw -Path $TemplatePath | ConvertFrom-Json
        if ($TemplateManifest.'$schema' -ne "rusty.studio.shell_template_manifest.v1") {
            throw "shell template manifest schema mismatch: $($TemplateEntry.template_path)"
        }
        if ($TemplateManifest.runtime_authority.command_session_authority -ne "rusty.manifold") {
            throw "shell template command/session authority mismatch"
        }
        if ($TemplateManifest.runtime_authority.install_launch_evidence_authority -ne "rusty.hostess") {
            throw "shell template install/launch/evidence authority mismatch"
        }
        if ($TemplateManifest.runtime_authority.studio_role -ne "authoring.export_planning") {
            throw "shell template Studio role mismatch"
        }

        $RelativeStagedDescriptorPath = $TemplateEntry.descriptor_path -replace '/', [System.IO.Path]::DirectorySeparatorChar
        $StagedDescriptorPath = Join-Path $ShellTemplatesDir $RelativeStagedDescriptorPath
        if (-not (Test-Path $StagedDescriptorPath)) {
            throw "shell template staged descriptor missing: $($TemplateEntry.descriptor_path)"
        }
        $StagedDescriptorValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-descriptor --descriptor $StagedDescriptorPath
        if ($LASTEXITCODE -ne 0) {
            throw "studio validate staged shell template descriptor failed with exit code $LASTEXITCODE"
        }
        $StagedDescriptorValidationText = $StagedDescriptorValidationOutput -join [Environment]::NewLine
        $StagedDescriptorValidation = $StagedDescriptorValidationText | ConvertFrom-Json
        if ($StagedDescriptorValidation.status -ne "pass") {
            throw "shell template staged descriptor validation did not pass: $($TemplateEntry.descriptor_path)"
        }
    }
    $SelectedShellBundleOutput = & cargo run --quiet -p rusty-studio-cli -- shell-bundle --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --output-dir $SelectedShellBundleDir
    if ($LASTEXITCODE -ne 0) {
        throw "studio selected shell bundle report failed with exit code $LASTEXITCODE"
    }
    $SelectedShellBundleText = $SelectedShellBundleOutput -join [Environment]::NewLine
    $SelectedShellBundle = $SelectedShellBundleText | ConvertFrom-Json
    if ($SelectedShellBundle.'$schema' -ne "rusty.studio.shell_bundle_report.v1") {
        throw "selected shell bundle schema mismatch"
    }
    if ($SelectedShellBundle.status -ne "exported") {
        throw "selected shell bundle was not exported"
    }
    if ($SelectedShellBundle.graph_id -ne "studio.graph.synthetic_wave_desktop") {
        throw "selected shell bundle graph id mismatch"
    }
    if ($SelectedShellBundle.artifact_manifest.artifacts.Count -ne 1) {
        throw "selected shell bundle should contain one artifact"
    }
    if ($SelectedShellBundle.template_index.templates.Count -ne 1) {
        throw "selected shell bundle should contain one template"
    }
    if (@($SelectedShellBundle.bundle_files) -notcontains "descriptors/studio.graph.synthetic_wave_desktop.shell-descriptor.json") {
        throw "selected shell bundle missing descriptor path"
    }
    if (@($SelectedShellBundle.bundle_files) -notcontains "shell-artifacts.json") {
        throw "selected shell bundle missing artifact manifest path"
    }
    if (@($SelectedShellBundle.bundle_files) -notcontains "shell-templates.json") {
        throw "selected shell bundle missing template index path"
    }
    if (@($SelectedShellBundle.bundle_files) -notcontains "shells/desktop/studio.graph.synthetic_wave_desktop.shell-template.json") {
        throw "selected shell bundle missing template manifest path"
    }
    if ($SelectedShellBundle.template_manifest.runtime_authority.command_session_authority -ne "rusty.manifold") {
        throw "selected shell bundle command/session authority mismatch"
    }
    if ($SelectedShellBundle.template_manifest.runtime_authority.install_launch_evidence_authority -ne "rusty.hostess") {
        throw "selected shell bundle install/launch/evidence authority mismatch"
    }
    if ($SelectedShellBundle.template_manifest.runtime_authority.studio_role -ne "authoring.export_planning") {
        throw "selected shell bundle Studio role mismatch"
    }
    $SelectedBundleDescriptorPath = Join-Path $SelectedShellBundleDir "descriptors\studio.graph.synthetic_wave_desktop.shell-descriptor.json"
    if (-not (Test-Path $SelectedBundleDescriptorPath)) {
        throw "selected shell bundle descriptor was not written"
    }
    $SelectedBundleManifestPath = Join-Path $SelectedShellBundleDir "shell-artifacts.json"
    if (-not (Test-Path $SelectedBundleManifestPath)) {
        throw "selected shell bundle artifact manifest was not written"
    }
    $SelectedBundleIndexPath = Join-Path $SelectedShellBundleDir "shell-templates.json"
    if (-not (Test-Path $SelectedBundleIndexPath)) {
        throw "selected shell bundle template index was not written"
    }
    $SelectedBundleTemplatePath = Join-Path $SelectedShellBundleDir "shells\desktop\studio.graph.synthetic_wave_desktop.shell-template.json"
    if (-not (Test-Path $SelectedBundleTemplatePath)) {
        throw "selected shell bundle template manifest was not written"
    }
    $SelectedBundleArtifactValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-artifacts --manifest $SelectedBundleManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate selected shell bundle artifacts failed with exit code $LASTEXITCODE"
    }
    $SelectedBundleArtifactValidation = ($SelectedBundleArtifactValidationOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($SelectedBundleArtifactValidation.status -ne "pass") {
        throw "selected shell bundle artifact validation did not pass"
    }
    $SelectedBundleTemplateValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-templates --index $SelectedBundleIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate selected shell bundle templates failed with exit code $LASTEXITCODE"
    }
    $SelectedBundleTemplateValidation = ($SelectedBundleTemplateValidationOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($SelectedBundleTemplateValidation.status -ne "pass") {
        throw "selected shell bundle template validation did not pass"
    }
    $SelectedBundleCurrentValidationOutput = & cargo run --quiet -p rusty-studio-cli -- validate-shell-bundle --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --bundle-dir $SelectedShellBundleDir
    if ($LASTEXITCODE -ne 0) {
        throw "studio validate selected shell bundle failed with exit code $LASTEXITCODE"
    }
    $SelectedBundleCurrentValidation = ($SelectedBundleCurrentValidationOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($SelectedBundleCurrentValidation.'$schema' -ne "rusty.studio.shell_bundle_validation_report.v1") {
        throw "selected shell bundle current validation schema mismatch"
    }
    if ($SelectedBundleCurrentValidation.status -ne "pass") {
        throw "selected shell bundle current validation did not pass"
    }
    if ($SelectedBundleCurrentValidation.graph_id -ne "studio.graph.synthetic_wave_desktop") {
        throw "selected shell bundle current validation graph id mismatch"
    }
    if (@($SelectedBundleCurrentValidation.expected_bundle_files).Count -ne 4) {
        throw "selected shell bundle current validation expected file count mismatch"
    }
    $FailedSelectedBundleChecks = @($SelectedBundleCurrentValidation.checks | Where-Object { $_.status -ne "pass" })
    if ($FailedSelectedBundleChecks.Count -ne 0) {
        throw "selected shell bundle current validation exposed failed checks"
    }
    $SelectedBundleCheckIds = @($SelectedBundleCurrentValidation.checks | ForEach-Object { $_.check_id })
    foreach ($RequiredSelectedBundleCheck in @(
        "studio.check.shell_bundle.current_preview",
        "studio.check.shell_bundle.descriptor.current_match",
        "studio.check.shell_bundle.artifact_manifest.current_match",
        "studio.check.shell_bundle.template_index.current_match",
        "studio.check.shell_bundle.template_manifest.current_match",
        "studio.check.shell_bundle.template_manifest.runtime_authority"
    )) {
        if ($SelectedBundleCheckIds -notcontains $RequiredSelectedBundleCheck) {
            throw "selected shell bundle current validation missing check $RequiredSelectedBundleCheck"
        }
    }
    $GenericDesktopHandoffOutput = & cargo run --quiet -p rusty-studio-cli -- shell-handoff --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --bundle-dir $SelectedShellBundleDir
    if ($LASTEXITCODE -ne 0) {
        throw "studio generic desktop shell handoff failed with exit code $LASTEXITCODE"
    }
    $GenericDesktopHandoff = ($GenericDesktopHandoffOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($GenericDesktopHandoff.'$schema' -ne "rusty.studio.shell_handoff_report.v1") {
        throw "generic desktop shell handoff schema mismatch"
    }
    if ($GenericDesktopHandoff.status -ne "pass") {
        throw "generic desktop shell handoff did not pass"
    }
    if ($GenericDesktopHandoff.handoff_kind -ne "desktop_shell") {
        throw "generic desktop shell handoff kind mismatch"
    }
    if ($GenericDesktopHandoff.consumer_id -ne "rusty-studio-desktop-shell") {
        throw "generic desktop shell handoff consumer mismatch"
    }
    if ($GenericDesktopHandoff.target_kind -ne "desktop") {
        throw "generic desktop shell handoff target mismatch"
    }
    $DesktopHandoffOutput = & cargo run --quiet -p rusty-studio-cli -- desktop-shell-handoff --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_desktop" --bundle-dir $SelectedShellBundleDir
    if ($LASTEXITCODE -ne 0) {
        throw "studio desktop shell handoff failed with exit code $LASTEXITCODE"
    }
    $DesktopHandoff = ($DesktopHandoffOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($DesktopHandoff.'$schema' -ne "rusty.studio.shell_handoff_report.v1") {
        throw "desktop shell handoff schema mismatch"
    }
    if ($DesktopHandoff.status -ne "pass") {
        throw "desktop shell handoff did not pass"
    }
    if ($DesktopHandoff.handoff_kind -ne "desktop_shell") {
        throw "desktop shell handoff kind mismatch"
    }
    if ($DesktopHandoff.consumer_id -ne "rusty-studio-desktop-shell") {
        throw "desktop shell handoff consumer mismatch"
    }
    if ($DesktopHandoff.target_kind -ne "desktop") {
        throw "desktop shell handoff target mismatch"
    }
    if (@($DesktopHandoff.consumer_args) -notcontains "--templates") {
        throw "desktop shell handoff missing --templates arg"
    }
    if (@($DesktopHandoff.consumer_args) -notcontains (Join-Path $SelectedShellBundleDir "shell-templates.json")) {
        throw "desktop shell handoff missing template index arg"
    }
    if ($DesktopHandoff.validation.status -ne "pass") {
        throw "desktop shell handoff validation did not pass"
    }
    if ($DesktopHandoff.runtime_authority.command_session_authority -ne "rusty.manifold") {
        throw "desktop shell handoff command/session authority mismatch"
    }
    if ($DesktopHandoff.runtime_authority.install_launch_evidence_authority -ne "rusty.hostess") {
        throw "desktop shell handoff install/launch/evidence authority mismatch"
    }
    if ($DesktopHandoff.runtime_authority.studio_role -ne "authoring.export_planning") {
        throw "desktop shell handoff Studio role mismatch"
    }
    foreach ($TargetHandoff in @(
        @{
            Graph = "studio.graph.synthetic_wave_phone"
            BundleDir = $SelectedPhoneShellBundleDir
            HandoffKind = "phone_shell"
            Consumer = "rusty-studio-phone-shell"
            TargetKind = "phone"
        },
        @{
            Graph = "studio.graph.synthetic_wave_headset"
            BundleDir = $SelectedQuestShellBundleDir
            HandoffKind = "quest_shell"
            Consumer = "rusty-studio-quest-shell"
            TargetKind = "quest"
        }
    )) {
        $TargetBundleOutput = & cargo run --quiet -p rusty-studio-cli -- shell-bundle --project "examples\synthetic-studio-project.json" --graph $TargetHandoff.Graph --output-dir $TargetHandoff.BundleDir
        if ($LASTEXITCODE -ne 0) {
            throw "studio selected target shell bundle failed for $($TargetHandoff.Graph) with exit code $LASTEXITCODE"
        }
        $TargetBundle = ($TargetBundleOutput -join [Environment]::NewLine) | ConvertFrom-Json
        if ($TargetBundle.status -ne "exported") {
            throw "selected target shell bundle did not export for $($TargetHandoff.Graph)"
        }
        $TargetHandoffOutput = & cargo run --quiet -p rusty-studio-cli -- shell-handoff --project "examples\synthetic-studio-project.json" --graph $TargetHandoff.Graph --bundle-dir $TargetHandoff.BundleDir
        if ($LASTEXITCODE -ne 0) {
            throw "studio target shell handoff failed for $($TargetHandoff.Graph) with exit code $LASTEXITCODE"
        }
        $TargetReport = ($TargetHandoffOutput -join [Environment]::NewLine) | ConvertFrom-Json
        if ($TargetReport.'$schema' -ne "rusty.studio.shell_handoff_report.v1") {
            throw "target shell handoff schema mismatch for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.status -ne "pass") {
            throw "target shell handoff did not pass for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.handoff_kind -ne $TargetHandoff.HandoffKind) {
            throw "target shell handoff kind mismatch for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.consumer_id -ne $TargetHandoff.Consumer) {
            throw "target shell handoff consumer mismatch for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.target_kind -ne $TargetHandoff.TargetKind) {
            throw "target shell handoff target mismatch for $($TargetHandoff.Graph)"
        }
        if (@($TargetReport.consumer_args) -notcontains "--templates") {
            throw "target shell handoff missing --templates arg for $($TargetHandoff.Graph)"
        }
        if (@($TargetReport.consumer_args) -notcontains (Join-Path $TargetHandoff.BundleDir "shell-templates.json")) {
            throw "target shell handoff missing template index arg for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.validation.status -ne "pass") {
            throw "target shell handoff validation did not pass for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.runtime_authority.command_session_authority -ne "rusty.manifold") {
            throw "target shell handoff command/session authority mismatch for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.runtime_authority.install_launch_evidence_authority -ne "rusty.hostess") {
            throw "target shell handoff install/launch/evidence authority mismatch for $($TargetHandoff.Graph)"
        }
        if ($TargetReport.runtime_authority.studio_role -ne "authoring.export_planning") {
            throw "target shell handoff Studio role mismatch for $($TargetHandoff.Graph)"
        }
    }
    $RejectedDesktopHandoffOutput = & cargo run --quiet -p rusty-studio-cli -- desktop-shell-handoff --project "examples\synthetic-studio-project.json" --graph "studio.graph.synthetic_wave_phone" --bundle-dir $SelectedPhoneShellBundleDir
    if ($LASTEXITCODE -ne 0) {
        throw "studio rejected desktop shell handoff command failed with exit code $LASTEXITCODE"
    }
    $RejectedDesktopHandoff = ($RejectedDesktopHandoffOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($RejectedDesktopHandoff.status -ne "fail") {
        throw "desktop shell handoff should reject phone bundle"
    }
    if ($RejectedDesktopHandoff.issue_code -ne "studio.issue.shell_handoff_target_mismatch") {
        throw "desktop shell handoff target mismatch issue missing"
    }
    $HandoffReadinessOutput = & cargo run --quiet -p rusty-studio-cli -- shell-handoff-readiness --project "examples\synthetic-studio-project.json" --bundle-root $SelectedShellBundleRoot
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell handoff readiness failed with exit code $LASTEXITCODE"
    }
    $HandoffReadiness = ($HandoffReadinessOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($HandoffReadiness.'$schema' -ne "rusty.studio.shell_handoff_readiness_report.v1") {
        throw "shell handoff readiness schema mismatch"
    }
    if ($HandoffReadiness.status -ne "pass") {
        throw "shell handoff readiness did not pass"
    }
    if (@($HandoffReadiness.entries).Count -ne 3) {
        throw "shell handoff readiness entry count mismatch"
    }
    foreach ($RequiredReadiness in @(
        @{ Graph = "studio.graph.synthetic_wave_desktop"; HandoffKind = "desktop_shell"; Consumer = "rusty-studio-desktop-shell"; TargetKind = "desktop" },
        @{ Graph = "studio.graph.synthetic_wave_phone"; HandoffKind = "phone_shell"; Consumer = "rusty-studio-phone-shell"; TargetKind = "phone" },
        @{ Graph = "studio.graph.synthetic_wave_headset"; HandoffKind = "quest_shell"; Consumer = "rusty-studio-quest-shell"; TargetKind = "quest" }
    )) {
        $Entry = @($HandoffReadiness.entries | Where-Object { $_.graph_id -eq $RequiredReadiness.Graph }) | Select-Object -First 1
        if ($null -eq $Entry) {
            throw "shell handoff readiness missing graph $($RequiredReadiness.Graph)"
        }
        if ($Entry.status -ne "pass") {
            throw "shell handoff readiness entry did not pass for $($RequiredReadiness.Graph)"
        }
        if ($Entry.handoff_kind -ne $RequiredReadiness.HandoffKind) {
            throw "shell handoff readiness handoff kind mismatch for $($RequiredReadiness.Graph)"
        }
        if ($Entry.consumer_id -ne $RequiredReadiness.Consumer) {
            throw "shell handoff readiness consumer mismatch for $($RequiredReadiness.Graph)"
        }
        if ($Entry.target_kind -ne $RequiredReadiness.TargetKind) {
            throw "shell handoff readiness target mismatch for $($RequiredReadiness.Graph)"
        }
        if ($Entry.validation_status -ne "pass") {
            throw "shell handoff readiness validation mismatch for $($RequiredReadiness.Graph)"
        }
        if ($Entry.failed_check_count -ne 0) {
            throw "shell handoff readiness failed check count mismatch for $($RequiredReadiness.Graph)"
        }
    }
} finally {
    Pop-Location
}
