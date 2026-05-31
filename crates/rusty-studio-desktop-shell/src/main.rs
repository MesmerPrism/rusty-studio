pub use makepad_widgets;

use makepad_widgets::*;
use rusty_studio_core::{
    load_shell_artifact_manifest, load_shell_descriptor, load_shell_template_index,
    load_shell_template_manifest, validate_shell_artifact_manifest, validate_shell_descriptor,
    validate_shell_template_index,
};
use rusty_studio_model::{
    StudioShellArtifact, StudioShellArtifactManifest, StudioShellBinding, StudioShellDescriptor,
    StudioShellTargetKind, StudioShellTemplateIndex, StudioShellTemplateIndexEntry,
    StudioShellTemplateManifest, StudioValidationStatus,
};
use std::path::{Path, PathBuf};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    let PageTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x111827
        draw_text.text_style: theme.font_bold{font_size: 24.0}
    }

    let SectionTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x263238
        draw_text.text_style: theme.font_bold{font_size: 16.0}
    }

    let FieldLabel = Label{
        width: 150.0 height: Fit
        draw_text.color: #x5d6875
        draw_text.text_style.font_size: 12.0
    }

    let FieldValue = Label{
        width: Fill height: Fit
        draw_text.color: #x111827
        draw_text.text_style.font_size: 13.0
    }

    let SmallValue = Label{
        width: Fill height: Fit
        draw_text.color: #x3f4a54
        draw_text.text_style.font_size: 12.0
    }

    let Panel = RoundedView{
        width: Fill height: Fit
        flow: Down
        spacing: 8.0
        padding: 14.0
        draw_bg +: {
            color: #xffffff
            border_color: #xd8dde3
            border_size: 1.0
            border_radius: 8.0
        }
    }

    let Row = View{
        width: Fill height: Fit
        flow: Right
        spacing: 10.0
        align: Align{y: 0.5}
    }

    let Rule = SolidView{
        width: Fill height: 1.0
        draw_bg.color: #xe7ebef
    }

    let DescriptorPanel = Panel{
        SectionTitle{text: "Descriptor"}
        Row{FieldLabel{text: "source"} descriptor_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "descriptor"} descriptor_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "project / graph"} descriptor_project_graph := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} descriptor_validation := FieldValue{text: ""}}
    }

    let ManifestPanel = Panel{
        SectionTitle{text: "Shell Artifact Index"}
        Row{FieldLabel{text: "manifest"} manifest_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "bundle"} manifest_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} manifest_validation := FieldValue{text: ""}}
        Row{FieldLabel{text: "artifacts"} artifact_rows := SmallValue{text: ""}}
    }

    let TemplatePanel = Panel{
        SectionTitle{text: "Shell Template Index"}
        Row{FieldLabel{text: "index"} template_index_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "bundle"} template_index_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} template_index_validation := FieldValue{text: ""}}
        Row{FieldLabel{text: "templates"} template_rows := SmallValue{text: ""}}
        Row{FieldLabel{text: "authority"} template_authority := SmallValue{text: ""}}
    }

    let HostPanel = Panel{
        SectionTitle{text: "Host"}
        Row{FieldLabel{text: "target"} host_target := FieldValue{text: ""}}
        Row{FieldLabel{text: "profile"} host_profile := FieldValue{text: ""}}
        Row{FieldLabel{text: "routes"} host_routes := SmallValue{text: ""}}
        Row{FieldLabel{text: "permissions"} host_permissions := SmallValue{text: ""}}
    }

    let PackagePanel = Panel{
        SectionTitle{text: "Package Graph"}
        Row{FieldLabel{text: "packages"} package_ids := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "modules"} module_ids := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "validation"} validation_slot_ids := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "stream bindings"} stream_bindings := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "command bindings"} command_bindings := SmallValue{text: ""}}
    }

    let AuthorityPanel = Panel{
        SectionTitle{text: "Authority"}
        Row{FieldLabel{text: "mode"} shell_mode := FieldValue{text: "read-only descriptor client"}}
        Row{FieldLabel{text: "boundary"} authority_note := SmallValue{text: ""}}
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass.clear_color: #xf4f6f7
                window.inner_size: vec2(1040, 760)
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 0.0

                    SolidView{
                        width: Fill height: Fit
                        padding: Inset{left: 24.0 right: 24.0 top: 18.0 bottom: 16.0}
                        flow: Right
                        align: Align{y: 0.5}
                        draw_bg.color: #xfbfcf8

                        View{
                            width: Fill height: Fit
                            flow: Down
                            spacing: 3.0
                            PageTitle{text: "Rusty Desktop Shell"}
                            subtitle_label := Label{
                                text: "descriptor-driven operator shell prototype"
                                draw_text.color: #x5d6875
                                draw_text.text_style.font_size: 12.0
                            }
                        }
                        mode_label := Label{
                            width: Fit height: Fit
                            text: "read-only client"
                            draw_text.color: #x2f6f5e
                            draw_text.text_style: theme.font_bold{font_size: 13.0}
                        }
                    }

                    Rule{}

                    ScrollYView{
                        width: Fill height: Fill
                        padding: 18.0
                        flow: Down
                        spacing: 12.0

                        ManifestPanel{}
                        TemplatePanel{}
                        DescriptorPanel{}
                        HostPanel{}
                        PackagePanel{}
                        AuthorityPanel{}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    manifest_source: Option<PathBuf>,
    #[rust]
    manifest: Option<StudioShellArtifactManifest>,
    #[rust]
    template_index_source: Option<PathBuf>,
    #[rust]
    template_index: Option<StudioShellTemplateIndex>,
    #[rust]
    template_manifest: Option<StudioShellTemplateManifest>,
    #[rust]
    descriptor_source: Option<PathBuf>,
    #[rust]
    descriptor: Option<StudioShellDescriptor>,
}

impl App {
    fn sync_descriptor(&mut self, cx: &mut Cx) {
        match load_initial_shell_input() {
            Ok(input) => self.set_shell_input(cx, input),
            Err(error) => self.sync_error(cx, &error),
        }
    }

    fn set_shell_input(&mut self, cx: &mut Cx, input: LoadedShellInput) {
        self.manifest_source = input.manifest_source;
        self.manifest = input.manifest;
        self.template_index_source = input.template_index_source;
        self.template_index = input.template_index;
        self.template_manifest = input.template_manifest;
        self.descriptor_source = Some(input.descriptor_source);
        self.descriptor = Some(input.descriptor);
        self.sync_loaded_descriptor(cx);
    }

    fn sync_loaded_descriptor(&mut self, cx: &mut Cx) {
        let Some(descriptor) = self.descriptor.clone() else {
            self.sync_error(cx, "no shell descriptor loaded");
            return;
        };
        self.sync_template_index(cx);
        self.sync_manifest(cx);
        let source = self.descriptor_source.clone().unwrap_or_default();
        self.ui
            .label(cx, ids!(descriptor_source))
            .set_text(cx, &source.display().to_string());
        self.ui
            .label(cx, ids!(descriptor_identity))
            .set_text(cx, &descriptor_identity_line(&descriptor));
        self.ui
            .label(cx, ids!(descriptor_project_graph))
            .set_text(cx, &descriptor_project_graph_line(&descriptor));
        self.ui
            .label(cx, ids!(descriptor_validation))
            .set_text(cx, &descriptor_validation_line(&descriptor));
        self.ui
            .label(cx, ids!(host_target))
            .set_text(cx, &descriptor.target_host_profile);
        self.ui
            .label(cx, ids!(host_profile))
            .set_text(cx, &host_profile_line(&descriptor));
        self.ui
            .label(cx, ids!(host_routes))
            .set_text(cx, &host_route_lines(&descriptor));
        self.ui
            .label(cx, ids!(host_permissions))
            .set_text(cx, &id_lines(&descriptor.host_profile.required_permissions));
        self.ui
            .label(cx, ids!(package_ids))
            .set_text(cx, &id_lines(&descriptor.package_ids));
        self.ui
            .label(cx, ids!(module_ids))
            .set_text(cx, &id_lines(&descriptor.module_ids));
        self.ui
            .label(cx, ids!(validation_slot_ids))
            .set_text(cx, &id_lines(&descriptor.validation_slot_ids));
        self.ui
            .label(cx, ids!(stream_bindings))
            .set_text(cx, &binding_lines(&descriptor.stream_bindings));
        self.ui
            .label(cx, ids!(command_bindings))
            .set_text(cx, &binding_lines(&descriptor.command_bindings));
        self.ui.label(cx, ids!(authority_note)).set_text(
            cx,
            "This shell consumes a Studio descriptor. Manifold and Hostess own runtime authority, command sessions, launch, and evidence.",
        );
    }

    fn sync_manifest(&mut self, cx: &mut Cx) {
        if let (Some(source), Some(manifest)) =
            (self.manifest_source.as_ref(), self.manifest.as_ref())
        {
            self.ui
                .label(cx, ids!(manifest_source))
                .set_text(cx, &source.display().to_string());
            self.ui
                .label(cx, ids!(manifest_identity))
                .set_text(cx, &manifest_identity_line(manifest));
            self.ui
                .label(cx, ids!(manifest_validation))
                .set_text(cx, &manifest_validation_line(manifest, source.parent()));
            self.ui
                .label(cx, ids!(artifact_rows))
                .set_text(cx, &artifact_lines(&manifest.artifacts));
        } else {
            self.ui
                .label(cx, ids!(manifest_source))
                .set_text(cx, "direct descriptor input");
            self.ui.label(cx, ids!(manifest_identity)).set_text(cx, "");
            self.ui
                .label(cx, ids!(manifest_validation))
                .set_text(cx, "");
            self.ui.label(cx, ids!(artifact_rows)).set_text(cx, "");
        }
    }

    fn sync_template_index(&mut self, cx: &mut Cx) {
        if let (Some(source), Some(index)) = (
            self.template_index_source.as_ref(),
            self.template_index.as_ref(),
        ) {
            self.ui
                .label(cx, ids!(template_index_source))
                .set_text(cx, &source.display().to_string());
            self.ui
                .label(cx, ids!(template_index_identity))
                .set_text(cx, &template_index_identity_line(index));
            self.ui
                .label(cx, ids!(template_index_validation))
                .set_text(cx, &template_index_validation_line(index, source.parent()));
            self.ui
                .label(cx, ids!(template_rows))
                .set_text(cx, &template_lines(&index.templates));
            self.ui.label(cx, ids!(template_authority)).set_text(
                cx,
                &template_authority_line(self.template_manifest.as_ref()),
            );
        } else {
            self.ui
                .label(cx, ids!(template_index_source))
                .set_text(cx, "no template index input");
            self.ui
                .label(cx, ids!(template_index_identity))
                .set_text(cx, "");
            self.ui
                .label(cx, ids!(template_index_validation))
                .set_text(cx, "");
            self.ui.label(cx, ids!(template_rows)).set_text(cx, "");
            self.ui.label(cx, ids!(template_authority)).set_text(cx, "");
        }
    }

    fn sync_error(&mut self, cx: &mut Cx, error: &str) {
        self.ui.label(cx, ids!(manifest_source)).set_text(cx, "");
        self.ui
            .label(cx, ids!(manifest_identity))
            .set_text(cx, "shell input load failed");
        self.ui
            .label(cx, ids!(manifest_validation))
            .set_text(cx, error);
        self.ui.label(cx, ids!(artifact_rows)).set_text(cx, "");
        self.ui
            .label(cx, ids!(template_index_source))
            .set_text(cx, "");
        self.ui
            .label(cx, ids!(template_index_identity))
            .set_text(cx, "template input load failed");
        self.ui
            .label(cx, ids!(template_index_validation))
            .set_text(cx, error);
        self.ui.label(cx, ids!(template_rows)).set_text(cx, "");
        self.ui.label(cx, ids!(template_authority)).set_text(cx, "");
        self.ui.label(cx, ids!(descriptor_source)).set_text(cx, "");
        self.ui
            .label(cx, ids!(descriptor_identity))
            .set_text(cx, "descriptor load failed");
        self.ui
            .label(cx, ids!(descriptor_project_graph))
            .set_text(cx, "");
        self.ui
            .label(cx, ids!(descriptor_validation))
            .set_text(cx, error);
        self.ui.label(cx, ids!(host_target)).set_text(cx, "");
        self.ui.label(cx, ids!(host_profile)).set_text(cx, "");
        self.ui.label(cx, ids!(host_routes)).set_text(cx, "");
        self.ui.label(cx, ids!(host_permissions)).set_text(cx, "");
        self.ui.label(cx, ids!(package_ids)).set_text(cx, "");
        self.ui.label(cx, ids!(module_ids)).set_text(cx, "");
        self.ui
            .label(cx, ids!(validation_slot_ids))
            .set_text(cx, "");
        self.ui.label(cx, ids!(stream_bindings)).set_text(cx, "");
        self.ui.label(cx, ids!(command_bindings)).set_text(cx, "");
        self.ui.label(cx, ids!(authority_note)).set_text(cx, "");
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.sync_descriptor(cx);
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

struct LoadedShellInput {
    manifest_source: Option<PathBuf>,
    manifest: Option<StudioShellArtifactManifest>,
    template_index_source: Option<PathBuf>,
    template_index: Option<StudioShellTemplateIndex>,
    template_manifest: Option<StudioShellTemplateManifest>,
    descriptor_source: PathBuf,
    descriptor: StudioShellDescriptor,
}

fn load_initial_shell_input() -> Result<LoadedShellInput, String> {
    if let Some(template_index_path) = template_index_path_from_args() {
        return load_shell_input_from_template_index(&template_index_path);
    }
    if let Some(manifest_path) = manifest_path_from_args() {
        return load_shell_input_from_manifest(&manifest_path);
    }
    if let Some(descriptor_path) = descriptor_path_from_args() {
        let descriptor = load_descriptor_for_path(&descriptor_path)?;
        return Ok(LoadedShellInput {
            manifest_source: None,
            manifest: None,
            template_index_source: None,
            template_index: None,
            template_manifest: None,
            descriptor_source: descriptor_path,
            descriptor,
        });
    }
    if let Some(template_index_path) = find_default_template_index_path() {
        return load_shell_input_from_template_index(&template_index_path);
    }
    if let Some(manifest_path) = find_default_manifest_path() {
        return load_shell_input_from_manifest(&manifest_path);
    }
    let descriptor_path = find_default_descriptor_path().ok_or_else(|| {
        "no descriptor or artifact manifest path supplied and no default shell input was found"
            .to_string()
    })?;
    let descriptor = load_descriptor_for_path(&descriptor_path)?;
    Ok(LoadedShellInput {
        manifest_source: None,
        manifest: None,
        template_index_source: None,
        template_index: None,
        template_manifest: None,
        descriptor_source: descriptor_path,
        descriptor,
    })
}

fn load_descriptor_for_path(path: &Path) -> Result<StudioShellDescriptor, String> {
    load_shell_descriptor(path).map_err(|error| error.to_string())
}

fn descriptor_path_from_args() -> Option<PathBuf> {
    path_from_args("--descriptor")
}

fn manifest_path_from_args() -> Option<PathBuf> {
    path_from_args("--manifest")
}

fn template_index_path_from_args() -> Option<PathBuf> {
    path_from_args("--templates")
}

fn path_from_args(flag: &str) -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next().map(PathBuf::from);
        }
    }
    None
}

