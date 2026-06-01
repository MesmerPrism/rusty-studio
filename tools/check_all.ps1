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
    $AddModuleOutput = Join-Path $RepoRoot "target\studio-edit-add-module.json"
    $AddPaletteModuleOutput = Join-Path $RepoRoot "target\studio-edit-add-palette-module.json"
    $RemoveModuleOutput = Join-Path $RepoRoot "target\studio-edit-remove-module.json"
    $AddBindingOutput = Join-Path $RepoRoot "target\studio-edit-add-binding.json"
    $RemoveBindingOutput = Join-Path $RepoRoot "target\studio-edit-remove-binding.json"
    $ShellOutput = Join-Path $RepoRoot "target\studio-shell-descriptor-desktop.json"
    $ShellArtifactsDir = Join-Path $RepoRoot "target\studio-shells"
    $ShellTemplatesDir = Join-Path $RepoRoot "target\studio-shell-templates"
    New-Item -ItemType Directory -Path (Split-Path $EditOutput) -Force | Out-Null
    foreach ($GeneratedOutput in @($EditOutput, $DiagnosticProjectOutput, $AddModuleOutput, $AddPaletteModuleOutput, $RemoveModuleOutput, $AddBindingOutput, $RemoveBindingOutput, $ShellOutput)) {
        if (Test-Path $GeneratedOutput) {
            Remove-Item -LiteralPath $GeneratedOutput
        }
    }
    foreach ($GeneratedDir in @($ShellArtifactsDir, $ShellTemplatesDir)) {
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
    $ViewModelDesktopGraph = $ViewModel.graphs | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_desktop" } | Select-Object -First 1
    if ($null -eq $ViewModelDesktopGraph) {
        throw "view model missing desktop graph row"
    }
    if ($ViewModelDesktopGraph.validation_issue_count -ne 0) {
        throw "valid desktop graph row should have no validation issues"
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
} finally {
    Pop-Location
}
