function Invoke-StudioHostessStagingCheck {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$true)]
        [string]$ProjectedMotionShellHandoffEvidencePath,
        [Parameter(Mandatory=$true)]
        [string]$ProjectedMotionShellHandoffReviewPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHandoffManifestPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessHandoffPackagePath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessOwnerIntakePath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceArchiveManifestPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceComparisonPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceManifestPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceMultiIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptancePath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptancePromotedIndexPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceRouteDriftComparisonPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceRouteDriftPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingAcceptanceSelectionPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingExecutionRequestPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingFilePlanPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingHandoffPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellHostessStagingPreviewPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellReleaseCandidateReviewManifestPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellReleaseCandidateReviewPath,
        [Parameter(Mandatory=$true)]
        [string]$ShellReleaseCandidateReviewPromotedIndexPath
    )

    $ShellHostessHandoffPackageOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-handoff-package --review-index $ShellReleaseCandidateReviewPromotedIndexPath --candidate-id "synthetic-ready-candidate" --output $ShellHostessHandoffPackagePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess handoff package failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessHandoffPackagePath)) {
        throw "shell Hostess handoff package was not written"
    }
    $ShellHostessHandoffPackage = ($ShellHostessHandoffPackageOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessHandoffPackage = Get-Content -Raw $ShellHostessHandoffPackagePath | ConvertFrom-Json
    foreach ($HostessPackageView in @($ShellHostessHandoffPackage, $WrittenShellHostessHandoffPackage)) {
        if ($HostessPackageView.'$schema' -ne "rusty.studio.shell_hostess_handoff_package.v1") {
            throw "shell Hostess handoff package schema mismatch"
        }
        if ($HostessPackageView.source_index_schema -ne "rusty.studio.shell_release_candidate_review_index.v1" -or $HostessPackageView.index_path -ne $ShellReleaseCandidateReviewPromotedIndexPath) {
            throw "shell Hostess handoff package source index mismatch"
        }
        if ($HostessPackageView.status -ne "ready" -or $null -ne $HostessPackageView.issue_code) {
            throw "shell Hostess handoff package should be ready"
        }
        if ($HostessPackageView.selected_candidate_id -ne "synthetic-ready-candidate" -or $HostessPackageView.candidate_id -ne "synthetic-ready-candidate") {
            throw "shell Hostess handoff package selected candidate mismatch"
        }
        if ($HostessPackageView.candidate_manifest_schema -ne "rusty.studio.shell_release_candidate_review_manifest.v1" -or $HostessPackageView.candidate_manifest_path -ne $ShellReleaseCandidateReviewManifestPath) {
            throw "shell Hostess handoff package candidate manifest mismatch"
        }
        if ($HostessPackageView.review_schema -ne "rusty.studio.shell_release_candidate_review.v1" -or $HostessPackageView.review_path -ne $ShellReleaseCandidateReviewPath) {
            throw "shell Hostess handoff package review path mismatch"
        }
        if ($HostessPackageView.handoff_manifest_schema -ne "rusty.studio.shell_handoff_manifest.v1" -or $HostessPackageView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess handoff package manifest path mismatch"
        }
        if ($HostessPackageView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessPackageView.project_id -ne "studio.project.synthetic_wave" -or $HostessPackageView.project_revision -ne 1) {
            throw "shell Hostess handoff package project identity mismatch"
        }
        if ($HostessPackageView.execution_policy -ne "not_executed.review_only" -or $HostessPackageView.handoff_owner -ne "rusty.hostess" -or $HostessPackageView.review_owner -ne "rusty.hostess") {
            throw "shell Hostess handoff package authority owner mismatch"
        }
        if ($HostessPackageView.command_session_authority -ne "rusty.manifold" -or $HostessPackageView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessPackageView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess handoff package runtime authority mismatch"
        }
        if ($HostessPackageView.handoff_ready_count -ne 3 -or $HostessPackageView.handoff_failed_count -ne 0 -or $HostessPackageView.handoff_missing_bundle_count -ne 0) {
            throw "shell Hostess handoff package handoff counts mismatch"
        }
        if ($HostessPackageView.acceptance_baseline_id -ne "synthetic-ready" -or $HostessPackageView.acceptance_baseline_status -ne "selected" -or $HostessPackageView.acceptance_comparison_status -ne "unchanged") {
            throw "shell Hostess handoff package acceptance baseline mismatch"
        }
        if ($HostessPackageView.export_package_baseline_id -ne "synthetic-ready-package" -or $HostessPackageView.export_package_baseline_status -ne "selected" -or $HostessPackageView.export_package_comparison_status -ne "unchanged") {
            throw "shell Hostess handoff package export package baseline mismatch"
        }
        foreach ($ActionId in @("hostess.review_release_candidate", "hostess.stage_generated_shells", "manifold.review_command_session_contract", "hostess.collect_install_launch_evidence")) {
            if (@($HostessPackageView.required_owner_actions | Where-Object { $_.action_id -eq $ActionId -and $_.status -eq "ready" -and $_.prohibited_in_studio -eq $true }).Count -ne 1) {
                throw "shell Hostess handoff package missing ready owner action $ActionId"
            }
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessPackageView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess handoff package missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessPackageView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess handoff package should not contain failed checks"
        }
    }
    $ShellHostessOwnerIntakeOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-owner-intake --package $ShellHostessHandoffPackagePath --output $ShellHostessOwnerIntakePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess owner intake failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessOwnerIntakePath)) {
        throw "shell Hostess owner intake was not written"
    }
    $ShellHostessOwnerIntake = ($ShellHostessOwnerIntakeOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessOwnerIntake = Get-Content -Raw $ShellHostessOwnerIntakePath | ConvertFrom-Json
    foreach ($HostessOwnerIntakeView in @($ShellHostessOwnerIntake, $WrittenShellHostessOwnerIntake)) {
        if ($HostessOwnerIntakeView.'$schema' -ne "rusty.studio.shell_hostess_owner_intake.v1") {
            throw "shell Hostess owner intake schema mismatch"
        }
        if ($HostessOwnerIntakeView.source_package_schema -ne "rusty.studio.shell_hostess_handoff_package.v1" -or $HostessOwnerIntakeView.package_path -ne $ShellHostessHandoffPackagePath) {
            throw "shell Hostess owner intake source package mismatch"
        }
        if ($HostessOwnerIntakeView.status -ne "ready" -or $null -ne $HostessOwnerIntakeView.issue_code) {
            throw "shell Hostess owner intake should be ready"
        }
        if ($HostessOwnerIntakeView.selected_candidate_id -ne "synthetic-ready-candidate" -or $HostessOwnerIntakeView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess owner intake selected candidate mismatch"
        }
        if ($HostessOwnerIntakeView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessOwnerIntakeView.project_id -ne "studio.project.synthetic_wave" -or $HostessOwnerIntakeView.project_revision -ne 1) {
            throw "shell Hostess owner intake project identity mismatch"
        }
        if ($HostessOwnerIntakeView.execution_policy -ne "not_executed.request_only" -or $HostessOwnerIntakeView.intake_owner -ne "rusty.hostess" -or $HostessOwnerIntakeView.handoff_owner -ne "rusty.hostess" -or $HostessOwnerIntakeView.review_owner -ne "rusty.hostess") {
            throw "shell Hostess owner intake authority owner mismatch"
        }
        if ($HostessOwnerIntakeView.command_session_authority -ne "rusty.manifold" -or $HostessOwnerIntakeView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessOwnerIntakeView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess owner intake runtime authority mismatch"
        }
        if ($HostessOwnerIntakeView.handoff_ready_count -ne 3 -or $HostessOwnerIntakeView.handoff_failed_count -ne 0 -or $HostessOwnerIntakeView.handoff_missing_bundle_count -ne 0) {
            throw "shell Hostess owner intake handoff counts mismatch"
        }
        if ($HostessOwnerIntakeView.acceptance_baseline_id -ne "synthetic-ready" -or $HostessOwnerIntakeView.acceptance_baseline_status -ne "selected" -or $HostessOwnerIntakeView.acceptance_comparison_status -ne "unchanged") {
            throw "shell Hostess owner intake acceptance baseline mismatch"
        }
        if ($HostessOwnerIntakeView.export_package_baseline_id -ne "synthetic-ready-package" -or $HostessOwnerIntakeView.export_package_baseline_status -ne "selected" -or $HostessOwnerIntakeView.export_package_comparison_status -ne "unchanged") {
            throw "shell Hostess owner intake export package baseline mismatch"
        }
        if ($HostessOwnerIntakeView.source_owner_action_count -ne 4 -or $HostessOwnerIntakeView.ready_assignment_count -ne 4 -or $HostessOwnerIntakeView.blocked_assignment_count -ne 0 -or $HostessOwnerIntakeView.hostess_ready_action_count -ne 3 -or $HostessOwnerIntakeView.manifold_ready_action_count -ne 1) {
            throw "shell Hostess owner intake assignment counts mismatch"
        }
        foreach ($Assignment in @(
            @{ ActionId = "hostess.review_release_candidate"; Owner = "rusty.hostess"; RequestKind = "hostess_owner_action_request" },
            @{ ActionId = "hostess.stage_generated_shells"; Owner = "rusty.hostess"; RequestKind = "hostess_owner_action_request" },
            @{ ActionId = "manifold.review_command_session_contract"; Owner = "rusty.manifold"; RequestKind = "manifold_owner_review_request" },
            @{ ActionId = "hostess.collect_install_launch_evidence"; Owner = "rusty.hostess"; RequestKind = "hostess_owner_action_request" }
        )) {
            if (@($HostessOwnerIntakeView.assignments | Where-Object { $_.action_id -eq $Assignment.ActionId -and $_.owner -eq $Assignment.Owner -and $_.request_kind -eq $Assignment.RequestKind -and $_.status -eq "ready" -and $_.prohibited_in_studio -eq $true }).Count -ne 1) {
                throw "shell Hostess owner intake missing ready assignment $($Assignment.ActionId)"
            }
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessOwnerIntakeView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess owner intake missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessOwnerIntakeView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess owner intake should not contain failed checks"
        }
    }
    $ShellHostessStagingPreviewOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-preview --intake $ShellHostessOwnerIntakePath --output $ShellHostessStagingPreviewPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging preview failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingPreviewPath)) {
        throw "shell Hostess staging preview was not written"
    }
    $ShellHostessStagingPreview = ($ShellHostessStagingPreviewOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingPreview = Get-Content -Raw $ShellHostessStagingPreviewPath | ConvertFrom-Json
    foreach ($HostessStagingPreviewView in @($ShellHostessStagingPreview, $WrittenShellHostessStagingPreview)) {
        if ($HostessStagingPreviewView.'$schema' -ne "rusty.studio.shell_hostess_staging_preview_manifest.v1") {
            throw "shell Hostess staging preview schema mismatch"
        }
        if ($HostessStagingPreviewView.source_intake_schema -ne "rusty.studio.shell_hostess_owner_intake.v1" -or $HostessStagingPreviewView.source_handoff_manifest_schema -ne "rusty.studio.shell_handoff_manifest.v1") {
            throw "shell Hostess staging preview source schema mismatch"
        }
        if ($HostessStagingPreviewView.intake_path -ne $ShellHostessOwnerIntakePath -or $HostessStagingPreviewView.package_path -ne $ShellHostessHandoffPackagePath -or $HostessStagingPreviewView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess staging preview source path mismatch"
        }
        if ($HostessStagingPreviewView.status -ne "ready" -or $null -ne $HostessStagingPreviewView.issue_code) {
            throw "shell Hostess staging preview should be ready"
        }
        if ($HostessStagingPreviewView.selected_candidate_id -ne "synthetic-ready-candidate") {
            throw "shell Hostess staging preview selected candidate mismatch"
        }
        if ($HostessStagingPreviewView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingPreviewView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingPreviewView.project_revision -ne 1) {
            throw "shell Hostess staging preview project identity mismatch"
        }
        if ($HostessStagingPreviewView.execution_policy -ne "not_executed.preview_only" -or $HostessStagingPreviewView.staging_owner -ne "rusty.hostess") {
            throw "shell Hostess staging preview policy owner mismatch"
        }
        if ($HostessStagingPreviewView.command_session_authority -ne "rusty.manifold" -or $HostessStagingPreviewView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingPreviewView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging preview runtime authority mismatch"
        }
        if ($HostessStagingPreviewView.assignment_count -ne 4 -or $HostessStagingPreviewView.ready_assignment_count -ne 4 -or $HostessStagingPreviewView.blocked_assignment_count -ne 0 -or $HostessStagingPreviewView.ready_group_count -ne 4 -or $HostessStagingPreviewView.blocked_group_count -ne 0) {
            throw "shell Hostess staging preview counts mismatch"
        }
        if ($HostessStagingPreviewView.expected_artifact_count -lt 18) {
            throw "shell Hostess staging preview should expose expected staging artifacts"
        }
        foreach ($RouteKind in @("hostess.review.release_candidate", "hostess.stage.generated_shells", "manifold.review.command_session_contract", "hostess.collect.install_launch_evidence")) {
            if (@($HostessStagingPreviewView.groups | Where-Object { $_.route_kind -eq $RouteKind -and $_.status -eq "ready" -and $_.prohibited_in_studio -eq $true }).Count -ne 1) {
                throw "shell Hostess staging preview missing ready route group $RouteKind"
            }
        }
        $StageGroup = @($HostessStagingPreviewView.groups | Where-Object { $_.route_kind -eq "hostess.stage.generated_shells" })[0]
        foreach ($ArtifactKind in @("hostess_handoff_package", "hostess_owner_intake", "shell_handoff_manifest", "shell_bundle_dir", "shell_descriptor", "shell_template_manifest")) {
            if (-not (@($StageGroup.expected_artifacts | ForEach-Object { $_.artifact_kind }) -contains $ArtifactKind)) {
                throw "shell Hostess staging preview stage group missing artifact $ArtifactKind"
            }
        }
        $ManifoldGroup = @($HostessStagingPreviewView.groups | Where-Object { $_.route_kind -eq "manifold.review.command_session_contract" })[0]
        if ($ManifoldGroup.owner -ne "rusty.manifold" -or $ManifoldGroup.request_kind -ne "manifold_owner_review_request") {
            throw "shell Hostess staging preview Manifold group owner mismatch"
        }
        if (@($ManifoldGroup.expected_artifacts | Where-Object { $_.route_hint -ne $null }).Count -lt 1) {
            throw "shell Hostess staging preview Manifold group should expose route hints"
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingPreviewView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging preview missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessStagingPreviewView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging preview should not contain failed checks"
        }
    }
    $ShellHostessStagingFilePlanOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-file-plan --preview $ShellHostessStagingPreviewPath --output $ShellHostessStagingFilePlanPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging file plan failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingFilePlanPath)) {
        throw "shell Hostess staging file plan was not written"
    }
    $ShellHostessStagingFilePlan = ($ShellHostessStagingFilePlanOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingFilePlan = Get-Content -Raw $ShellHostessStagingFilePlanPath | ConvertFrom-Json
    foreach ($HostessStagingFilePlanView in @($ShellHostessStagingFilePlan, $WrittenShellHostessStagingFilePlan)) {
        if ($HostessStagingFilePlanView.'$schema' -ne "rusty.studio.shell_hostess_staging_file_plan.v1") {
            throw "shell Hostess staging file plan schema mismatch"
        }
        if ($HostessStagingFilePlanView.source_preview_schema -ne "rusty.studio.shell_hostess_staging_preview_manifest.v1" -or $HostessStagingFilePlanView.preview_path -ne $ShellHostessStagingPreviewPath) {
            throw "shell Hostess staging file plan source preview mismatch"
        }
        if ($HostessStagingFilePlanView.intake_path -ne $ShellHostessOwnerIntakePath -or $HostessStagingFilePlanView.package_path -ne $ShellHostessHandoffPackagePath -or $HostessStagingFilePlanView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess staging file plan source path mismatch"
        }
        if ($HostessStagingFilePlanView.status -ne "ready" -or $null -ne $HostessStagingFilePlanView.issue_code) {
            throw "shell Hostess staging file plan should be ready"
        }
        if ($HostessStagingFilePlanView.selected_candidate_id -ne "synthetic-ready-candidate") {
            throw "shell Hostess staging file plan selected candidate mismatch"
        }
        if ($HostessStagingFilePlanView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingFilePlanView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingFilePlanView.project_revision -ne 1) {
            throw "shell Hostess staging file plan project identity mismatch"
        }
        if ($HostessStagingFilePlanView.execution_policy -ne "not_executed.dry_run_only" -or $HostessStagingFilePlanView.staging_owner -ne "rusty.hostess") {
            throw "shell Hostess staging file plan policy owner mismatch"
        }
        if ($HostessStagingFilePlanView.command_session_authority -ne "rusty.manifold" -or $HostessStagingFilePlanView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingFilePlanView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging file plan runtime authority mismatch"
        }
        if ($HostessStagingFilePlanView.preview_group_count -ne 4 -or $HostessStagingFilePlanView.ready_preview_group_count -ne 4 -or $HostessStagingFilePlanView.blocked_preview_group_count -ne 0) {
            throw "shell Hostess staging file plan preview group counts mismatch"
        }
        if ($HostessStagingFilePlanView.source_artifact_count -le $HostessStagingFilePlanView.planned_file_count -or $HostessStagingFilePlanView.planned_file_count -ne 17 -or $HostessStagingFilePlanView.duplicate_artifact_count -lt 1) {
            throw "shell Hostess staging file plan artifact dedupe counts mismatch"
        }
        if ($HostessStagingFilePlanView.request_count -ne 4 -or $HostessStagingFilePlanView.ready_request_count -ne 4 -or $HostessStagingFilePlanView.blocked_request_count -ne 0 -or $HostessStagingFilePlanView.target_request_count -ne 3 -or $HostessStagingFilePlanView.shared_request_count -ne 1) {
            throw "shell Hostess staging file plan request counts mismatch"
        }
        $SharedRequest = @($HostessStagingFilePlanView.requests | Where-Object { $_.target_key -eq "shared" })[0]
        if ($null -eq $SharedRequest -or $SharedRequest.status -ne "ready" -or $SharedRequest.request_kind -ne "hostess_shared_staging_file_plan") {
            throw "shell Hostess staging file plan missing shared request"
        }
        foreach ($ArtifactKind in @("candidate_manifest", "release_candidate_review", "hostess_handoff_package", "hostess_owner_intake", "shell_handoff_manifest")) {
            if (-not (@($SharedRequest.planned_files | ForEach-Object { $_.artifact_kind }) -contains $ArtifactKind)) {
                throw "shell Hostess staging file plan shared request missing $ArtifactKind"
            }
        }
        foreach ($Target in @("desktop", "phone", "quest")) {
            $TargetRequest = @($HostessStagingFilePlanView.requests | Where-Object { $_.target_key -like "$Target/*" })[0]
            if ($null -eq $TargetRequest -or $TargetRequest.status -ne "ready" -or $TargetRequest.request_kind -ne "hostess_target_staging_file_plan" -or $TargetRequest.planned_file_count -ne 4) {
                throw "shell Hostess staging file plan missing target request $Target"
            }
            foreach ($ArtifactKind in @("shell_bundle_dir", "shell_descriptor", "manifold_shell_handoff", "shell_template_manifest")) {
                if (-not (@($TargetRequest.planned_files | ForEach-Object { $_.artifact_kind }) -contains $ArtifactKind)) {
                    throw "shell Hostess staging file plan target $Target missing artifact $ArtifactKind"
                }
            }
            if (@($TargetRequest.planned_files | Where-Object { $_.destination_path -like "hostess-staging/targets/$Target/*" }).Count -ne 4) {
                throw "shell Hostess staging file plan target $Target destination mismatch"
            }
        }
        if (@($HostessStagingFilePlanView.requests | ForEach-Object { $_.planned_files } | Where-Object { @($_.source_action_ids).Count -eq 0 -or @($_.source_route_kinds).Count -eq 0 }).Count -ne 0) {
            throw "shell Hostess staging file plan should retain source provenance"
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingFilePlanView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging file plan missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessStagingFilePlanView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging file plan should not contain failed checks"
        }
    }
    $ShellHostessStagingHandoffOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-handoff --file-plan $ShellHostessStagingFilePlanPath --output $ShellHostessStagingHandoffPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging handoff failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingHandoffPath)) {
        throw "shell Hostess staging handoff was not written"
    }
    $ShellHostessStagingHandoff = ($ShellHostessStagingHandoffOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingHandoff = Get-Content -Raw $ShellHostessStagingHandoffPath | ConvertFrom-Json
    foreach ($HostessStagingHandoffView in @($ShellHostessStagingHandoff, $WrittenShellHostessStagingHandoff)) {
        if ($HostessStagingHandoffView.'$schema' -ne "rusty.studio.shell_hostess_staging_handoff_envelope.v1") {
            throw "shell Hostess staging handoff schema mismatch"
        }
        if ($HostessStagingHandoffView.source_file_plan_schema -ne "rusty.studio.shell_hostess_staging_file_plan.v1" -or $HostessStagingHandoffView.file_plan_path -ne $ShellHostessStagingFilePlanPath) {
            throw "shell Hostess staging handoff source file plan mismatch"
        }
        if ($HostessStagingHandoffView.preview_path -ne $ShellHostessStagingPreviewPath -or $HostessStagingHandoffView.intake_path -ne $ShellHostessOwnerIntakePath -or $HostessStagingHandoffView.package_path -ne $ShellHostessHandoffPackagePath -or $HostessStagingHandoffView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess staging handoff source path mismatch"
        }
        if ($HostessStagingHandoffView.status -ne "ready" -or $null -ne $HostessStagingHandoffView.issue_code) {
            throw "shell Hostess staging handoff should be ready"
        }
        if ($HostessStagingHandoffView.selected_candidate_id -ne "synthetic-ready-candidate" -or $HostessStagingHandoffView.envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1") {
            throw "shell Hostess staging handoff selected candidate or envelope id mismatch"
        }
        if ($HostessStagingHandoffView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingHandoffView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingHandoffView.project_revision -ne 1) {
            throw "shell Hostess staging handoff project identity mismatch"
        }
        if ($HostessStagingHandoffView.execution_policy -ne "not_executed.handoff_only" -or $HostessStagingHandoffView.handoff_owner -ne "rusty.hostess" -or $HostessStagingHandoffView.staging_owner -ne "rusty.hostess") {
            throw "shell Hostess staging handoff policy owner mismatch"
        }
        if ($HostessStagingHandoffView.command_session_authority -ne "rusty.manifold" -or $HostessStagingHandoffView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingHandoffView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging handoff runtime authority mismatch"
        }
        if ($HostessStagingHandoffView.planned_file_count -ne 17 -or $HostessStagingHandoffView.request_count -ne 4 -or $HostessStagingHandoffView.ready_request_count -ne 4 -or $HostessStagingHandoffView.blocked_request_count -ne 0 -or $HostessStagingHandoffView.target_request_count -ne 3 -or $HostessStagingHandoffView.shared_request_count -ne 1) {
            throw "shell Hostess staging handoff request counts mismatch"
        }
        if ($HostessStagingHandoffView.instruction_count -ne 4 -or $HostessStagingHandoffView.ready_instruction_count -ne 4 -or $HostessStagingHandoffView.blocked_instruction_count -ne 0) {
            throw "shell Hostess staging handoff instruction counts mismatch"
        }
        if ($HostessStagingHandoffView.provenance.checksum_algorithm -ne "fnv1a64.studio_staging_file_plan.v1" -or $HostessStagingHandoffView.provenance.plan_checksum.Length -ne 16) {
            throw "shell Hostess staging handoff checksum mismatch"
        }
        foreach ($ArtifactKind in @("shell_template_manifest", "hostess_owner_intake")) {
            if (-not (@($HostessStagingHandoffView.provenance.source_artifact_kinds) -contains $ArtifactKind)) {
                throw "shell Hostess staging handoff provenance missing artifact $ArtifactKind"
            }
        }
        foreach ($ActionId in @("hostess.stage_generated_shells", "manifold.review_command_session_contract", "hostess.collect_install_launch_evidence")) {
            if (-not (@($HostessStagingHandoffView.provenance.source_action_ids) -contains $ActionId)) {
                throw "shell Hostess staging handoff provenance missing action $ActionId"
            }
        }
        foreach ($RouteKind in @("hostess.stage.generated_shells", "manifold.review.command_session_contract", "hostess.collect.install_launch_evidence")) {
            if (-not (@($HostessStagingHandoffView.provenance.source_route_kinds) -contains $RouteKind)) {
                throw "shell Hostess staging handoff provenance missing route $RouteKind"
            }
        }
        if (-not (@($HostessStagingHandoffView.provenance.target_keys) -contains "shared")) {
            throw "shell Hostess staging handoff provenance missing shared target"
        }
        foreach ($Target in @("desktop", "phone", "quest")) {
            if (@($HostessStagingHandoffView.provenance.target_keys | Where-Object { $_ -like "$Target/*" }).Count -ne 1) {
                throw "shell Hostess staging handoff provenance missing target $Target"
            }
        }
        $SharedSummary = @($HostessStagingHandoffView.request_summaries | Where-Object { $_.target_key -eq "shared" })[0]
        if ($null -eq $SharedSummary -or $SharedSummary.status -ne "ready" -or $SharedSummary.planned_file_count -ne 5) {
            throw "shell Hostess staging handoff shared summary mismatch"
        }
        foreach ($Target in @("desktop", "phone", "quest")) {
            $TargetSummary = @($HostessStagingHandoffView.request_summaries | Where-Object { $_.target_key -like "$Target/*" })[0]
            if ($null -eq $TargetSummary -or $TargetSummary.status -ne "ready" -or $TargetSummary.planned_file_count -ne 4) {
                throw "shell Hostess staging handoff target summary mismatch for $Target"
            }
        }
        foreach ($Instruction in @(
            @{ Id = "hostess.copy_staging_files"; Owner = "rusty.hostess"; Route = "hostess.stage.files_from_plan" },
            @{ Id = "manifold.review_command_session_contract"; Owner = "rusty.manifold"; Route = "manifold.review.command_session_contract" },
            @{ Id = "hostess.collect_install_launch_evidence"; Owner = "rusty.hostess"; Route = "hostess.collect.install_launch_evidence" }
        )) {
            if (@($HostessStagingHandoffView.owner_instructions | Where-Object { $_.instruction_id -eq $Instruction.Id -and $_.owner -eq $Instruction.Owner -and $_.route_kind -eq $Instruction.Route -and $_.status -eq "ready" -and $_.prohibited_in_studio -eq $true }).Count -ne 1) {
                throw "shell Hostess staging handoff missing ready instruction $($Instruction.Id)"
            }
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingHandoffView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging handoff missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessStagingHandoffView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging handoff should not contain failed checks"
        }
    }
    $ShellHostessStagingAcceptanceOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-checklist --handoff $ShellHostessStagingHandoffPath --output $ShellHostessStagingAcceptancePath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance checklist failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptancePath)) {
        throw "shell Hostess staging acceptance checklist was not written"
    }
    $ShellHostessStagingAcceptance = ($ShellHostessStagingAcceptanceOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingAcceptance = Get-Content -Raw $ShellHostessStagingAcceptancePath | ConvertFrom-Json
    foreach ($HostessStagingAcceptanceView in @($ShellHostessStagingAcceptance, $WrittenShellHostessStagingAcceptance)) {
        if ($HostessStagingAcceptanceView.'$schema' -ne "rusty.studio.shell_hostess_staging_acceptance_checklist.v1") {
            throw "shell Hostess staging acceptance checklist schema mismatch"
        }
        if ($HostessStagingAcceptanceView.source_handoff_schema -ne "rusty.studio.shell_hostess_staging_handoff_envelope.v1" -or $HostessStagingAcceptanceView.handoff_path -ne $ShellHostessStagingHandoffPath) {
            throw "shell Hostess staging acceptance checklist source handoff mismatch"
        }
        if ($HostessStagingAcceptanceView.file_plan_path -ne $ShellHostessStagingFilePlanPath -or $HostessStagingAcceptanceView.preview_path -ne $ShellHostessStagingPreviewPath -or $HostessStagingAcceptanceView.intake_path -ne $ShellHostessOwnerIntakePath -or $HostessStagingAcceptanceView.package_path -ne $ShellHostessHandoffPackagePath -or $HostessStagingAcceptanceView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess staging acceptance checklist source path mismatch"
        }
        if ($HostessStagingAcceptanceView.status -ne "ready" -or $null -ne $HostessStagingAcceptanceView.issue_code) {
            throw "shell Hostess staging acceptance checklist should be ready"
        }
        if ($HostessStagingAcceptanceView.selected_candidate_id -ne "synthetic-ready-candidate" -or $HostessStagingAcceptanceView.envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1") {
            throw "shell Hostess staging acceptance selected candidate or envelope id mismatch"
        }
        if ($HostessStagingAcceptanceView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingAcceptanceView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingAcceptanceView.project_revision -ne 1) {
            throw "shell Hostess staging acceptance project identity mismatch"
        }
        if ($HostessStagingAcceptanceView.execution_policy -ne "not_executed.acceptance_check_only" -or $HostessStagingAcceptanceView.checklist_owner -ne "rusty.hostess" -or $HostessStagingAcceptanceView.handoff_owner -ne "rusty.hostess" -or $HostessStagingAcceptanceView.staging_owner -ne "rusty.hostess") {
            throw "shell Hostess staging acceptance policy owner mismatch"
        }
        if ($HostessStagingAcceptanceView.command_session_authority -ne "rusty.manifold" -or $HostessStagingAcceptanceView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingAcceptanceView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging acceptance runtime authority mismatch"
        }
        if ($HostessStagingAcceptanceView.request_count -ne 4 -or $HostessStagingAcceptanceView.ready_request_count -ne 4 -or $HostessStagingAcceptanceView.blocked_request_count -ne 0 -or $HostessStagingAcceptanceView.instruction_count -ne 4 -or $HostessStagingAcceptanceView.ready_instruction_count -ne 4 -or $HostessStagingAcceptanceView.blocked_instruction_count -ne 0) {
            throw "shell Hostess staging acceptance source count mismatch"
        }
        if ($HostessStagingAcceptanceView.ready_item_count -ne 6 -or $HostessStagingAcceptanceView.blocked_item_count -ne 0 -or $HostessStagingAcceptanceView.rejected_item_count -ne 0) {
            throw "shell Hostess staging acceptance item counts mismatch"
        }
        if ($HostessStagingAcceptanceView.checksum_algorithm -ne "fnv1a64.studio_staging_file_plan.v1" -or $HostessStagingAcceptanceView.plan_checksum.Length -ne 16) {
            throw "shell Hostess staging acceptance checksum mismatch"
        }
        foreach ($Entry in @(
            @{ Id = "hostess.accept_staging_handoff"; Owner = "rusty.hostess"; Route = "hostess.accept.staging_handoff" },
            @{ Id = "hostess.verify_staging_file_plan_checksum"; Owner = "rusty.hostess"; Route = "hostess.verify.staging_file_plan_checksum" },
            @{ Id = "hostess.review_staging_file_requests"; Owner = "rusty.hostess"; Route = "hostess.review.staging_file_requests" },
            @{ Id = "hostess.copy_staging_files"; Owner = "rusty.hostess"; Route = "hostess.stage.files_from_plan" },
            @{ Id = "manifold.review_command_session_contract"; Owner = "rusty.manifold"; Route = "manifold.review.command_session_contract" },
            @{ Id = "hostess.collect_install_launch_evidence"; Owner = "rusty.hostess"; Route = "hostess.collect.install_launch_evidence" }
        )) {
            if (@($HostessStagingAcceptanceView.entries | Where-Object { $_.item_id -eq $Entry.Id -and $_.owner -eq $Entry.Owner -and $_.route_kind -eq $Entry.Route -and $_.status -eq "ready" -and $_.prohibited_in_studio -eq $true }).Count -ne 1) {
                throw "shell Hostess staging acceptance missing ready entry $($Entry.Id)"
            }
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingAcceptanceView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging acceptance missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessStagingAcceptanceView.handoff_checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging acceptance should not contain failed handoff checks"
        }
    }
    $ShellHostessStagingAcceptanceManifestOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-manifest --checklist $ShellHostessStagingAcceptancePath --acceptance-id "synthetic-hostess-staging-ready" --label "Synthetic Hostess staging ready acceptance" --output $ShellHostessStagingAcceptanceManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance manifest failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceManifestPath)) {
        throw "shell Hostess staging acceptance manifest was not written"
    }
    $ShellHostessStagingAcceptanceManifest = ($ShellHostessStagingAcceptanceManifestOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingAcceptanceManifest = Get-Content -Raw $ShellHostessStagingAcceptanceManifestPath | ConvertFrom-Json
    foreach ($HostessStagingAcceptanceManifestView in @($ShellHostessStagingAcceptanceManifest, $WrittenShellHostessStagingAcceptanceManifest)) {
        if ($HostessStagingAcceptanceManifestView.'$schema' -ne "rusty.studio.shell_hostess_staging_acceptance_manifest.v1") {
            throw "shell Hostess staging acceptance manifest schema mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.acceptance_id -ne "synthetic-hostess-staging-ready" -or $HostessStagingAcceptanceManifestView.label -ne "Synthetic Hostess staging ready acceptance") {
            throw "shell Hostess staging acceptance manifest identity mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.checklist_path -ne $ShellHostessStagingAcceptancePath -or $HostessStagingAcceptanceManifestView.checklist_schema -ne "rusty.studio.shell_hostess_staging_acceptance_checklist.v1") {
            throw "shell Hostess staging acceptance manifest checklist mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1" -or $HostessStagingAcceptanceManifestView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingAcceptanceManifestView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingAcceptanceManifestView.project_revision -ne 1) {
            throw "shell Hostess staging acceptance manifest source identity mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.status -ne "ready" -or $null -ne $HostessStagingAcceptanceManifestView.issue_code) {
            throw "shell Hostess staging acceptance manifest should be ready"
        }
        if ($HostessStagingAcceptanceManifestView.execution_policy -ne "not_executed.acceptance_check_only" -or $HostessStagingAcceptanceManifestView.checklist_owner -ne "rusty.hostess" -or $HostessStagingAcceptanceManifestView.handoff_owner -ne "rusty.hostess" -or $HostessStagingAcceptanceManifestView.staging_owner -ne "rusty.hostess") {
            throw "shell Hostess staging acceptance manifest owner mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.command_session_authority -ne "rusty.manifold" -or $HostessStagingAcceptanceManifestView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingAcceptanceManifestView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging acceptance manifest authority mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.ready_item_count -ne 6 -or $HostessStagingAcceptanceManifestView.blocked_item_count -ne 0 -or $HostessStagingAcceptanceManifestView.rejected_item_count -ne 0 -or $HostessStagingAcceptanceManifestView.request_count -ne 4 -or $HostessStagingAcceptanceManifestView.instruction_count -ne 4) {
            throw "shell Hostess staging acceptance manifest counts mismatch"
        }
        if ($HostessStagingAcceptanceManifestView.checksum_algorithm -ne "fnv1a64.studio_staging_file_plan.v1" -or $HostessStagingAcceptanceManifestView.plan_checksum.Length -ne 16) {
            throw "shell Hostess staging acceptance manifest checksum mismatch"
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingAcceptanceManifestView.prohibited_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging acceptance manifest missing prohibited action $ProhibitedAction"
            }
        }
    }
    $ShellHostessStagingAcceptanceIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-index --acceptance-manifest $ShellHostessStagingAcceptanceManifestPath --default-acceptance-id "synthetic-hostess-staging-ready" --output $ShellHostessStagingAcceptanceIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance index failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceIndexPath)) {
        throw "shell Hostess staging acceptance index was not written"
    }
    $ShellHostessStagingAcceptanceIndex = ($ShellHostessStagingAcceptanceIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingAcceptanceIndex = Get-Content -Raw $ShellHostessStagingAcceptanceIndexPath | ConvertFrom-Json
    foreach ($HostessStagingAcceptanceIndexView in @($ShellHostessStagingAcceptanceIndex, $WrittenShellHostessStagingAcceptanceIndex)) {
        if ($HostessStagingAcceptanceIndexView.'$schema' -ne "rusty.studio.shell_hostess_staging_acceptance_index.v1") {
            throw "shell Hostess staging acceptance index schema mismatch"
        }
        if ($HostessStagingAcceptanceIndexView.default_acceptance_id -ne "synthetic-hostess-staging-ready" -or $HostessStagingAcceptanceIndexView.acceptance_count -ne 1 -or $HostessStagingAcceptanceIndexView.ready_acceptance_count -ne 1 -or $HostessStagingAcceptanceIndexView.blocked_acceptance_count -ne 0 -or $HostessStagingAcceptanceIndexView.rejected_acceptance_count -ne 0) {
            throw "shell Hostess staging acceptance index counts mismatch"
        }
        if (-not (@($HostessStagingAcceptanceIndexView.project_ids) -contains "studio.project.synthetic_wave") -or -not (@($HostessStagingAcceptanceIndexView.envelope_ids) -contains "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1") -or -not (@($HostessStagingAcceptanceIndexView.manifest_ids) -contains "studio.shell_handoffs.studio.project.synthetic_wave")) {
            throw "shell Hostess staging acceptance index identity sets mismatch"
        }
        $ReadyAcceptanceEntry = @($HostessStagingAcceptanceIndexView.entries | Where-Object { $_.acceptance_id -eq "synthetic-hostess-staging-ready" })[0]
        if ($null -eq $ReadyAcceptanceEntry -or $ReadyAcceptanceEntry.acceptance_manifest_path -ne $ShellHostessStagingAcceptanceManifestPath -or $ReadyAcceptanceEntry.checklist_path -ne $ShellHostessStagingAcceptancePath -or $ReadyAcceptanceEntry.status -ne "ready" -or $ReadyAcceptanceEntry.ready_item_count -ne 6) {
            throw "shell Hostess staging acceptance index ready entry mismatch"
        }
        if ($ReadyAcceptanceEntry.execution_policy -ne "not_executed.acceptance_check_only" -or $ReadyAcceptanceEntry.command_session_authority -ne "rusty.manifold" -or $ReadyAcceptanceEntry.install_launch_evidence_authority -ne "rusty.hostess" -or $ReadyAcceptanceEntry.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging acceptance index authority mismatch"
        }
    }
    $ShellHostessStagingAcceptanceSelectionOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-selection --acceptance-index $ShellHostessStagingAcceptanceIndexPath --acceptance-id "synthetic-hostess-staging-ready" --output $ShellHostessStagingAcceptanceSelectionPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance selection failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceSelectionPath)) {
        throw "shell Hostess staging acceptance selection was not written"
    }
    $ShellHostessStagingAcceptanceSelection = ($ShellHostessStagingAcceptanceSelectionOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($ShellHostessStagingAcceptanceSelection.'$schema' -ne "rusty.studio.shell_hostess_staging_acceptance_selection.v1" -or $ShellHostessStagingAcceptanceSelection.source_index_schema -ne "rusty.studio.shell_hostess_staging_acceptance_index.v1") {
        throw "shell Hostess staging acceptance selection schema mismatch"
    }
    if ($ShellHostessStagingAcceptanceSelection.status -ne "selected" -or $ShellHostessStagingAcceptanceSelection.requested_acceptance_id -ne "synthetic-hostess-staging-ready" -or $ShellHostessStagingAcceptanceSelection.selected_acceptance_id -ne "synthetic-hostess-staging-ready" -or $ShellHostessStagingAcceptanceSelection.default_acceptance_id -ne "synthetic-hostess-staging-ready") {
        throw "shell Hostess staging acceptance selection identity mismatch"
    }
    if ($ShellHostessStagingAcceptanceSelection.acceptance_count -ne 1 -or $ShellHostessStagingAcceptanceSelection.ready_acceptance_count -ne 1 -or @($ShellHostessStagingAcceptanceSelection.entries | Where-Object { $_.acceptance_id -eq "synthetic-hostess-staging-ready" -and $_.selected -eq $true -and $_.default -eq $true }).Count -ne 1) {
        throw "shell Hostess staging acceptance selection counts mismatch"
    }
    $ShellHostessStagingAcceptanceComparisonOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-comparison --acceptance-index $ShellHostessStagingAcceptanceIndexPath --acceptance-id "synthetic-hostess-staging-ready" --candidate $ShellHostessStagingAcceptancePath --output $ShellHostessStagingAcceptanceComparisonPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance comparison failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceComparisonPath)) {
        throw "shell Hostess staging acceptance comparison was not written"
    }
    $ShellHostessStagingAcceptanceComparison = ($ShellHostessStagingAcceptanceComparisonOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingAcceptanceComparison = Get-Content -Raw $ShellHostessStagingAcceptanceComparisonPath | ConvertFrom-Json
    foreach ($HostessStagingAcceptanceComparisonView in @($ShellHostessStagingAcceptanceComparison, $WrittenShellHostessStagingAcceptanceComparison)) {
        if ($HostessStagingAcceptanceComparisonView.'$schema' -ne "rusty.studio.shell_hostess_staging_acceptance_comparison.v1") {
            throw "shell Hostess staging acceptance comparison schema mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_identity_schema -ne "rusty.studio.shell_hostess_staging_acceptance_manifest.v1" -or $HostessStagingAcceptanceComparisonView.baseline_acceptance_id -ne "synthetic-hostess-staging-ready") {
            throw "shell Hostess staging acceptance comparison baseline identity mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_index_schema -ne "rusty.studio.shell_hostess_staging_acceptance_index.v1" -or $HostessStagingAcceptanceComparisonView.baseline_index_selected_acceptance_id -ne "synthetic-hostess-staging-ready" -or $HostessStagingAcceptanceComparisonView.baseline_index_default_acceptance_id -ne "synthetic-hostess-staging-ready") {
            throw "shell Hostess staging acceptance comparison index identity mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_schema -ne "rusty.studio.shell_hostess_staging_acceptance_checklist.v1" -or $HostessStagingAcceptanceComparisonView.candidate_schema -ne "rusty.studio.shell_hostess_staging_acceptance_checklist.v1") {
            throw "shell Hostess staging acceptance comparison checklist schema mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1" -or $HostessStagingAcceptanceComparisonView.candidate_envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1") {
            throw "shell Hostess staging acceptance comparison envelope mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_project_id -ne "studio.project.synthetic_wave" -or $HostessStagingAcceptanceComparisonView.candidate_project_id -ne "studio.project.synthetic_wave" -or $HostessStagingAcceptanceComparisonView.baseline_project_revision -ne 1 -or $HostessStagingAcceptanceComparisonView.candidate_project_revision -ne 1) {
            throw "shell Hostess staging acceptance comparison project mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.status -ne "unchanged" -or $null -ne $HostessStagingAcceptanceComparisonView.issue_code) {
            throw "shell Hostess staging acceptance comparison status mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.ready_item_delta -ne 0 -or $HostessStagingAcceptanceComparisonView.blocked_item_delta -ne 0 -or $HostessStagingAcceptanceComparisonView.rejected_item_delta -ne 0) {
            throw "shell Hostess staging acceptance comparison deltas mismatch"
        }
        if ($HostessStagingAcceptanceComparisonView.baseline_ready_item_count -ne 6 -or $HostessStagingAcceptanceComparisonView.candidate_ready_item_count -ne 6 -or $HostessStagingAcceptanceComparisonView.baseline_blocked_item_count -ne 0 -or $HostessStagingAcceptanceComparisonView.candidate_blocked_item_count -ne 0) {
            throw "shell Hostess staging acceptance comparison counts mismatch"
        }
        if (@($HostessStagingAcceptanceComparisonView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging acceptance comparison has failed checks"
        }
        if (@($HostessStagingAcceptanceComparisonView.entries).Count -ne 6 -or @($HostessStagingAcceptanceComparisonView.entries | Where-Object { $_.change -eq "unchanged" }).Count -ne 6) {
            throw "shell Hostess staging acceptance comparison entries mismatch"
        }
        if (@($HostessStagingAcceptanceComparisonView.entries | Where-Object { $_.item_id -eq "hostess.copy_staging_files" -and $_.owner -eq "rusty.hostess" -and $_.candidate_route_kind -eq "hostess.stage.files_from_plan" -and $_.change -eq "unchanged" }).Count -ne 1) {
            throw "shell Hostess staging acceptance comparison Hostess action row mismatch"
        }
        if (@($HostessStagingAcceptanceComparisonView.entries | Where-Object { $_.item_id -eq "manifold.review_command_session_contract" -and $_.owner -eq "rusty.manifold" -and $_.candidate_route_kind -eq "manifold.review.command_session_contract" -and $_.change -eq "unchanged" }).Count -ne 1) {
            throw "shell Hostess staging acceptance comparison Manifold row mismatch"
        }
    }
    $RouteDriftAcceptance = Get-Content -Raw $ShellHostessStagingAcceptancePath | ConvertFrom-Json
    foreach ($Entry in $RouteDriftAcceptance.entries) {
        if ($Entry.item_id -eq "hostess.accept_staging_handoff") {
            $Entry.owner = "rusty.studio"
        }
        if ($Entry.item_id -eq "hostess.copy_staging_files") {
            $Entry.route_kind = "hostess.stage.files_from_drifted_plan"
        }
        if ($Entry.item_id -eq "hostess.review_staging_file_requests") {
            $Entry.prohibited_in_studio = $false
            $Entry.expected_input_path = "target\drifted-input.json"
        }
    }
    $RouteDriftAcceptance | ConvertTo-Json -Depth 20 | Set-Content -Path $ShellHostessStagingAcceptanceRouteDriftPath
    $ShellHostessStagingAcceptanceRouteDriftComparisonOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-comparison --acceptance-index $ShellHostessStagingAcceptanceIndexPath --acceptance-id "synthetic-hostess-staging-ready" --candidate $ShellHostessStagingAcceptanceRouteDriftPath --output $ShellHostessStagingAcceptanceRouteDriftComparisonPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance route-drift comparison failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceRouteDriftComparisonPath)) {
        throw "shell Hostess staging acceptance route-drift comparison was not written"
    }
    $ShellHostessStagingAcceptanceRouteDriftComparison = ($ShellHostessStagingAcceptanceRouteDriftComparisonOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingAcceptanceRouteDriftComparison = Get-Content -Raw $ShellHostessStagingAcceptanceRouteDriftComparisonPath | ConvertFrom-Json
    foreach ($RouteDriftComparisonView in @($ShellHostessStagingAcceptanceRouteDriftComparison, $WrittenShellHostessStagingAcceptanceRouteDriftComparison)) {
        if ($RouteDriftComparisonView.status -ne "incomparable" -or $RouteDriftComparisonView.issue_code -ne "studio.issue.shell_hostess_staging_acceptance_entry_drift") {
            throw "shell Hostess staging acceptance route drift did not become incomparable"
        }
        if ($RouteDriftComparisonView.ready_item_delta -ne 0 -or $RouteDriftComparisonView.blocked_item_delta -ne 0 -or $RouteDriftComparisonView.rejected_item_delta -ne 0) {
            throw "shell Hostess staging acceptance route drift deltas changed unexpectedly"
        }
        $RouteDriftChangedEntries = @($RouteDriftComparisonView.entries | Where-Object { $_.change -eq "changed" })
        if ($RouteDriftChangedEntries.Count -gt 0 -and $RouteDriftComparisonView.status -eq "unchanged") {
            throw "shell Hostess staging acceptance route drift changed entries were reported as unchanged"
        }
        if (@($RouteDriftChangedEntries | Where-Object { $_.issue_code -ne "studio.issue.shell_hostess_staging_acceptance_entry_drift" }).Count -ne 0) {
            throw "shell Hostess staging acceptance route drift changed entries are missing drift issue codes"
        }
        if (@($RouteDriftComparisonView.entries | Where-Object { $_.change -eq "changed" -and $_.issue_code -eq "studio.issue.shell_hostess_staging_acceptance_entry_drift" }).Count -ne 3) {
            throw "shell Hostess staging acceptance route drift changed-entry count mismatch"
        }
        if (@($RouteDriftComparisonView.checks | Where-Object { $_.check_id -eq "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts" -and $_.status -eq "fail" -and $_.issue_code -eq "studio.issue.shell_hostess_staging_acceptance_entry_drift" }).Count -ne 1) {
            throw "shell Hostess staging acceptance route drift failed check missing"
        }
        if (@($RouteDriftComparisonView.entries | Where-Object { $_.item_id -eq "hostess.copy_staging_files" -and $_.candidate_route_kind -eq "hostess.stage.files_from_drifted_plan" -and $_.change -eq "changed" }).Count -ne 1) {
            throw "shell Hostess staging acceptance route drift action row mismatch"
        }
    }
    $ProjectedMotionShellHandoffReviewOutput = & cargo run --quiet -p rusty-studio-cli -- projected-motion-breath-shell-handoff-review --evidence $ProjectedMotionShellHandoffEvidencePath --output $ProjectedMotionShellHandoffReviewPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio PMB shell handoff review failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ProjectedMotionShellHandoffReviewPath)) {
        throw "PMB shell handoff review was not written"
    }
    $ProjectedMotionShellHandoffReview = ($ProjectedMotionShellHandoffReviewOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenProjectedMotionShellHandoffReview = Get-Content -Raw $ProjectedMotionShellHandoffReviewPath | ConvertFrom-Json
    foreach ($PmbShellHandoffReviewView in @($ProjectedMotionShellHandoffReview, $WrittenProjectedMotionShellHandoffReview)) {
        if ($PmbShellHandoffReviewView.'$schema' -ne "rusty.studio.projected_motion_breath_shell_handoff_review.v1") {
            throw "PMB shell handoff review schema mismatch"
        }
        if ($PmbShellHandoffReviewView.source_evidence_schema -ne "rusty.hostess.projected_motion_breath.shell_handoff_validation_evidence.v1" -or $PmbShellHandoffReviewView.source_evidence_path -ne $ProjectedMotionShellHandoffEvidencePath) {
            throw "PMB shell handoff review source evidence mismatch"
        }
        if ($PmbShellHandoffReviewView.status -ne "ready" -or $null -ne $PmbShellHandoffReviewView.issue_code) {
            throw "PMB shell handoff review should be ready"
        }
        if ($PmbShellHandoffReviewView.target_package_id -ne "package.projected_motion_breath" -or $PmbShellHandoffReviewView.handoff_id -ne "shell_handoff.projected_motion_breath.loopback" -or $PmbShellHandoffReviewView.shell_app_id -ne "app.downstream_shell") {
            throw "PMB shell handoff review identity mismatch"
        }
        if ($PmbShellHandoffReviewView.execution_policy -ne "not_executed.review_only" -or $PmbShellHandoffReviewView.runtime_authority -ne "rusty.manifold" -or $PmbShellHandoffReviewView.authoring_authority -ne "rusty.studio" -or $PmbShellHandoffReviewView.platform_validation_authority -ne "rusty.hostess") {
            throw "PMB shell handoff review authority mismatch"
        }
        if ($PmbShellHandoffReviewView.runtime_execution_performed -ne $false -or $PmbShellHandoffReviewView.platform_execution_performed -ne $false -or $PmbShellHandoffReviewView.broker_transport_used -ne $false -or $PmbShellHandoffReviewView.downstream_shell_runtime_used -ne $false -or $PmbShellHandoffReviewView.legacy_app_dependency_used -ne $false) {
            throw "PMB shell handoff review should remain review-only"
        }
        if ($PmbShellHandoffReviewView.required_binding_count -ne 3 -or $PmbShellHandoffReviewView.ready_required_binding_count -ne 3 -or $PmbShellHandoffReviewView.feedback_receipt_exported -ne $true -or $PmbShellHandoffReviewView.feedback_sink_provides_receipt -ne $true) {
            throw "PMB shell handoff review binding counts mismatch"
        }
        foreach ($Binding in @("stream.motion.object_pose:publish", "stream.breath.feedback_state:subscribe", "stream.breath.feedback_receipt:publish")) {
            if (-not (@($PmbShellHandoffReviewView.stream_bindings) -contains $Binding)) {
                throw "PMB shell handoff review missing binding $Binding"
            }
        }
        if (-not (@($PmbShellHandoffReviewView.command_ids) -contains "command.breath.status") -or -not (@($PmbShellHandoffReviewView.transport_ids) -contains "transport.shell_loopback")) {
            throw "PMB shell handoff review command or transport mismatch"
        }
        if (@($PmbShellHandoffReviewView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "PMB shell handoff review should not contain failed checks"
        }
    }
    $ShellHostessStagingExecutionRequestOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-execution-request --acceptance-index $ShellHostessStagingAcceptanceIndexPath --acceptance-id "synthetic-hostess-staging-ready" --output $ShellHostessStagingExecutionRequestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging execution request failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingExecutionRequestPath)) {
        throw "shell Hostess staging execution request was not written"
    }
    $ShellHostessStagingExecutionRequest = ($ShellHostessStagingExecutionRequestOutput -join [Environment]::NewLine) | ConvertFrom-Json
    $WrittenShellHostessStagingExecutionRequest = Get-Content -Raw $ShellHostessStagingExecutionRequestPath | ConvertFrom-Json
    foreach ($HostessStagingExecutionRequestView in @($ShellHostessStagingExecutionRequest, $WrittenShellHostessStagingExecutionRequest)) {
        if ($HostessStagingExecutionRequestView.'$schema' -ne "rusty.studio.shell_hostess_staging_execution_request.v1") {
            throw "shell Hostess staging execution request schema mismatch"
        }
        if ($HostessStagingExecutionRequestView.source_acceptance_index_schema -ne "rusty.studio.shell_hostess_staging_acceptance_index.v1" -or $HostessStagingExecutionRequestView.acceptance_index_path -ne $ShellHostessStagingAcceptanceIndexPath) {
            throw "shell Hostess staging execution request index source mismatch"
        }
        if ($HostessStagingExecutionRequestView.selected_acceptance_id -ne "synthetic-hostess-staging-ready" -or $HostessStagingExecutionRequestView.acceptance_manifest_path -ne $ShellHostessStagingAcceptanceManifestPath -or $HostessStagingExecutionRequestView.acceptance_checklist_path -ne $ShellHostessStagingAcceptancePath) {
            throw "shell Hostess staging execution request acceptance source mismatch"
        }
        if ($HostessStagingExecutionRequestView.acceptance_schema -ne "rusty.studio.shell_hostess_staging_acceptance_manifest.v1" -or $HostessStagingExecutionRequestView.acceptance_checklist_schema -ne "rusty.studio.shell_hostess_staging_acceptance_checklist.v1" -or $HostessStagingExecutionRequestView.source_handoff_schema -ne "rusty.studio.shell_hostess_staging_handoff_envelope.v1") {
            throw "shell Hostess staging execution request source schema mismatch"
        }
        if ($HostessStagingExecutionRequestView.handoff_path -ne $ShellHostessStagingHandoffPath -or $HostessStagingExecutionRequestView.file_plan_path -ne $ShellHostessStagingFilePlanPath -or $HostessStagingExecutionRequestView.preview_path -ne $ShellHostessStagingPreviewPath -or $HostessStagingExecutionRequestView.intake_path -ne $ShellHostessOwnerIntakePath -or $HostessStagingExecutionRequestView.package_path -ne $ShellHostessHandoffPackagePath -or $HostessStagingExecutionRequestView.handoff_manifest_path -ne $ShellHandoffManifestPath) {
            throw "shell Hostess staging execution request source path mismatch"
        }
        if ($HostessStagingExecutionRequestView.status -ne "ready" -or $null -ne $HostessStagingExecutionRequestView.issue_code) {
            throw "shell Hostess staging execution request should be ready"
        }
        if ($HostessStagingExecutionRequestView.execution_policy -ne "not_executed.hostess_request_only" -or $HostessStagingExecutionRequestView.adapter_owner -ne "rusty.hostess" -or $HostessStagingExecutionRequestView.requester_role -ne "rusty.studio") {
            throw "shell Hostess staging execution request policy owner mismatch"
        }
        if ($HostessStagingExecutionRequestView.command_session_authority -ne "rusty.manifold" -or $HostessStagingExecutionRequestView.install_launch_evidence_authority -ne "rusty.hostess" -or $HostessStagingExecutionRequestView.studio_role -ne "authoring.export_planning") {
            throw "shell Hostess staging execution request authority mismatch"
        }
        if ($HostessStagingExecutionRequestView.envelope_id -ne "studio.hostess_staging_handoff.studio.project.synthetic_wave.rev1" -or $HostessStagingExecutionRequestView.manifest_id -ne "studio.shell_handoffs.studio.project.synthetic_wave" -or $HostessStagingExecutionRequestView.project_id -ne "studio.project.synthetic_wave" -or $HostessStagingExecutionRequestView.project_revision -ne 1 -or $HostessStagingExecutionRequestView.selected_candidate_id -ne "synthetic-ready-candidate") {
            throw "shell Hostess staging execution request identity mismatch"
        }
        if ($HostessStagingExecutionRequestView.request_count -ne 4 -or $HostessStagingExecutionRequestView.ready_request_count -ne 4 -or $HostessStagingExecutionRequestView.blocked_request_count -ne 0 -or $HostessStagingExecutionRequestView.instruction_count -ne 4 -or $HostessStagingExecutionRequestView.ready_instruction_count -ne 4 -or $HostessStagingExecutionRequestView.blocked_instruction_count -ne 0) {
            throw "shell Hostess staging execution request source counts mismatch"
        }
        if ($HostessStagingExecutionRequestView.checksum_algorithm -ne "fnv1a64.studio_staging_file_plan.v1" -or $HostessStagingExecutionRequestView.plan_checksum.Length -ne 16) {
            throw "shell Hostess staging execution request checksum mismatch"
        }
        if ($HostessStagingExecutionRequestView.adapter_action_count -ne 6 -or $HostessStagingExecutionRequestView.ready_adapter_action_count -ne 6 -or $HostessStagingExecutionRequestView.blocked_adapter_action_count -ne 0) {
            throw "shell Hostess staging execution request action counts mismatch"
        }
        if ($HostessStagingExecutionRequestView.pmb_shell_handoff_review_required -ne $true -or $HostessStagingExecutionRequestView.pmb_shell_handoff_review_ready -ne $true) {
            throw "shell Hostess staging execution request PMB gate should be required and ready"
        }
        if ([string]::IsNullOrWhiteSpace($HostessStagingExecutionRequestView.pmb_shell_handoff_review_path) -or -not $HostessStagingExecutionRequestView.pmb_shell_handoff_review_path.EndsWith("pmb-shell-handoff.studio-review.json")) {
            throw "shell Hostess staging execution request PMB review path mismatch"
        }
        if ($HostessStagingExecutionRequestView.source_pmb_shell_handoff_review_schema -ne "rusty.studio.projected_motion_breath_shell_handoff_review.v1" -or $HostessStagingExecutionRequestView.source_pmb_shell_handoff_review_status -ne "ready" -or $null -ne $HostessStagingExecutionRequestView.source_pmb_shell_handoff_review_issue_code) {
            throw "shell Hostess staging execution request PMB review source mismatch"
        }
        if ($HostessStagingExecutionRequestView.source_pmb_shell_handoff_id -ne "shell_handoff.projected_motion_breath.loopback" -or $HostessStagingExecutionRequestView.source_pmb_shell_app_id -ne "app.downstream_shell") {
            throw "shell Hostess staging execution request PMB handoff identity mismatch"
        }
        $PmbReviewCliArgs = @($HostessStagingExecutionRequestView.hostess_operator_start_preflight_cli_args)
        if ($PmbReviewCliArgs.Count -ne 3 -or $PmbReviewCliArgs[0] -ne "--pmb-shell-handoff-review-in" -or $PmbReviewCliArgs[1] -ne $HostessStagingExecutionRequestView.pmb_shell_handoff_review_path -or $PmbReviewCliArgs[2] -ne "--require-pmb-shell-handoff-review") {
            throw "shell Hostess staging execution request PMB Hostess CLI args mismatch"
        }
        if (@($HostessStagingExecutionRequestView.checks | Where-Object { $_.check_id -eq "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review" -and $_.status -eq "pass" }).Count -ne 1) {
            throw "shell Hostess staging execution request missing passing PMB gate check"
        }
        foreach ($Action in @(
            @{ Id = "adapter.hostess.accept_staging_handoff"; Source = "hostess.accept_staging_handoff"; Owner = "rusty.hostess"; Route = "hostess.accept.staging_handoff" },
            @{ Id = "adapter.hostess.copy_staging_files"; Source = "hostess.copy_staging_files"; Owner = "rusty.hostess"; Route = "hostess.stage.files_from_plan" },
            @{ Id = "adapter.manifold.review_command_session_contract"; Source = "manifold.review_command_session_contract"; Owner = "rusty.manifold"; Route = "manifold.review.command_session_contract" },
            @{ Id = "adapter.hostess.collect_install_launch_evidence"; Source = "hostess.collect_install_launch_evidence"; Owner = "rusty.hostess"; Route = "hostess.collect.install_launch_evidence" }
        )) {
            if (@($HostessStagingExecutionRequestView.actions | Where-Object { $_.action_id -eq $Action.Id -and $_.source_item_id -eq $Action.Source -and $_.owner -eq $Action.Owner -and $_.route_kind -eq $Action.Route -and $_.status -eq "ready" -and $_.ack_required -eq $true -and $_.execution_in_studio -eq $false }).Count -ne 1) {
                throw "shell Hostess staging execution request missing action $($Action.Id)"
            }
        }
        if (@($HostessStagingExecutionRequestView.actions | Where-Object { $_.execution_in_studio -ne $false -or $_.ack_required -ne $true }).Count -ne 0) {
            throw "shell Hostess staging execution request action boundary mismatch"
        }
        foreach ($ProhibitedAction in @("stage_generated_shells", "install", "launch", "open_command_session", "collect_device_evidence", "collect_install_launch_evidence")) {
            if (-not (@($HostessStagingExecutionRequestView.prohibited_studio_actions) -contains $ProhibitedAction)) {
                throw "shell Hostess staging execution request missing prohibited action $ProhibitedAction"
            }
        }
        if (@($HostessStagingExecutionRequestView.checks | Where-Object { $_.status -eq "fail" }).Count -ne 0) {
            throw "shell Hostess staging execution request should not contain failed checks"
        }
        if ($HostessStagingExecutionRequestView.ack_template.'$schema' -ne "rusty.studio.shell_hostess_staging_execution_ack.v1" -or $HostessStagingExecutionRequestView.ack_template.request_id -ne $HostessStagingExecutionRequestView.request_id -or $HostessStagingExecutionRequestView.ack_template.accepted_by -ne "rusty.hostess" -or $HostessStagingExecutionRequestView.ack_template.ack_status -ne "pending" -or $HostessStagingExecutionRequestView.ack_template.execution_in_studio -ne $false) {
            throw "shell Hostess staging execution request ack template mismatch"
        }
        if (@($HostessStagingExecutionRequestView.ack_template.required_action_ids).Count -ne 6 -or @($HostessStagingExecutionRequestView.ack_template.accepted_action_ids).Count -ne 0 -or -not (@($HostessStagingExecutionRequestView.ack_template.required_evidence_kinds) -contains "hostess_install_launch_evidence_receipt") -or -not (@($HostessStagingExecutionRequestView.ack_template.required_evidence_kinds) -contains "manifold_command_session_contract_review")) {
            throw "shell Hostess staging execution request ack template evidence mismatch"
        }
        if ($HostessStagingExecutionRequestView.reject_template.'$schema' -ne "rusty.studio.shell_hostess_staging_execution_reject.v1" -or $HostessStagingExecutionRequestView.reject_template.request_id -ne $HostessStagingExecutionRequestView.request_id -or $HostessStagingExecutionRequestView.reject_template.rejected_by -ne "rusty.hostess" -or $HostessStagingExecutionRequestView.reject_template.reject_status -ne "pending" -or $HostessStagingExecutionRequestView.reject_template.execution_in_studio -ne $false) {
            throw "shell Hostess staging execution request reject template mismatch"
        }
        if (@($HostessStagingExecutionRequestView.reject_template.request_action_ids).Count -ne 6 -or @($HostessStagingExecutionRequestView.reject_template.rejected_action_ids).Count -ne 0) {
            throw "shell Hostess staging execution request reject template action mismatch"
        }
    }
    $ShellHostessStagingAcceptanceArchiveManifestOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-manifest --checklist $ShellHostessStagingAcceptancePath --acceptance-id "synthetic-hostess-staging-ready-archive" --label "Synthetic Hostess staging ready acceptance archive" --output $ShellHostessStagingAcceptanceArchiveManifestPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance archive manifest failed with exit code $LASTEXITCODE"
    }
    $ShellHostessStagingAcceptanceMultiIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-index-append --acceptance-index $ShellHostessStagingAcceptanceIndexPath --acceptance-manifest $ShellHostessStagingAcceptanceArchiveManifestPath --default-acceptance-id "synthetic-hostess-staging-ready-archive" --output $ShellHostessStagingAcceptanceMultiIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance index append failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptanceMultiIndexPath)) {
        throw "shell Hostess staging acceptance multi index was not written"
    }
    $ShellHostessStagingAcceptanceMultiIndex = ($ShellHostessStagingAcceptanceMultiIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($ShellHostessStagingAcceptanceMultiIndex.default_acceptance_id -ne "synthetic-hostess-staging-ready-archive" -or $ShellHostessStagingAcceptanceMultiIndex.acceptance_count -ne 2 -or $ShellHostessStagingAcceptanceMultiIndex.ready_acceptance_count -ne 2) {
        throw "shell Hostess staging acceptance multi index mismatch"
    }
    if (@($ShellHostessStagingAcceptanceMultiIndex.entries | Where-Object { $_.acceptance_id -eq "synthetic-hostess-staging-ready-archive" -and $_.acceptance_manifest_path -eq $ShellHostessStagingAcceptanceArchiveManifestPath }).Count -ne 1) {
        throw "shell Hostess staging acceptance multi index missing archive"
    }
    $ShellHostessStagingAcceptancePromotedIndexOutput = & cargo run --quiet -p rusty-studio-cli -- shell-hostess-staging-acceptance-index-promote --acceptance-index $ShellHostessStagingAcceptanceMultiIndexPath --acceptance-id "synthetic-hostess-staging-ready" --output $ShellHostessStagingAcceptancePromotedIndexPath
    if ($LASTEXITCODE -ne 0) {
        throw "studio shell Hostess staging acceptance index promote failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path $ShellHostessStagingAcceptancePromotedIndexPath)) {
        throw "shell Hostess staging acceptance promoted index was not written"
    }
    $ShellHostessStagingAcceptancePromotedIndex = ($ShellHostessStagingAcceptancePromotedIndexOutput -join [Environment]::NewLine) | ConvertFrom-Json
    if ($ShellHostessStagingAcceptancePromotedIndex.default_acceptance_id -ne "synthetic-hostess-staging-ready" -or $ShellHostessStagingAcceptancePromotedIndex.acceptance_count -ne 2) {
        throw "shell Hostess staging acceptance promoted index mismatch"
    }
}