fn find_default_descriptor_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("target/studio-shell-descriptor-desktop.json"),
        current_dir.join("../../target/studio-shell-descriptor-desktop.json"),
        current_dir.join("../../../target/studio-shell-descriptor-desktop.json"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn find_default_manifest_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("target/studio-shells/shell-artifacts.json"),
        current_dir.join("../../target/studio-shells/shell-artifacts.json"),
        current_dir.join("../../../target/studio-shells/shell-artifacts.json"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn find_default_template_index_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("target/studio-shell-templates/shell-templates.json"),
        current_dir.join("../../target/studio-shell-templates/shell-templates.json"),
        current_dir.join("../../../target/studio-shell-templates/shell-templates.json"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn load_shell_input_from_manifest(path: &Path) -> Result<LoadedShellInput, String> {
    let manifest = load_shell_artifact_manifest(path).map_err(|error| error.to_string())?;
    let validation = validate_shell_artifact_manifest(&manifest, path.parent());
    if validation.status != StudioValidationStatus::Pass {
        return Err(manifest_validation_line(&manifest, path.parent()));
    }
    let artifact = selected_manifest_artifact(&manifest)
        .ok_or_else(|| "manifest does not declare a loadable shell artifact".to_string())?;
    let descriptor_source = manifest_descriptor_path(path, artifact);
    let descriptor = load_descriptor_for_path(&descriptor_source)?;
    Ok(LoadedShellInput {
        manifest_source: Some(path.to_path_buf()),
        manifest: Some(manifest),
        template_index_source: None,
        template_index: None,
        template_manifest: None,
        descriptor_source,
        descriptor,
    })
}

fn load_shell_input_from_template_index(path: &Path) -> Result<LoadedShellInput, String> {
    let index = load_shell_template_index(path).map_err(|error| error.to_string())?;
    let validation = validate_shell_template_index(&index, path.parent());
    if validation.status != StudioValidationStatus::Pass {
        return Err(template_index_validation_line(&index, path.parent()));
    }
    let entry = selected_template_entry(&index)
        .ok_or_else(|| "template index does not declare a loadable shell template".to_string())?;
    let template_source = template_manifest_path(path, entry);
    let template_manifest =
        load_shell_template_manifest(&template_source).map_err(|error| error.to_string())?;
    let descriptor_source = template_descriptor_path(path, entry);
    let descriptor = load_descriptor_for_path(&descriptor_source)?;
    Ok(LoadedShellInput {
        manifest_source: None,
        manifest: None,
        template_index_source: Some(path.to_path_buf()),
        template_index: Some(index),
        template_manifest: Some(template_manifest),
        descriptor_source,
        descriptor,
    })
}

fn selected_manifest_artifact(
    manifest: &StudioShellArtifactManifest,
) -> Option<&StudioShellArtifact> {
    manifest
        .artifacts
        .iter()
        .find(|artifact| artifact.target_kind == StudioShellTargetKind::Desktop)
        .or_else(|| manifest.artifacts.first())
}

fn selected_template_entry(
    index: &StudioShellTemplateIndex,
) -> Option<&StudioShellTemplateIndexEntry> {
    index
        .templates
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
        .or_else(|| index.templates.first())
}

fn manifest_descriptor_path(manifest_path: &Path, artifact: &StudioShellArtifact) -> PathBuf {
    artifact.descriptor_path.split('/').fold(
        manifest_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf(),
        |path, segment| path.join(segment),
    )
}

fn template_manifest_path(index_path: &Path, entry: &StudioShellTemplateIndexEntry) -> PathBuf {
    entry.template_path.split('/').fold(
        index_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf(),
        |path, segment| path.join(segment),
    )
}

fn template_descriptor_path(index_path: &Path, entry: &StudioShellTemplateIndexEntry) -> PathBuf {
    entry.descriptor_path.split('/').fold(
        index_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf(),
        |path, segment| path.join(segment),
    )
}

fn descriptor_identity_line(descriptor: &StudioShellDescriptor) -> String {
    format!("{} ({})", descriptor.display_name, descriptor.descriptor_id)
}

fn descriptor_project_graph_line(descriptor: &StudioShellDescriptor) -> String {
    format!(
        "{} rev {} / {} / {}",
        descriptor.project_id,
        descriptor.project_revision,
        descriptor.graph_id,
        descriptor.shell_id
    )
}

fn descriptor_validation_line(descriptor: &StudioShellDescriptor) -> String {
    let report = validate_shell_descriptor(descriptor);
    let pass_count = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Pass)
        .count();
    let fail_count = report.checks.len() - pass_count;
    format!(
        "{}; {pass_count} passing checks, {fail_count} failing checks",
        validation_status_word(report.status)
    )
}

fn manifest_identity_line(manifest: &StudioShellArtifactManifest) -> String {
    format!(
        "{} / {} rev {}",
        manifest.manifest_id, manifest.project_id, manifest.project_revision
    )
}

fn manifest_validation_line(
    manifest: &StudioShellArtifactManifest,
    base_dir: Option<&Path>,
) -> String {
    let report = validate_shell_artifact_manifest(manifest, base_dir);
    let pass_count = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Pass)
        .count();
    let fail_count = report.checks.len() - pass_count;
    format!(
        "{}; {pass_count} passing checks, {fail_count} failing checks",
        validation_status_word(report.status)
    )
}

fn template_index_identity_line(index: &StudioShellTemplateIndex) -> String {
    format!(
        "{} / {} rev {}",
        index.index_id, index.project_id, index.project_revision
    )
}

fn template_index_validation_line(
    index: &StudioShellTemplateIndex,
    base_dir: Option<&Path>,
) -> String {
    let report = validate_shell_template_index(index, base_dir);
    let pass_count = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Pass)
        .count();
    let fail_count = report.checks.len() - pass_count;
    format!(
        "{}; {pass_count} passing checks, {fail_count} failing checks",
        validation_status_word(report.status)
    )
}

