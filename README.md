# Rusty Studio

Rusty Studio is the schema-first authoring and export-planning app for the
clean Rusty stack. The core semantics are plain Rust model/core/CLI crates so
Studio behavior can be validated before and alongside the Makepad graph editor.

Current scope:

- model a Studio project graph;
- validate node and edge topology;
- resolve a selected host profile;
- produce an export-plan skeleton;
- expose deterministic CLI commands for agents and future GUI parity;
- expose catalog package/module and host-profile palette rows through the
  shared view model;
- expose actionable failed validation diagnostics with graph/reference targets
  through the shared view model and Makepad surface;
- expose validation issue counts on graph, node, and edge rows for future
  issue-to-row canvas focus;
- expose deterministic and requested validation issue focus for inspector and
  future canvas selection;
- expose read-only selected node inspector details for package, module, and
  host-profile references through the shared view model;
- expose read-only selected edge inspector details for graph bindings and
  endpoint references through the shared view model;
- expose optional schema-only graph layout hints for nodes and edges through
  the shared view model and Makepad read-only canvas;
- render those layout hints in a Makepad custom read-only node graph canvas;
- route Makepad canvas node and edge selection through shared view-model
  selection requests;
- add the next available catalog module from the full palette or from a
  selected package through a shared core/CLI/Makepad mutation path;
- render the resolved synthetic project in a Makepad desktop authoring surface;
- navigate between desktop and headset graph targets without mutating project
  state;
- retarget a graph host profile through a shared core/CLI/Makepad mutation
  path with edit reports and revision bumps;
- add package/module graph links through a shared core/CLI/Makepad mutation
  path with catalog-backed validation and revision bumps;
- remove module graph links through the same shared mutation path with
  incident-edge cleanup and validation before commit;
- add and remove typed graph bindings through a shared core/CLI/Makepad
  mutation path with endpoint-kind checks and revision bumps;
- remove the selected module node or selected binding edge from Makepad by
  deriving the same core edit request used by CLI commands;
- add a command binding from the selected graph's operator shell to the
  selected module node through the same shared edit route;
- export and validate a schema-only shell descriptor for a selected graph;
- export a schema-only shell artifact manifest for desktop, phone, and Quest
  operator shell descriptors;
- export per-target shell template manifests with staged descriptors while
  preserving Hostess/Manifold runtime authority;
- validate generated shell template indexes and manifests;
- expose a selected-graph shell descriptor/template preview through the shared
  view model and Makepad surface;
- export a selected-graph schema-only shell bundle from shared core logic,
  CLI, and Makepad preview action;
- validate a selected-graph shell bundle against the current graph preview
  through shared core logic, CLI, and Makepad preview action;
- prepare validation-gated desktop, phone, and Quest shell handoff reports
  from selected bundles without spawning, installing, launching, or collecting
  evidence;
- inspect multi-target shell handoff readiness across exported selected graph
  bundles through shared core logic, CLI, and Makepad;
- correlate shell handoff readiness with export-plan package, module,
  operator-shell, and target-profile coverage;
- summarize shell handoff readiness by desktop, phone, and Quest target groups
  with ready, failed, and missing-bundle counts;
- surface ready, failed, missing bundle, and template-index paths in target
  readiness summaries so failed target rows are actionable;
- write a durable schema-only shell handoff manifest that archives readiness
  paths, target summaries, consumer arguments, and authority boundaries for
  future Hostess/Manifold-owned routes;
- validate shell handoff manifests as downstream intake contracts without
  granting Studio install, launch, command-session, or evidence authority;
- render a minimal Makepad desktop shell from a descriptor, artifact manifest,
  or shell-template index.

Current non-scope:

- freeform editable graph canvas;
- runtime sockets;
- device APIs;
- Android, Quest, OpenXR, ADB, BLE, camera, or media dependencies;
- dynamic plugin loading.

