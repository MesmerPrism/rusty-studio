use super::super::super::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShellPreviewPanel = Panel{
        SectionTitle{text: "Shell Preview"}
        ButtonRow{
            export_shell_bundle_button := ActionButton{text: "Export Preview Files"}
            verify_shell_bundle_button := ActionButton{text: "Verify Preview Files"}
            shell_handoff_button := ActionButton{text: "Prepare Operator Shell"}
            shell_readiness_button := ActionButton{text: "Inspect All Handoffs"}
            shell_runbook_button := ActionButton{text: "Inspect Runbook"}
            shell_export_package_button := ActionButton{text: "Review Export Package"}
            shell_export_package_baseline_button := ActionButton{text: "Write Package Baseline"}
            shell_export_package_baseline_append_button := ActionButton{text: "Archive Package Baseline"}
            shell_export_package_baseline_summary_button := ActionButton{text: "Inspect Package Baseline"}
            shell_export_package_baseline_next_button := ActionButton{text: "Next Package Baseline"}
            shell_export_package_baseline_promote_button := ActionButton{text: "Promote Package Baseline"}
            shell_export_package_compare_button := ActionButton{text: "Compare Package"}
            shell_manifest_button := ActionButton{text: "Write Handoff Manifest"}
            shell_acceptance_button := ActionButton{text: "Review Acceptance"}
            shell_acceptance_baseline_button := ActionButton{text: "Write Baseline"}
            shell_acceptance_baseline_append_button := ActionButton{text: "Archive Baseline"}
            shell_acceptance_baseline_summary_button := ActionButton{text: "Inspect Baseline"}
            shell_acceptance_baseline_next_button := ActionButton{text: "Next Baseline"}
            shell_acceptance_baseline_promote_button := ActionButton{text: "Promote Baseline"}
            shell_acceptance_compare_button := ActionButton{text: "Compare Acceptance"}
            shell_release_candidate_button := ActionButton{text: "Review Release Candidate"}
            shell_release_candidate_manifest_button := ActionButton{text: "Write Candidate"}
            shell_release_candidate_append_button := ActionButton{text: "Archive Candidate"}
            shell_release_candidate_summary_button := ActionButton{text: "Inspect Candidate"}
            shell_release_candidate_next_button := ActionButton{text: "Next Candidate"}
            shell_release_candidate_promote_button := ActionButton{text: "Promote Candidate"}
            shell_hostess_handoff_package_button := ActionButton{text: "Review Hostess Package"}
            shell_hostess_owner_intake_button := ActionButton{text: "Review Hostess Intake"}
            shell_hostess_staging_preview_button := ActionButton{text: "Preview Hostess Staging"}
            shell_hostess_staging_file_plan_button := ActionButton{text: "Plan Staging Files"}
            shell_hostess_staging_handoff_button := ActionButton{text: "Prepare Hostess Handoff"}
            shell_hostess_staging_acceptance_button := ActionButton{text: "Check Hostess Handoff"}
            shell_hostess_staging_acceptance_append_button := ActionButton{text: "Archive Hostess Check"}
            shell_hostess_staging_acceptance_summary_button := ActionButton{text: "Inspect Hostess Checks"}
            shell_hostess_staging_acceptance_next_button := ActionButton{text: "Next Hostess Check"}
            shell_hostess_staging_acceptance_promote_button := ActionButton{text: "Promote Hostess Check"}
            shell_hostess_staging_acceptance_compare_button := ActionButton{text: "Compare Hostess Check"}
            shell_hostess_staging_execution_request_button := ActionButton{text: "Request Hostess Adapter"}
        }
        Row{FieldLabel{text: "descriptor"} shell_preview := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "routes"} shell_routes := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "template"} shell_template := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "bundle"} shell_bundle_status := SmallValue{text: ""}}
    }
}