fn template_lines(entries: &[StudioShellTemplateIndexEntry]) -> String {
    if entries.is_empty() {
        return "none".to_string();
    }
    entries
        .iter()
        .map(|entry| {
            format!(
                "{}: {} / {}\n  template: {}\n  descriptor: {}",
                target_kind_label(entry.target_kind),
                entry.graph_id,
                entry.shell_id,
                entry.template_path,
                entry.descriptor_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn template_authority_line(template: Option<&StudioShellTemplateManifest>) -> String {
    let Some(template) = template else {
        return "".to_string();
    };
    format!(
        "command/session: {}\ninstall/launch/evidence: {}\nstudio: {}",
        template.runtime_authority.command_session_authority,
        template.runtime_authority.install_launch_evidence_authority,
        template.runtime_authority.studio_role
    )
}

fn artifact_lines(artifacts: &[StudioShellArtifact]) -> String {
    if artifacts.is_empty() {
        return "none".to_string();
    }
    artifacts
        .iter()
        .map(|artifact| {
            format!(
                "{}: {} / {}\n  app: {}\n  descriptor: {}",
                target_kind_label(artifact.target_kind),
                artifact.graph_id,
                artifact.shell_id,
                optional_value(artifact.app_id.as_deref()),
                artifact.descriptor_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn target_kind_label(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}

fn validation_status_word(status: StudioValidationStatus) -> &'static str {
    match status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    }
}

fn host_profile_line(descriptor: &StudioShellDescriptor) -> String {
    format!(
        "{} / app {}",
        optional_value(descriptor.host_profile.host_profile.as_deref()),
        optional_value(descriptor.host_profile.app_id.as_deref())
    )
}

fn host_route_lines(descriptor: &StudioShellDescriptor) -> String {
    [
        ("install", descriptor.host_profile.install_route.as_deref()),
        ("launch", descriptor.host_profile.launch_route.as_deref()),
        ("command", descriptor.host_profile.command_bridge.as_deref()),
        (
            "evidence",
            descriptor.host_profile.evidence_pull_route.as_deref(),
        ),
    ]
    .into_iter()
    .map(|(label, value)| format!("{label}: {}", optional_value(value)))
    .collect::<Vec<_>>()
    .join("\n")
}

fn id_lines(ids: &[String]) -> String {
    if ids.is_empty() {
        "none".to_string()
    } else {
        ids.join("\n")
    }
}

fn binding_lines(bindings: &[StudioShellBinding]) -> String {
    if bindings.is_empty() {
        return "none".to_string();
    }
    bindings
        .iter()
        .map(|binding| {
            format!(
                "{}: {} -> {}",
                binding.binding_id, binding.source_node_id, binding.target_node_id
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn optional_value(value: Option<&str>) -> &str {
    value.unwrap_or("not declared")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_studio_model::{
        StudioShellArtifact, StudioShellHostProfile, StudioShellHostRoutes,
        StudioShellRuntimeAuthority, StudioShellTemplateIndexEntry, SHELL_ARTIFACT_MANIFEST_SCHEMA,
        SHELL_DESCRIPTOR_SCHEMA, SHELL_TEMPLATE_INDEX_SCHEMA, SHELL_TEMPLATE_MANIFEST_SCHEMA,
    };

    fn sample_descriptor() -> StudioShellDescriptor {
        StudioShellDescriptor {
            schema_id: SHELL_DESCRIPTOR_SCHEMA.to_string(),
            descriptor_id: "studio.shell_descriptor.test".to_string(),
            project_id: "studio.project.test".to_string(),
            project_revision: 1,
            graph_id: "studio.graph.test".to_string(),
            display_name: "Test Shell".to_string(),
            shell_id: "shell.test.operator".to_string(),
            shell_label: "Operator".to_string(),
            target_host_profile: "host_run.profile.desktop".to_string(),
            host_profile: StudioShellHostProfile {
                profile_id: "host_run.profile.desktop".to_string(),
                host_profile: Some("host.desktop".to_string()),
                app_id: Some("app.host_shell.desktop".to_string()),
                install_route: Some("install.local_process".to_string()),
                launch_route: Some("launch.local_process".to_string()),
                command_bridge: Some("bridge.local_cli".to_string()),
                evidence_pull_route: Some("evidence.filesystem".to_string()),
                required_permissions: vec![],
            },
            package_ids: vec!["package.synthetic".to_string()],
            module_ids: vec!["module.synthetic_provider".to_string()],
            validation_slot_ids: vec!["validation.synthetic".to_string()],
            stream_bindings: vec![StudioShellBinding {
                binding_id: "edge.provider_to_processor".to_string(),
                source_node_id: "node.module.provider".to_string(),
                target_node_id: "node.module.processor".to_string(),
            }],
            command_bindings: vec![],
        }
    }

    fn sample_manifest() -> StudioShellArtifactManifest {
        StudioShellArtifactManifest {
            schema_id: SHELL_ARTIFACT_MANIFEST_SCHEMA.to_string(),
            manifest_id: "studio.shell_artifacts.test".to_string(),
            project_id: "studio.project.test".to_string(),
            project_revision: 1,
            artifacts: vec![
                StudioShellArtifact {
                    artifact_id: "studio.shell_artifact.phone".to_string(),
                    graph_id: "studio.graph.phone".to_string(),
                    shell_id: "shell.test.phone".to_string(),
                    target_kind: StudioShellTargetKind::Phone,
                    target_host_profile: "host_run.profile.mobile".to_string(),
                    host_profile_class: Some("host.mobile".to_string()),
                    descriptor_path: "descriptors/phone.json".to_string(),
                    app_id: Some("app.host_shell.mobile".to_string()),
                    install_route: Some("install.android_package".to_string()),
                    launch_route: Some("launch.android_intent".to_string()),
                    command_bridge: Some("bridge.adb_intent_file".to_string()),
                    evidence_pull_route: Some("evidence.adb_pull".to_string()),
                    package_ids: vec!["package.synthetic".to_string()],
                    module_ids: vec!["module.synthetic_provider".to_string()],
                },
                StudioShellArtifact {
                    artifact_id: "studio.shell_artifact.desktop".to_string(),
                    graph_id: "studio.graph.desktop".to_string(),
                    shell_id: "shell.test.desktop".to_string(),
                    target_kind: StudioShellTargetKind::Desktop,
                    target_host_profile: "host_run.profile.desktop".to_string(),
                    host_profile_class: Some("host.desktop".to_string()),
                    descriptor_path: "descriptors/desktop.json".to_string(),
                    app_id: Some("app.host_shell.desktop".to_string()),
                    install_route: Some("install.local_process".to_string()),
                    launch_route: Some("launch.local_process".to_string()),
                    command_bridge: Some("bridge.local_cli".to_string()),
                    evidence_pull_route: Some("evidence.filesystem".to_string()),
                    package_ids: vec!["package.synthetic".to_string()],
                    module_ids: vec!["module.synthetic_provider".to_string()],
                },
            ],
        }
    }

    fn sample_template_index() -> StudioShellTemplateIndex {
        StudioShellTemplateIndex {
            schema_id: SHELL_TEMPLATE_INDEX_SCHEMA.to_string(),
            index_id: "studio.shell_templates.test".to_string(),
            manifest_id: "studio.shell_artifacts.test".to_string(),
            project_id: "studio.project.test".to_string(),
            project_revision: 1,
            templates: vec![
                StudioShellTemplateIndexEntry {
                    template_id: "studio.shell_template.phone".to_string(),
                    artifact_id: "studio.shell_artifact.phone".to_string(),
                    graph_id: "studio.graph.phone".to_string(),
                    shell_id: "shell.test.phone".to_string(),
                    target_kind: StudioShellTargetKind::Phone,
                    template_path: "shells/phone/phone.json".to_string(),
                    descriptor_path: "descriptors/phone.json".to_string(),
                },
                StudioShellTemplateIndexEntry {
                    template_id: "studio.shell_template.desktop".to_string(),
                    artifact_id: "studio.shell_artifact.desktop".to_string(),
                    graph_id: "studio.graph.desktop".to_string(),
                    shell_id: "shell.test.desktop".to_string(),
                    target_kind: StudioShellTargetKind::Desktop,
                    template_path: "shells/desktop/desktop.json".to_string(),
                    descriptor_path: "descriptors/desktop.json".to_string(),
                },
            ],
        }
    }

    fn sample_template_manifest() -> StudioShellTemplateManifest {
        StudioShellTemplateManifest {
            schema_id: SHELL_TEMPLATE_MANIFEST_SCHEMA.to_string(),
            template_id: "studio.shell_template.desktop".to_string(),
            artifact_id: "studio.shell_artifact.desktop".to_string(),
            graph_id: "studio.graph.desktop".to_string(),
            shell_id: "shell.test.desktop".to_string(),
            target_kind: StudioShellTargetKind::Desktop,
            target_host_profile: "host_run.profile.desktop".to_string(),
            host_profile_class: Some("host.desktop".to_string()),
            source_descriptor_path: "descriptors/desktop.json".to_string(),
            descriptor_path: "descriptors/desktop.json".to_string(),
            runtime_authority: StudioShellRuntimeAuthority {
                command_session_authority: "rusty.manifold".to_string(),
                install_launch_evidence_authority: "rusty.hostess".to_string(),
                studio_role: "authoring.export_planning".to_string(),
            },
            host_routes: StudioShellHostRoutes {
                app_id: Some("app.host_shell.desktop".to_string()),
                install_route: Some("install.local_process".to_string()),
                launch_route: Some("launch.local_process".to_string()),
                command_bridge: Some("bridge.local_cli".to_string()),
                evidence_pull_route: Some("evidence.filesystem".to_string()),
            },
            package_ids: vec!["package.synthetic".to_string()],
            module_ids: vec!["module.synthetic_provider".to_string()],
        }
    }

    #[test]
    fn descriptor_helpers_surface_pass_validation() {
        let descriptor = sample_descriptor();

        let line = descriptor_validation_line(&descriptor);

        assert!(line.starts_with("pass;"));
        assert!(line.ends_with("0 failing checks"));
        assert_eq!(
            descriptor_identity_line(&descriptor),
            "Test Shell (studio.shell_descriptor.test)"
        );
    }

    #[test]
    fn binding_lines_report_empty_and_present_bindings() {
        let descriptor = sample_descriptor();

        assert_eq!(binding_lines(&descriptor.command_bindings), "none");
        assert!(binding_lines(&descriptor.stream_bindings)
            .contains("edge.provider_to_processor: node.module.provider -> node.module.processor"));
    }

    #[test]
    fn manifest_helpers_prefer_desktop_and_list_artifacts() {
        let manifest = sample_manifest();
        let selected = selected_manifest_artifact(&manifest).expect("selected artifact");
        let lines = artifact_lines(&manifest.artifacts);

        assert_eq!(selected.target_kind, StudioShellTargetKind::Desktop);
        assert!(lines.contains("phone: studio.graph.phone / shell.test.phone"));
        assert!(lines.contains("desktop: studio.graph.desktop / shell.test.desktop"));
        assert!(manifest_identity_line(&manifest).contains("studio.project.test rev 1"));
    }

    #[test]
    fn template_helpers_prefer_desktop_and_surface_authority() {
        let index = sample_template_index();
        let template = sample_template_manifest();
        let selected = selected_template_entry(&index).expect("selected template");
        let lines = template_lines(&index.templates);
        let authority = template_authority_line(Some(&template));

        assert_eq!(selected.target_kind, StudioShellTargetKind::Desktop);
        assert!(lines.contains("phone: studio.graph.phone / shell.test.phone"));
        assert!(lines.contains("desktop: studio.graph.desktop / shell.test.desktop"));
        assert!(template_index_identity_line(&index).contains("studio.project.test rev 1"));
        assert!(authority.contains("command/session: rusty.manifold"));
        assert!(authority.contains("install/launch/evidence: rusty.hostess"));
    }
}
