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
- render the resolved synthetic project in a Makepad desktop authoring surface;
- navigate between desktop and headset graph targets without mutating project
  state;
- retarget a graph host profile through a shared core/CLI/Makepad mutation
  path with edit reports and revision bumps;
- export and validate a schema-only shell descriptor for a selected graph;
- export a schema-only shell artifact manifest for desktop, phone, and Quest
  operator shell descriptors;
- export per-target shell template manifests with staged descriptors while
  preserving Hostess/Manifold runtime authority;
- validate generated shell template indexes and manifests;
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
cargo run -p rusty-studio-cli -- retarget-host --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --host-profile host_run.profile.headset --output target\studio-edit-retarget-headset.json
cargo run -p rusty-studio-cli -- shell-descriptor --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_desktop --output target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-cli -- validate-shell-descriptor --descriptor target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-cli -- shell-artifacts --project examples\synthetic-studio-project.json --output-dir target\studio-shells
cargo run -p rusty-studio-cli -- validate-shell-artifacts --manifest target\studio-shells\shell-artifacts.json
cargo run -p rusty-studio-cli -- shell-templates --manifest target\studio-shells\shell-artifacts.json --output-dir target\studio-shell-templates
cargo run -p rusty-studio-cli -- validate-shell-templates --index target\studio-shell-templates\shell-templates.json
cargo run -p rusty-studio-makepad -- --project examples\synthetic-studio-project.json --graph studio.graph.synthetic_wave_headset
cargo run -p rusty-studio-desktop-shell -- --descriptor target\studio-shell-descriptor-desktop.json
cargo run -p rusty-studio-desktop-shell -- --manifest target\studio-shells\shell-artifacts.json
cargo run -p rusty-studio-desktop-shell -- --templates target\studio-shell-templates\shell-templates.json
```

All CLI commands print JSON. The Makepad surface uses the same core retarget
operation and displays the resulting edit report after an accepted or rejected
request. The desktop shell is a read-only descriptor, artifact-manifest, or
template-index consumer; it does not own runtime command/session authority,
launch, or evidence collection.
