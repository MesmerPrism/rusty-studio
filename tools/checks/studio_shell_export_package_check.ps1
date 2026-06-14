function Invoke-StudioShellExportPackageCheck {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$true)]
        [string]$DamagedShellBundleRoot,
        [Parameter(Mandatory=$true)]
        [string]$DamagedShellExportPackagePath,
        [Parameter(Mandatory=$true)]
        [string]$DamagedShellHandoffManifestPath,
        [Parameter(Mandatory=$true)]
        [string]$DamagedTemplateShellBundleRoot,
        [Parameter(Mandatory=$true)]
        [string]$DamagedTemplateShellExportPackageBaselinePath,
        [Parameter(Mandatory=$true)]
        [string]$DamagedTemplateShellExportPackagePath,
        [Parameter(Mandatory=$true)]
        [string]$DamagedTemplateShellHandoffManifestPath,
        [Parameter(Mandatory=$true)]
        [object]$HandoffManifest,
        [Parameter(Mandatory=$true)]
        [string]$RegressedShellExportPackageComparisonPath,
        [Parameter(Mandatory=$true)]
        [string]$SelectedShellBundleRoot,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageBaselineIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageBaselinePath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageBaselineSelectionPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageComparisonPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageIndexComparisonPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackageMultiBaselineIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackagePath,
        [Parameter(Mandatory=$true)]
        [string]$ShellExportPackagePromotedBaselineIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$SourceProjectPath
    )

    $ShellExportPackageOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package --project $SourceProjectPath --bundle-root $SelectedShellBundleRoot --output $ShellExportPackagePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackagePath)) {
        throw "shell export package was not written"
    }
    $ShellExportPackage = ($ShellExportPackageOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackage = Get-Content -Raw $ShellExportPackagePath | ConvertFrom-Json
    foreach ($PackageView in @($ShellExportPackage, $WrittenShellExportPackage)) {
        if ($PackageView.'$schema' -ne "rusty.studio.shell_export_package_report.v1") {
            throw "shell export package schema mismatch"
        }
        if ($PackageView.source_manifest_schema -ne "rusty.studio.shell_handoff_manifest.v1") {
            throw "shell export package source manifest schema mismatch"
        }
        if ($PackageView.source_runbook_schema -ne "rusty.studio.shell_runbook_report.v1") {
            throw "shell export package source runbook schema mismatch"
        }
        if ($PackageView.package_id -ne "studio.shell_export_package.studio.project.synthetic_wave") {
            throw "shell export package id mismatch"
        }
        if ($PackageView.manifest_id -ne $HandoffManifest.manifest_id -or $PackageView.project_id -ne $HandoffManifest.project_id -or $PackageView.project_revision -ne $HandoffManifest.project_revision) {
            throw "shell export package source manifest metadata mismatch"
        }
        if ($PackageView.status -ne "ready") {
            throw "shell export package was not ready"
        }
        if ($null -ne $PackageView.issue_code) {
            throw "shell export package should not carry a top-level issue"
        }
        if ($PackageView.execution_policy -ne "not_executed.review_only" -or $PackageView.review_owner -ne "rusty.hostess") {
            throw "shell export package review policy mismatch"
        }
        if ($PackageView.command_session_authority -ne "rusty.manifold" -or $PackageView.install_launch_evidence_authority -ne "rusty.hostess" -or $PackageView.studio_role -ne "authoring.export_planning") {
            throw "shell export package authority mismatch"
        }
        if ($PackageView.ready_count -ne 3 -or $PackageView.blocked_count -ne 0 -or $PackageView.rejected_count -ne 0) {
            throw "shell export package counts mismatch"
        }
        if ($PackageView.descriptor_count -ne 3 -or $PackageView.template_manifest_count -ne 3 -or $PackageView.runbook_entry_count -ne 3) {
            throw "shell export package artifact counts mismatch"
        }
        if (@($PackageView.target_summaries).Count -ne 3) {
            throw "shell export package target summary count mismatch"
        }
        if (@($PackageView.entries).Count -ne 3) {
            throw "shell export package entry count mismatch"
        }
        foreach ($RequiredAction in @("install", "launch", "open_command_session", "collect_device_evidence")) {
            if (@($PackageView.prohibited_actions) -notcontains $RequiredAction) {
                throw "shell export package missing prohibited action $RequiredAction"
            }
        }
        foreach ($RequiredPackage in @(
            @{ Graph = "studio.graph.synthetic_wave_desktop"; TargetKind = "desktop"; TargetProfile = "host_run.profile.desktop"; RouteKind = "desktop_operator_shell"; Consumer = "rusty-studio-desktop-shell"; Shell = "shell.synthetic_wave.desktop_operator"; Install = "install.local_process"; Launch = "launch.local_process"; Bridge = "bridge.local_cli"; Evidence = "evidence.filesystem" },
            @{ Graph = "studio.graph.synthetic_wave_phone"; TargetKind = "phone"; TargetProfile = "host_run.profile.mobile"; RouteKind = "phone_operator_shell"; Consumer = "rusty-studio-phone-shell"; Shell = "shell.synthetic_wave.phone_operator"; Install = "install.android_package"; Launch = "launch.android_intent"; Bridge = "bridge.adb_intent_file"; Evidence = "evidence.adb_pull" },
            @{ Graph = "studio.graph.synthetic_wave_headset"; TargetKind = "quest"; TargetProfile = "host_run.profile.headset"; RouteKind = "quest_operator_shell"; Consumer = "rusty-studio-quest-shell"; Shell = "shell.synthetic_wave.quest_operator"; Install = "install.android_package"; Launch = "launch.android_intent"; Bridge = "bridge.adb_intent_file"; Evidence = "evidence.adb_pull" }
        )) {
            $PackageTarget = @($PackageView.target_summaries | Where-Object { $_.target_kind -eq $RequiredPackage.TargetKind }) | Select-Object -First 1
            if ($null -eq $PackageTarget) {
                throw "shell export package missing target $($RequiredPackage.TargetKind)"
            }
            if ($PackageTarget.ready_count -ne 1 -or $PackageTarget.blocked_count -ne 0 -or $PackageTarget.rejected_count -ne 0 -or $PackageTarget.descriptor_count -ne 1 -or $PackageTarget.template_manifest_count -ne 1) {
                throw "shell export package target counts mismatch for $($RequiredPackage.TargetKind)"
            }
            if (-not (@($PackageTarget.graph_ids) -contains $RequiredPackage.Graph)) {
                throw "shell export package target graph id mismatch for $($RequiredPackage.TargetKind)"
            }
            if (-not (@($PackageTarget.consumer_ids) -contains $RequiredPackage.Consumer)) {
                throw "shell export package target consumer mismatch for $($RequiredPackage.TargetKind)"
            }
            if (-not (@($PackageTarget.responsible_owners) -contains "rusty.hostess")) {
                throw "shell export package target owner mismatch for $($RequiredPackage.TargetKind)"
            }
            if (@($PackageTarget.issue_codes).Count -ne 0) {
                throw "shell export package target issue codes mismatch for $($RequiredPackage.TargetKind)"
            }

            $PackageEntry = @($PackageView.entries | Where-Object { $_.graph_id -eq $RequiredPackage.Graph }) | Select-Object -First 1
            if ($null -eq $PackageEntry) {
                throw "shell export package missing graph $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.status -ne "ready" -or $null -ne $PackageEntry.issue_code) {
                throw "shell export package entry status mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.responsible_owner -ne "rusty.hostess" -or $PackageEntry.execution_policy -ne "not_executed.review_only" -or $PackageEntry.next_required_action -ne "review_with_runtime_owner") {
                throw "shell export package owner or policy mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.consumer_id -ne $RequiredPackage.Consumer -or $PackageEntry.target_kind -ne $RequiredPackage.TargetKind -or $PackageEntry.target_host_profile -ne $RequiredPackage.TargetProfile -or $PackageEntry.runtime_route_kind -ne $RequiredPackage.RouteKind) {
                throw "shell export package target metadata mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.host_routes.install_route -ne $RequiredPackage.Install -or $PackageEntry.host_routes.launch_route -ne $RequiredPackage.Launch -or $PackageEntry.host_routes.command_bridge -ne $RequiredPackage.Bridge -or $PackageEntry.host_routes.evidence_pull_route -ne $RequiredPackage.Evidence) {
                throw "shell export package host route mismatch for $($RequiredPackage.Graph)"
            }
            if ($null -eq $PackageEntry.descriptor -or $PackageEntry.descriptor.descriptor_id -ne "studio.shell_descriptor.$($RequiredPackage.Graph)" -or $PackageEntry.descriptor.graph_id -ne $RequiredPackage.Graph -or $PackageEntry.descriptor.shell_id -ne $RequiredPackage.Shell -or $PackageEntry.descriptor.target_host_profile -ne $RequiredPackage.TargetProfile) {
                throw "shell export package descriptor ref mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.descriptor.package_count -lt 1 -or $PackageEntry.descriptor.module_count -lt 1) {
                throw "shell export package descriptor counts mismatch for $($RequiredPackage.Graph)"
            }
            if ($null -eq $PackageEntry.template_manifest -or $PackageEntry.template_manifest.template_id -ne "studio.shell_template.$($RequiredPackage.Graph)" -or $PackageEntry.template_manifest.artifact_id -ne "studio.shell_artifact.$($RequiredPackage.Graph)" -or $PackageEntry.template_manifest.graph_id -ne $RequiredPackage.Graph -or $PackageEntry.template_manifest.shell_id -ne $RequiredPackage.Shell -or $PackageEntry.template_manifest.target_host_profile -ne $RequiredPackage.TargetProfile) {
                throw "shell export package template ref mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.template_manifest.runtime_authority.command_session_authority -ne "rusty.manifold" -or $PackageEntry.template_manifest.runtime_authority.install_launch_evidence_authority -ne "rusty.hostess" -or $PackageEntry.template_manifest.runtime_authority.studio_role -ne "authoring.export_planning") {
                throw "shell export package template authority mismatch for $($RequiredPackage.Graph)"
            }
            if ($PackageEntry.template_manifest.host_routes.install_route -ne $RequiredPackage.Install -or $PackageEntry.template_manifest.host_routes.launch_route -ne $RequiredPackage.Launch -or $PackageEntry.template_manifest.host_routes.command_bridge -ne $RequiredPackage.Bridge -or $PackageEntry.template_manifest.host_routes.evidence_pull_route -ne $RequiredPackage.Evidence) {
                throw "shell export package template host route mismatch for $($RequiredPackage.Graph)"
            }
            $RunbookRequest = @($PackageEntry.runbook_cli_request)
            if ($RunbookRequest.Count -lt 7 -or $RunbookRequest[0] -ne "cargo" -or $RunbookRequest[1] -ne "run" -or $RunbookRequest[2] -ne "-p" -or $RunbookRequest[3] -ne $RequiredPackage.Consumer -or $RunbookRequest[4] -ne "--") {
                throw "shell export package runbook CLI request prefix mismatch for $($RequiredPackage.Graph)"
            }
            if ($RunbookRequest -notcontains "--templates") {
                throw "shell export package runbook CLI request missing --templates for $($RequiredPackage.Graph)"
            }
        }
    }
    Copy-Item -Recurse -LiteralPath $SelectedShellBundleRoot -Destination $DamagedShellBundleRoot
    $DamagedShellHandoffManifestOutput = & cargo run --quiet -p rusty-studio-cli -- shell-handoff-manifest --project $SourceProjectPath --bundle-root $DamagedShellBundleRoot --output $DamagedShellHandoffManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio damaged shell handoff manifest failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $DamagedShellHandoffManifestPath)) {
        throw "damaged shell handoff manifest was not written"
    }
    $DamagedManifest = ($DamagedShellHandoffManifestOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($DamagedManifest.status -ne "pass" -or $DamagedManifest.ready_count -ne 3) {
        throw "damaged shell manifest should be archived before descriptor removal"
    }
    $DamagedPhoneDescriptorPath = Join-Path $DamagedShellBundleRoot "studio.graph.synthetic_wave_phone\descriptors\studio.graph.synthetic_wave_phone.shell-descriptor.json"
    if (-not (Test-Path $DamagedPhoneDescriptorPath)) {
        throw "damaged shell descriptor fixture path missing before removal"
    }
    Remove-Item -LiteralPath $DamagedPhoneDescriptorPath
    $DamagedShellExportPackageOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package --manifest $DamagedShellHandoffManifestPath --output $DamagedShellExportPackagePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio damaged shell export package failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $DamagedShellExportPackagePath)) {
        throw "damaged shell export package was not written"
    }
    $DamagedShellExportPackage = ($DamagedShellExportPackageOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenDamagedShellExportPackage = Get-Content -Raw $DamagedShellExportPackagePath | ConvertFrom-Json
    foreach ($DamagedPackageView in @($DamagedShellExportPackage, $WrittenDamagedShellExportPackage)) {
        if ($DamagedPackageView.'$schema' -ne "rusty.studio.shell_export_package_report.v1") {
            throw "damaged shell export package schema mismatch"
        }
        if ($DamagedPackageView.source_manifest_schema -ne "rusty.studio.shell_handoff_manifest.v1" -or $DamagedPackageView.source_runbook_schema -ne "rusty.studio.shell_runbook_report.v1") {
            throw "damaged shell export package source schema mismatch"
        }
        if ($DamagedPackageView.status -ne "blocked") {
            throw "damaged shell export package should be blocked"
        }
        if ($DamagedPackageView.issue_code -ne "studio.issue.shell_export_package_descriptor_load_failed") {
            throw "damaged shell export package issue mismatch"
        }
        if ($DamagedPackageView.ready_count -ne 2 -or $DamagedPackageView.blocked_count -ne 1 -or $DamagedPackageView.rejected_count -ne 0) {
            throw "damaged shell export package counts mismatch"
        }
        if ($DamagedPackageView.descriptor_count -ne 2 -or $DamagedPackageView.template_manifest_count -ne 3 -or $DamagedPackageView.runbook_entry_count -ne 3) {
            throw "damaged shell export package artifact counts mismatch"
        }
        if ($DamagedPackageView.execution_policy -ne "not_executed.review_only" -or $DamagedPackageView.review_owner -ne "rusty.hostess") {
            throw "damaged shell export package review policy mismatch"
        }
        if ($DamagedPackageView.command_session_authority -ne "rusty.manifold" -or $DamagedPackageView.install_launch_evidence_authority -ne "rusty.hostess" -or $DamagedPackageView.studio_role -ne "authoring.export_planning") {
            throw "damaged shell export package authority mismatch"
        }
        $DamagedPhoneTarget = @($DamagedPackageView.target_summaries | Where-Object { $_.target_kind -eq "phone" }) | Select-Object -First 1
        if ($null -eq $DamagedPhoneTarget -or $DamagedPhoneTarget.ready_count -ne 0 -or $DamagedPhoneTarget.blocked_count -ne 1 -or $DamagedPhoneTarget.descriptor_count -ne 0 -or $DamagedPhoneTarget.template_manifest_count -ne 1) {
            throw "damaged shell export package phone target mismatch"
        }
        if (-not (@($DamagedPhoneTarget.issue_codes) -contains "studio.issue.shell_export_package_descriptor_load_failed")) {
            throw "damaged shell export package phone issue mismatch"
        }
        $DamagedPhoneEntry = @($DamagedPackageView.entries | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_phone" }) | Select-Object -First 1
        if ($null -eq $DamagedPhoneEntry) {
            throw "damaged shell export package missing phone entry"
        }
        if ($DamagedPhoneEntry.status -ne "blocked" -or $DamagedPhoneEntry.responsible_owner -ne "rusty.studio" -or $DamagedPhoneEntry.issue_code -ne "studio.issue.shell_export_package_descriptor_load_failed") {
            throw "damaged shell export package phone entry status mismatch"
        }
        if ($null -ne $DamagedPhoneEntry.descriptor -or $null -eq $DamagedPhoneEntry.template_manifest) {
            throw "damaged shell export package phone refs mismatch"
        }
        if (@($DamagedPhoneEntry.runbook_cli_request).Count -ne 0) {
            throw "damaged shell export package blocked phone should not expose a CLI request"
        }
        if ($DamagedPhoneEntry.template_manifest.host_routes.command_bridge -ne "bridge.adb_intent_file") {
            throw "damaged shell export package phone template host route mismatch"
        }
        foreach ($ReadyGraph in @("studio.graph.synthetic_wave_desktop", "studio.graph.synthetic_wave_headset")) {
            $ReadyEntry = @($DamagedPackageView.entries | Where-Object { $_.graph_id -eq $ReadyGraph }) | Select-Object -First 1
            if ($null -eq $ReadyEntry) {
                throw "damaged shell export package missing ready graph $ReadyGraph"
            }
            if ($ReadyEntry.status -ne "ready" -or $ReadyEntry.responsible_owner -ne "rusty.hostess" -or $null -eq $ReadyEntry.descriptor -or $null -eq $ReadyEntry.template_manifest) {
                throw "damaged shell export package ready graph mismatch for $ReadyGraph"
            }
            if (@($ReadyEntry.runbook_cli_request).Count -lt 7 -or -not (@($ReadyEntry.runbook_cli_request) -contains "--templates")) {
                throw "damaged shell export package ready graph CLI request mismatch for $ReadyGraph"
            }
        }
    }
    Copy-Item -Recurse -LiteralPath $SelectedShellBundleRoot -Destination $DamagedTemplateShellBundleRoot
    $DamagedTemplateShellHandoffManifestOutput = & cargo run --quiet -p rusty-studio-cli -- shell-handoff-manifest --project $SourceProjectPath --bundle-root $DamagedTemplateShellBundleRoot --output $DamagedTemplateShellHandoffManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio damaged template shell handoff manifest failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $DamagedTemplateShellHandoffManifestPath)) {
        throw "damaged template shell handoff manifest was not written"
    }
    $DamagedTemplateManifest = ($DamagedTemplateShellHandoffManifestOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($DamagedTemplateManifest.status -ne "pass" -or $DamagedTemplateManifest.ready_count -ne 3) {
        throw "damaged template shell manifest should be archived before template removal"
    }
    $DamagedPhoneTemplatePath = Join-Path $DamagedTemplateShellBundleRoot "studio.graph.synthetic_wave_phone\shells\phone\studio.graph.synthetic_wave_phone.shell-template.json"
    if (-not (Test-Path $DamagedPhoneTemplatePath)) {
        throw "damaged shell template fixture path missing before removal"
    }
    Remove-Item -LiteralPath $DamagedPhoneTemplatePath
    $DamagedTemplateShellExportPackageOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package --manifest $DamagedTemplateShellHandoffManifestPath --output $DamagedTemplateShellExportPackagePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio damaged template shell export package failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $DamagedTemplateShellExportPackagePath)) {
        throw "damaged template shell export package was not written"
    }
    $DamagedTemplateShellExportPackage = ($DamagedTemplateShellExportPackageOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenDamagedTemplateShellExportPackage = Get-Content -Raw $DamagedTemplateShellExportPackagePath | ConvertFrom-Json
    foreach ($DamagedTemplatePackageView in @($DamagedTemplateShellExportPackage, $WrittenDamagedTemplateShellExportPackage)) {
        if ($DamagedTemplatePackageView.'$schema' -ne "rusty.studio.shell_export_package_report.v1") {
            throw "damaged template shell export package schema mismatch"
        }
        if ($DamagedTemplatePackageView.source_manifest_schema -ne "rusty.studio.shell_handoff_manifest.v1" -or $DamagedTemplatePackageView.source_runbook_schema -ne "rusty.studio.shell_runbook_report.v1") {
            throw "damaged template shell export package source schema mismatch"
        }
        if ($DamagedTemplatePackageView.status -ne "blocked") {
            throw "damaged template shell export package should be blocked"
        }
        if ($DamagedTemplatePackageView.issue_code -ne "studio.issue.shell_export_package_template_load_failed") {
            throw "damaged template shell export package issue mismatch"
        }
        if ($DamagedTemplatePackageView.ready_count -ne 2 -or $DamagedTemplatePackageView.blocked_count -ne 1 -or $DamagedTemplatePackageView.rejected_count -ne 0) {
            throw "damaged template shell export package counts mismatch"
        }
        if ($DamagedTemplatePackageView.descriptor_count -ne 3 -or $DamagedTemplatePackageView.template_manifest_count -ne 2 -or $DamagedTemplatePackageView.runbook_entry_count -ne 3) {
            throw "damaged template shell export package artifact counts mismatch"
        }
        if ($DamagedTemplatePackageView.execution_policy -ne "not_executed.review_only" -or $DamagedTemplatePackageView.review_owner -ne "rusty.hostess") {
            throw "damaged template shell export package review policy mismatch"
        }
        if ($DamagedTemplatePackageView.command_session_authority -ne "rusty.manifold" -or $DamagedTemplatePackageView.install_launch_evidence_authority -ne "rusty.hostess" -or $DamagedTemplatePackageView.studio_role -ne "authoring.export_planning") {
            throw "damaged template shell export package authority mismatch"
        }
        $DamagedTemplatePhoneTarget = @($DamagedTemplatePackageView.target_summaries | Where-Object { $_.target_kind -eq "phone" }) | Select-Object -First 1
        if ($null -eq $DamagedTemplatePhoneTarget -or $DamagedTemplatePhoneTarget.ready_count -ne 0 -or $DamagedTemplatePhoneTarget.blocked_count -ne 1 -or $DamagedTemplatePhoneTarget.descriptor_count -ne 1 -or $DamagedTemplatePhoneTarget.template_manifest_count -ne 0) {
            throw "damaged template shell export package phone target mismatch"
        }
        if (-not (@($DamagedTemplatePhoneTarget.issue_codes) -contains "studio.issue.shell_export_package_template_load_failed")) {
            throw "damaged template shell export package phone issue mismatch"
        }
        $DamagedTemplatePhoneEntry = @($DamagedTemplatePackageView.entries | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_phone" }) | Select-Object -First 1
        if ($null -eq $DamagedTemplatePhoneEntry) {
            throw "damaged template shell export package missing phone entry"
        }
        if ($DamagedTemplatePhoneEntry.status -ne "blocked" -or $DamagedTemplatePhoneEntry.responsible_owner -ne "rusty.studio" -or $DamagedTemplatePhoneEntry.issue_code -ne "studio.issue.shell_export_package_template_load_failed") {
            throw "damaged template shell export package phone entry status mismatch"
        }
        if ($null -eq $DamagedTemplatePhoneEntry.descriptor -or $null -ne $DamagedTemplatePhoneEntry.template_manifest) {
            throw "damaged template shell export package phone refs mismatch"
        }
        if (@($DamagedTemplatePhoneEntry.runbook_cli_request).Count -ne 0) {
            throw "damaged template shell export package blocked phone should not expose a CLI request"
        }
        if ($null -ne $DamagedTemplatePhoneEntry.host_routes.install_route -or $null -ne $DamagedTemplatePhoneEntry.host_routes.launch_route -or $null -ne $DamagedTemplatePhoneEntry.host_routes.command_bridge -or $null -ne $DamagedTemplatePhoneEntry.host_routes.evidence_pull_route) {
            throw "damaged template shell export package blocked phone should not expose host routes"
        }
        foreach ($ReadyGraph in @("studio.graph.synthetic_wave_desktop", "studio.graph.synthetic_wave_headset")) {
            $ReadyEntry = @($DamagedTemplatePackageView.entries | Where-Object { $_.graph_id -eq $ReadyGraph }) | Select-Object -First 1
            if ($null -eq $ReadyEntry) {
                throw "damaged template shell export package missing ready graph $ReadyGraph"
            }
            if ($ReadyEntry.status -ne "ready" -or $ReadyEntry.responsible_owner -ne "rusty.hostess" -or $null -eq $ReadyEntry.descriptor -or $null -eq $ReadyEntry.template_manifest) {
                throw "damaged template shell export package ready graph mismatch for $ReadyGraph"
            }
            if (@($ReadyEntry.runbook_cli_request).Count -lt 7 -or -not (@($ReadyEntry.runbook_cli_request) -contains "--templates")) {
                throw "damaged template shell export package ready graph CLI request mismatch for $ReadyGraph"
            }
        }
    }
    $ShellExportPackageComparisonOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-comparison --baseline $ShellExportPackagePath --candidate $ShellExportPackagePath --output $ShellExportPackageComparisonPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package comparison failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageComparisonPath)) {
        throw "shell export package comparison was not written"
    }
    $ShellExportPackageComparison = ($ShellExportPackageComparisonOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackageComparison = Get-Content -Raw $ShellExportPackageComparisonPath | ConvertFrom-Json
    foreach ($ExportComparisonView in @($ShellExportPackageComparison, $WrittenShellExportPackageComparison)) {
        if ($ExportComparisonView.'$schema' -ne "rusty.studio.shell_export_package_comparison.v1") {
            throw "shell export package comparison schema mismatch"
        }
        if ($ExportComparisonView.baseline_schema -ne "rusty.studio.shell_export_package_report.v1" -or $ExportComparisonView.candidate_schema -ne "rusty.studio.shell_export_package_report.v1") {
            throw "shell export package comparison source schema mismatch"
        }
        if ($ExportComparisonView.baseline_package_id -ne $ShellExportPackage.package_id -or $ExportComparisonView.candidate_package_id -ne $ShellExportPackage.package_id) {
            throw "shell export package comparison package id mismatch"
        }
        if ($ExportComparisonView.baseline_manifest_id -ne $ShellExportPackage.manifest_id -or $ExportComparisonView.candidate_manifest_id -ne $ShellExportPackage.manifest_id) {
            throw "shell export package comparison manifest id mismatch"
        }
        if ($ExportComparisonView.status -ne "unchanged") {
            throw "shell export package comparison should be unchanged"
        }
        if ($null -ne $ExportComparisonView.issue_code) {
            throw "unchanged shell export package comparison should not carry an issue"
        }
        if ($ExportComparisonView.baseline_status -ne "ready" -or $ExportComparisonView.candidate_status -ne "ready") {
            throw "shell export package comparison status inputs mismatch"
        }
        if ($ExportComparisonView.ready_delta -ne 0 -or $ExportComparisonView.blocked_delta -ne 0 -or $ExportComparisonView.rejected_delta -ne 0 -or $ExportComparisonView.descriptor_delta -ne 0 -or $ExportComparisonView.template_manifest_delta -ne 0 -or $ExportComparisonView.runbook_entry_delta -ne 0) {
            throw "shell export package comparison deltas should be zero"
        }
        if (@($ExportComparisonView.entries).Count -ne 3) {
            throw "shell export package comparison entry count mismatch"
        }
        if (@($ExportComparisonView.entries | Where-Object { $_.change -ne "unchanged" }).Count -ne 0) {
            throw "shell export package comparison should not report changed entries"
        }
        if (@($ExportComparisonView.entries | Where-Object { -not $_.baseline_descriptor_present -or -not $_.candidate_descriptor_present -or -not $_.baseline_template_manifest_present -or -not $_.candidate_template_manifest_present -or -not $_.baseline_runbook_cli_request_present -or -not $_.candidate_runbook_cli_request_present }).Count -ne 0) {
            throw "unchanged shell export package comparison entry artifact flags mismatch"
        }
        if (@($ExportComparisonView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell export package comparison checks reported failures"
        }
    }
    $RegressedShellExportPackageComparisonOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-comparison --baseline $ShellExportPackagePath --candidate $DamagedTemplateShellExportPackagePath --output $RegressedShellExportPackageComparisonPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio regressed shell export package comparison failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $RegressedShellExportPackageComparisonPath)) {
        throw "regressed shell export package comparison was not written"
    }
    $RegressedShellExportPackageComparison = ($RegressedShellExportPackageComparisonOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenRegressedShellExportPackageComparison = Get-Content -Raw $RegressedShellExportPackageComparisonPath | ConvertFrom-Json
    foreach ($RegressedExportComparisonView in @($RegressedShellExportPackageComparison, $WrittenRegressedShellExportPackageComparison)) {
        if ($RegressedExportComparisonView.'$schema' -ne "rusty.studio.shell_export_package_comparison.v1") {
            throw "regressed shell export package comparison schema mismatch"
        }
        if ($RegressedExportComparisonView.status -ne "regressed") {
            throw "regressed shell export package comparison status mismatch"
        }
        if ($RegressedExportComparisonView.issue_code -ne "studio.issue.shell_export_package_template_load_failed") {
            throw "regressed shell export package comparison issue mismatch"
        }
        if ($RegressedExportComparisonView.baseline_status -ne "ready" -or $RegressedExportComparisonView.candidate_status -ne "blocked") {
            throw "regressed shell export package comparison input status mismatch"
        }
        if ($RegressedExportComparisonView.ready_delta -ne -1 -or $RegressedExportComparisonView.blocked_delta -ne 1 -or $RegressedExportComparisonView.rejected_delta -ne 0) {
            throw "regressed shell export package comparison readiness deltas mismatch"
        }
        if ($RegressedExportComparisonView.descriptor_delta -ne 0 -or $RegressedExportComparisonView.template_manifest_delta -ne -1 -or $RegressedExportComparisonView.runbook_entry_delta -ne 0) {
            throw "regressed shell export package comparison artifact deltas mismatch"
        }
        if (@($RegressedExportComparisonView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "regressed shell export package comparison checks reported failures"
        }
        $RegressedPhoneEntry = @($RegressedExportComparisonView.entries | Where-Object { $_.graph_id -eq "studio.graph.synthetic_wave_phone" }) | Select-Object -First 1
        if ($null -eq $RegressedPhoneEntry) {
            throw "regressed shell export package comparison missing phone entry"
        }
        if ($RegressedPhoneEntry.change -ne "regressed" -or $RegressedPhoneEntry.score_delta -ne -1 -or $RegressedPhoneEntry.issue_code -ne "studio.issue.shell_export_package_template_load_failed") {
            throw "regressed shell export package comparison phone entry mismatch"
        }
        if (-not $RegressedPhoneEntry.baseline_descriptor_present -or -not $RegressedPhoneEntry.candidate_descriptor_present) {
            throw "regressed shell export package comparison phone descriptor flags mismatch"
        }
        if (-not $RegressedPhoneEntry.baseline_template_manifest_present -or $RegressedPhoneEntry.candidate_template_manifest_present) {
            throw "regressed shell export package comparison phone template flags mismatch"
        }
        if (-not $RegressedPhoneEntry.baseline_runbook_cli_request_present -or $RegressedPhoneEntry.candidate_runbook_cli_request_present) {
            throw "regressed shell export package comparison phone runbook flags mismatch"
        }
        if (@($RegressedExportComparisonView.entries | Where-Object { $_.graph_id -ne "studio.graph.synthetic_wave_phone" -and $_.change -ne "unchanged" }).Count -ne 0) {
            throw "regressed shell export package comparison should keep undamaged entries unchanged"
        }
    }
    $ShellExportPackageBaselineOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline --package-report $ShellExportPackagePath --baseline-id "synthetic-ready-package" --label "Synthetic ready export package baseline" --output $ShellExportPackageBaselinePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package baseline failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageBaselinePath)) {
        throw "shell export package baseline manifest was not written"
    }
    $ShellExportPackageBaseline = ($ShellExportPackageBaselineOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackageBaseline = Get-Content -Raw $ShellExportPackageBaselinePath | ConvertFrom-Json
    foreach ($ExportPackageBaselineView in @($ShellExportPackageBaseline, $WrittenShellExportPackageBaseline)) {
        if ($ExportPackageBaselineView.'$schema' -ne "rusty.studio.shell_export_package_baseline_manifest.v1") {
            throw "shell export package baseline schema mismatch"
        }
        if ($ExportPackageBaselineView.baseline_id -ne "synthetic-ready-package" -or $ExportPackageBaselineView.label -ne "Synthetic ready export package baseline") {
            throw "shell export package baseline identity mismatch"
        }
        if ($ExportPackageBaselineView.package_path -ne $ShellExportPackagePath) {
            throw "shell export package baseline package path mismatch"
        }
        if ($ExportPackageBaselineView.package_schema -ne "rusty.studio.shell_export_package_report.v1") {
            throw "shell export package baseline package schema mismatch"
        }
        if ($ExportPackageBaselineView.package_id -ne $ShellExportPackage.package_id -or $ExportPackageBaselineView.manifest_id -ne $ShellExportPackage.manifest_id) {
            throw "shell export package baseline package identity mismatch"
        }
        if ($ExportPackageBaselineView.project_id -ne $ShellExportPackage.project_id -or $ExportPackageBaselineView.project_revision -ne $ShellExportPackage.project_revision) {
            throw "shell export package baseline project metadata mismatch"
        }
        if ($ExportPackageBaselineView.status -ne "ready" -or $ExportPackageBaselineView.ready_count -ne 3 -or $ExportPackageBaselineView.blocked_count -ne 0 -or $ExportPackageBaselineView.rejected_count -ne 0) {
            throw "shell export package baseline readiness mismatch"
        }
        if ($ExportPackageBaselineView.descriptor_count -ne 3 -or $ExportPackageBaselineView.template_manifest_count -ne 3 -or $ExportPackageBaselineView.runbook_entry_count -ne 3 -or $ExportPackageBaselineView.target_count -ne 3) {
            throw "shell export package baseline count mismatch"
        }
        if ($ExportPackageBaselineView.execution_policy -ne "not_executed.review_only" -or $ExportPackageBaselineView.command_session_authority -ne "rusty.manifold" -or $ExportPackageBaselineView.install_launch_evidence_authority -ne "rusty.hostess" -or $ExportPackageBaselineView.studio_role -ne "authoring.export_planning") {
            throw "shell export package baseline authority mismatch"
        }
    }
    $ShellExportPackageBaselineIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline-index --baseline-manifest $ShellExportPackageBaselinePath --default-baseline-id "synthetic-ready-package" --output $ShellExportPackageBaselineIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package baseline index failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageBaselineIndexPath)) {
        throw "shell export package baseline index was not written"
    }
    $ShellExportPackageBaselineIndex = ($ShellExportPackageBaselineIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackageBaselineIndex = Get-Content -Raw $ShellExportPackageBaselineIndexPath | ConvertFrom-Json
    foreach ($ExportPackageBaselineIndexView in @($ShellExportPackageBaselineIndex, $WrittenShellExportPackageBaselineIndex)) {
        if ($ExportPackageBaselineIndexView.'$schema' -ne "rusty.studio.shell_export_package_baseline_index.v1") {
            throw "shell export package baseline index schema mismatch"
        }
        if ($ExportPackageBaselineIndexView.default_baseline_id -ne "synthetic-ready-package") {
            throw "shell export package baseline index default mismatch"
        }
        if ($ExportPackageBaselineIndexView.baseline_count -ne 1 -or $ExportPackageBaselineIndexView.ready_baseline_count -ne 1 -or $ExportPackageBaselineIndexView.blocked_baseline_count -ne 0 -or $ExportPackageBaselineIndexView.rejected_baseline_count -ne 0) {
            throw "shell export package baseline index counts mismatch"
        }
        if (@($ExportPackageBaselineIndexView.project_ids).Count -ne 1 -or @($ExportPackageBaselineIndexView.project_ids)[0] -ne $ShellExportPackage.project_id) {
            throw "shell export package baseline index project ids mismatch"
        }
        if (@($ExportPackageBaselineIndexView.package_ids).Count -ne 1 -or @($ExportPackageBaselineIndexView.package_ids)[0] -ne $ShellExportPackage.package_id) {
            throw "shell export package baseline index package ids mismatch"
        }
        if (@($ExportPackageBaselineIndexView.manifest_ids).Count -ne 1 -or @($ExportPackageBaselineIndexView.manifest_ids)[0] -ne $ShellExportPackage.manifest_id) {
            throw "shell export package baseline index manifest ids mismatch"
        }
        if (@($ExportPackageBaselineIndexView.entries).Count -ne 1) {
            throw "shell export package baseline index entry count mismatch"
        }
        $ExportPackageBaselineIndexEntry = @($ExportPackageBaselineIndexView.entries)[0]
        if ($ExportPackageBaselineIndexEntry.baseline_id -ne "synthetic-ready-package" -or $ExportPackageBaselineIndexEntry.label -ne "Synthetic ready export package baseline") {
            throw "shell export package baseline index entry identity mismatch"
        }
        if ($ExportPackageBaselineIndexEntry.baseline_manifest_path -ne $ShellExportPackageBaselinePath -or $ExportPackageBaselineIndexEntry.package_path -ne $ShellExportPackagePath) {
            throw "shell export package baseline index entry path mismatch"
        }
        if ($ExportPackageBaselineIndexEntry.package_schema -ne "rusty.studio.shell_export_package_report.v1" -or $ExportPackageBaselineIndexEntry.package_id -ne $ShellExportPackage.package_id) {
            throw "shell export package baseline index entry package mismatch"
        }
        if ($ExportPackageBaselineIndexEntry.status -ne "ready" -or $ExportPackageBaselineIndexEntry.ready_count -ne 3 -or $ExportPackageBaselineIndexEntry.blocked_count -ne 0 -or $ExportPackageBaselineIndexEntry.rejected_count -ne 0 -or $ExportPackageBaselineIndexEntry.descriptor_count -ne 3 -or $ExportPackageBaselineIndexEntry.template_manifest_count -ne 3 -or $ExportPackageBaselineIndexEntry.runbook_entry_count -ne 3 -or $ExportPackageBaselineIndexEntry.target_count -ne 3) {
            throw "shell export package baseline index entry readiness mismatch"
        }
    }
    $ShellExportPackageBaselineSelectionOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline-selection --baseline-index $ShellExportPackageBaselineIndexPath --baseline-id "synthetic-ready-package" --output $ShellExportPackageBaselineSelectionPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package baseline selection failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageBaselineSelectionPath)) {
        throw "shell export package baseline selection was not written"
    }
    $ShellExportPackageBaselineSelection = ($ShellExportPackageBaselineSelectionOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackageBaselineSelection = Get-Content -Raw $ShellExportPackageBaselineSelectionPath | ConvertFrom-Json
    foreach ($ExportPackageBaselineSelectionView in @($ShellExportPackageBaselineSelection, $WrittenShellExportPackageBaselineSelection)) {
        if ($ExportPackageBaselineSelectionView.'$schema' -ne "rusty.studio.shell_export_package_baseline_selection.v1") {
            throw "shell export package baseline selection schema mismatch"
        }
        if ($ExportPackageBaselineSelectionView.source_index_schema -ne "rusty.studio.shell_export_package_baseline_index.v1") {
            throw "shell export package baseline selection source index schema mismatch"
        }
        if ($ExportPackageBaselineSelectionView.index_path -ne $ShellExportPackageBaselineIndexPath) {
            throw "shell export package baseline selection index path mismatch"
        }
        if ($ExportPackageBaselineSelectionView.requested_baseline_id -ne "synthetic-ready-package" -or $ExportPackageBaselineSelectionView.default_baseline_id -ne "synthetic-ready-package" -or $ExportPackageBaselineSelectionView.selected_baseline_id -ne "synthetic-ready-package") {
            throw "shell export package baseline selection id mismatch"
        }
        if ($ExportPackageBaselineSelectionView.status -ne "selected") {
            throw "shell export package baseline selection status mismatch"
        }
        if ($null -ne $ExportPackageBaselineSelectionView.issue_code) {
            throw "selected shell export package baseline selection should not carry an issue"
        }
        if ($ExportPackageBaselineSelectionView.baseline_count -ne 1 -or $ExportPackageBaselineSelectionView.ready_baseline_count -ne 1 -or $ExportPackageBaselineSelectionView.blocked_baseline_count -ne 0 -or $ExportPackageBaselineSelectionView.rejected_baseline_count -ne 0) {
            throw "shell export package baseline selection counts mismatch"
        }
        if (@($ExportPackageBaselineSelectionView.entries).Count -ne 1) {
            throw "shell export package baseline selection entry count mismatch"
        }
        $ExportPackageBaselineSelectionEntry = @($ExportPackageBaselineSelectionView.entries)[0]
        $ExportPackageBaselineSelectionEntryDefault = $ExportPackageBaselineSelectionEntry.PSObject.Properties["default"].Value
        if ($ExportPackageBaselineSelectionEntry.baseline_id -ne "synthetic-ready-package" -or -not $ExportPackageBaselineSelectionEntry.selected -or -not $ExportPackageBaselineSelectionEntryDefault) {
            throw "shell export package baseline selection entry selection mismatch"
        }
        if ($ExportPackageBaselineSelectionEntry.package_path -ne $ShellExportPackagePath -or $ExportPackageBaselineSelectionEntry.package_id -ne $ShellExportPackage.package_id) {
            throw "shell export package baseline selection entry package mismatch"
        }
    }
    $ShellExportPackageIndexComparisonOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-comparison --baseline-index $ShellExportPackageBaselineIndexPath --baseline-id "synthetic-ready-package" --candidate $ShellExportPackagePath --output $ShellExportPackageIndexComparisonPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio indexed shell export package comparison failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageIndexComparisonPath)) {
        throw "indexed shell export package comparison was not written"
    }
    $ShellExportPackageIndexComparison = ($ShellExportPackageIndexComparisonOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellExportPackageIndexComparison = Get-Content -Raw $ShellExportPackageIndexComparisonPath | ConvertFrom-Json
    foreach ($ExportPackageIndexComparisonView in @($ShellExportPackageIndexComparison, $WrittenShellExportPackageIndexComparison)) {
        if ($ExportPackageIndexComparisonView.'$schema' -ne "rusty.studio.shell_export_package_comparison.v1") {
            throw "indexed shell export package comparison schema mismatch"
        }
        if ($ExportPackageIndexComparisonView.baseline_identity_schema -ne "rusty.studio.shell_export_package_baseline_manifest.v1") {
            throw "indexed shell export package comparison baseline identity schema mismatch"
        }
        if ($ExportPackageIndexComparisonView.baseline_id -ne "synthetic-ready-package" -or $ExportPackageIndexComparisonView.baseline_label -ne "Synthetic ready export package baseline") {
            throw "indexed shell export package comparison baseline identity mismatch"
        }
        if ($ExportPackageIndexComparisonView.baseline_package_path -ne $ShellExportPackagePath) {
            throw "indexed shell export package comparison baseline package path mismatch"
        }
        if ($ExportPackageIndexComparisonView.baseline_index_schema -ne "rusty.studio.shell_export_package_baseline_index.v1" -or $ExportPackageIndexComparisonView.baseline_index_path -ne $ShellExportPackageBaselineIndexPath) {
            throw "indexed shell export package comparison baseline index mismatch"
        }
        if ($ExportPackageIndexComparisonView.baseline_index_default_baseline_id -ne "synthetic-ready-package" -or $ExportPackageIndexComparisonView.baseline_index_selected_baseline_id -ne "synthetic-ready-package") {
            throw "indexed shell export package comparison baseline index selection mismatch"
        }
        if ($ExportPackageIndexComparisonView.status -ne "unchanged") {
            throw "indexed shell export package comparison should be unchanged"
        }
        if (@($ExportPackageIndexComparisonView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "indexed shell export package comparison checks reported failures"
        }
        if (@($ExportPackageIndexComparisonView.checks | Where-Object { $_.check_id -like "*baseline_identity*" }).Count -lt 6) {
            throw "indexed shell export package comparison did not include baseline identity checks"
        }
        if (@($ExportPackageIndexComparisonView.checks | Where-Object { $_.check_id -like "*baseline_index*" }).Count -lt 7) {
            throw "indexed shell export package comparison did not include baseline index checks"
        }
    }
    $DamagedTemplateShellExportPackageBaselineOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline --package-report $DamagedTemplateShellExportPackagePath --baseline-id "synthetic-blocked-package" --label "Synthetic blocked export package baseline" --output $DamagedTemplateShellExportPackageBaselinePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio damaged template shell export package baseline failed with exit code $LASTEXITCODE"
    }
    $DamagedTemplateShellExportPackageBaseline = ($DamagedTemplateShellExportPackageBaselineOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($DamagedTemplateShellExportPackageBaseline.'$schema' -ne "rusty.studio.shell_export_package_baseline_manifest.v1") {
        throw "damaged template shell export package baseline schema mismatch"
    }
    if ($DamagedTemplateShellExportPackageBaseline.status -ne "blocked" -or $DamagedTemplateShellExportPackageBaseline.ready_count -ne 2 -or $DamagedTemplateShellExportPackageBaseline.blocked_count -ne 1) {
        throw "damaged template shell export package baseline readiness mismatch"
    }
    $ShellExportPackageMultiBaselineIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline-index-append --baseline-index $ShellExportPackageBaselineIndexPath --baseline-manifest $DamagedTemplateShellExportPackageBaselinePath --default-baseline-id "synthetic-blocked-package" --output $ShellExportPackageMultiBaselineIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package baseline index append failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackageMultiBaselineIndexPath)) {
        throw "multi-baseline shell export package index was not written"
    }
    $ShellExportPackageMultiBaselineIndex = ($ShellExportPackageMultiBaselineIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($ShellExportPackageMultiBaselineIndex.'$schema' -ne "rusty.studio.shell_export_package_baseline_index.v1") {
        throw "multi-baseline shell export package index schema mismatch"
    }
    if ($ShellExportPackageMultiBaselineIndex.default_baseline_id -ne "synthetic-blocked-package") {
        throw "multi-baseline shell export package index default mismatch"
    }
    if ($ShellExportPackageMultiBaselineIndex.baseline_count -ne 2 -or $ShellExportPackageMultiBaselineIndex.ready_baseline_count -ne 1 -or $ShellExportPackageMultiBaselineIndex.blocked_baseline_count -ne 1 -or $ShellExportPackageMultiBaselineIndex.rejected_baseline_count -ne 0) {
        throw "multi-baseline shell export package index counts mismatch"
    }
    $MultiExportPackageSelectionOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline-selection --baseline-index $ShellExportPackageMultiBaselineIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio multi-baseline shell export package selection failed with exit code $LASTEXITCODE"
    }
    $MultiExportPackageSelection = ($MultiExportPackageSelectionOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($MultiExportPackageSelection.status -ne "selected" -or $MultiExportPackageSelection.default_baseline_id -ne "synthetic-blocked-package" -or $MultiExportPackageSelection.selected_baseline_id -ne "synthetic-blocked-package") {
        throw "multi-baseline shell export package selection default mismatch"
    }
    $ShellExportPackagePromotedBaselineIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-export-package-baseline-index-promote --baseline-index $ShellExportPackageMultiBaselineIndexPath --baseline-id "synthetic-ready-package" --output $ShellExportPackagePromotedBaselineIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell export package baseline index promote failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellExportPackagePromotedBaselineIndexPath)) {
        throw "promoted shell export package index was not written"
    }
    $ShellExportPackagePromotedBaselineIndex = ($ShellExportPackagePromotedBaselineIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($ShellExportPackagePromotedBaselineIndex.default_baseline_id -ne "synthetic-ready-package") {
        throw "promoted shell export package index default mismatch"
    }
    if ($ShellExportPackagePromotedBaselineIndex.baseline_count -ne 2 -or $ShellExportPackagePromotedBaselineIndex.ready_baseline_count -ne 1 -or $ShellExportPackagePromotedBaselineIndex.blocked_baseline_count -ne 1) {
        throw "promoted shell export package index counts mismatch"
    }
    $MissingExportPackagePromoteResult = Invoke-NativeExpectedFailure "cargo" @(
        "run",
        "--quiet",
        "-p",
        "rusty-studio-cli",
        "--",
        "shell-export-package-baseline-index-promote",
        "--baseline-index",
        $ShellExportPackageMultiBaselineIndexPath,
        "--baseline-id",
        "synthetic-missing-package"
    )
    if ($MissingExportPackagePromoteResult.ExitCode -eq 0) {
        throw "missing shell export package baseline promote should fail"
    }
    if ((($MissingExportPackagePromoteResult.Output -join [Environment]::NewLine) -notmatch "--baseline-id was not found in --baseline-index")) {
        throw "missing shell export package baseline promote error mismatch"
    }
}