## Commands

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
cargo run -p rusty-studio-cli -- validate --project examples\synthetic-studio-project.json
cargo run -p rusty-studio-cli -- resolve --project examples\synthetic-studio-project.json
cargo run -p rusty-studio-cli -- export-plan --project examples\synthetic-studio-project.json
cargo run -p rusty-studio-cli -- view-model --project examples\synthetic-studio-project.json
cargo run -p rusty-studio-cli -- view-model --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_headset
cargo run -p rusty-studio-cli -- view-model --project target\studio-diagnostic-project.json --graph studio.graph.synthetic_wave_desktop --issue studio.check.graph.studio.graph.synthetic_wave_desktop.package_refs
cargo run -p rusty-studio-cli -- view-model --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --node node.host_profile.desktop
cargo run -p rusty-studio-cli -- view-model --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --edge edge.provider_to_processor
cargo run -p rusty-studio-cli -- retarget-host --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --host-profile host_run.profile.headset --output target\studio-edit-retarget-headset.json
cargo run -p rusty-studio-cli -- add-module --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --package package.biosignal_sensor --module module.biosignal_sensor.provider --label "Biosignal Provider" --output target\studio-edit-add-module.json
cargo run -p rusty-studio-cli -- add-palette-module --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --output target\studio-edit-add-palette-module.json
cargo run -p rusty-studio-cli -- add-palette-module --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --package package.hand_animation --output target\studio-edit-add-selected-package-module.json
cargo run -p rusty-studio-cli -- remove-module --project target\studio-edit-add-module.json --graph studio.graph.synthetic_wave_desktop --module module.biosignal_sensor.provider --output target\studio-edit-remove-module.json
cargo run -p rusty-studio-cli -- add-binding --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --kind command --source-node node.shell.operator --target-node node.module.synthetic_wave_provider --output target\studio-edit-add-binding.json
cargo run -p rusty-studio-cli -- remove-binding --project target\studio-edit-add-binding.json --graph studio.graph.synthetic_wave_desktop --kind command --source-node node.shell.operator --target-node node.module.synthetic_wave_provider --output target\studio-edit-remove-binding.json
cargo run -p rusty-studio-cli -- shell-descriptor --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --output target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-cli -- validate-shell-descriptor --descriptor target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-cli -- shell-artifacts --project examples\synthetic-studio-project.json --output-dir target\studio-shells
cargo run -p rusty-studio-cli -- validate-shell-artifacts --manifest target\studio-shells\shell-artifacts.json
cargo run -p rusty-studio-cli -- shell-templates --manifest target\studio-shells\shell-artifacts.json --output-dir target\studio-shell-templates
cargo run -p rusty-studio-cli -- validate-shell-templates --index target\studio-shell-templates\shell-templates.json
cargo run -p rusty-studio-cli -- shell-bundle --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --output-dir target\studio-selected-shell\studio.graph.synthetic_wave_desktop
cargo run -p rusty-studio-cli -- validate-shell-bundle --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --bundle-dir target\studio-selected-shell\studio.graph.synthetic_wave_desktop
cargo run -p rusty-studio-cli -- shell-handoff --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --bundle-dir target\studio-selected-shell\studio.graph.synthetic_wave_desktop
cargo run -p rusty-studio-cli -- desktop-shell-handoff --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --bundle-dir target\studio-selected-shell\studio.graph.synthetic_wave_desktop
cargo run -p rusty-studio-cli -- shell-bundle --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_phone --output-dir target\studio-selected-shell\studio.graph.synthetic_wave_phone
cargo run -p rusty-studio-cli -- shell-handoff --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_phone --bundle-dir target\studio-selected-shell\studio.graph.synthetic_wave_phone
cargo run -p rusty-studio-cli -- shell-bundle --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_headset --output-dir target\studio-selected-shell\studio.graph.synthetic_wave_headset
cargo run -p rusty-studio-cli -- shell-handoff --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_headset --bundle-dir target\studio-selected-shell\studio.graph.synthetic_wave_headset
cargo run -p rusty-studio-cli -- shell-handoff-readiness --project examples\synthetic-studio-project.json --bundle-root target\studio-selected-shell
cargo run -p rusty-studio-cli -- shell-handoff-manifest --project examples\synthetic-studio-project.json --bundle-root target\studio-selected-shell --output target\studio-shell-handoffs\shell-handoffs.json
cargo run -p rusty-studio-cli -- validate-shell-handoff-manifest --manifest target\studio-shell-handoffs\shell-handoffs.json
cargo run -p rusty-studio-makepad -- --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_headset
cargo run -p rusty-studio-desktop-shell -- --descriptor target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-desktop-shell -- --manifest target\studio-shells\shell-artifacts.json
cargo run -p rusty-studio-desktop-shell -- --templates target\studio-shell-templates\shell-templates.json
```

All CLI commands print JSON. The Makepad surface uses the same core edit
operations and displays the resulting edit report after an accepted or rejected
request. The desktop shell is a read-only descriptor, artifact-manifest, or
template-index consumer; it does not own runtime command/session authority,
launch, or evidence collection.
