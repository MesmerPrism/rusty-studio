# Rusty Studio Agent Notes

Rusty Studio is the authoring, validation, diagnostics, deployment-planning,
and generated-shell lane for Rusty Morphospace. It is not runtime authority.

Rusty Morphospace is the top-level project/platform umbrella. Studio remains
the workflow and authoring lane inside that umbrella; it may compose Matter,
Lattice, Manifold, Optics, GUI, Quest, and Hostess surfaces without becoming
their runtime authority.

Project-owned source in this repo is licensed `AGPL-3.0-or-later`. Keep
third-party dependencies, Makepad toolkit code, generated shell outputs,
package evidence, binary releases, and external tools under their own
provenance and notice requirements; see `docs/LICENSING.md`.

## Boundaries

- Manifold owns command/session/lease/runtime authority.
- Manifold packages own package manifests, fixtures, and processor cores.
- Hostess owns install, launch, platform acquisition, and evidence collection.
- Lattice owns situated relation contracts such as spaces, transforms, tracked
  poses, view sets, spatial input roles, frame-state binding, calibration,
  validity, confidence, and runtime capabilities.
- Rusty Studio owns authored project graphs, validation, export planning,
  diagnostics, and generated shell intent.
- Makepad will render Studio and generated shells, but core model and CLI logic
  must stay independent of Makepad.

## Slice Rules

- Keep this repo schema-first and deterministic.
- Every future GUI mutation needs a CLI route over the same implementation.
- Do not add Android, Quest, OpenXR, ADB, BLE, camera, media, dynamic plugin,
  or Makepad dependencies to `rusty-studio-model` or `rusty-studio-core`.
- Keep Makepad dependencies isolated to `rusty-studio-makepad` and later
  generated-shell UI crates.
- Keep Studio core, CLI, descriptors, fixtures, and export planning
  Makepad-free; Makepad is a UI shell dependency, not Studio authority.
- Studio may author and validate Lattice descriptors, but it must not become
  Lattice runtime authority or import OpenXR/Quest/Makepad runtime APIs into
  model/core.
- Use relative paths in committed fixtures.
- Keep private/local paths out of examples and docs intended for this repo.

## Validation

Use the repo wrapper:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

`tools/check_all.ps1` owns the ordered repo validation flow. Large scenario
families live under `tools/checks/`; currently
`tools/checks/studio_hostess_staging_check.ps1` owns the request-only
Hostess staging package, intake, preview, file-plan, handoff, acceptance,
PMB review, execution-request, and acceptance-index validation chain.

The graph-discoverable target is:

```powershell
just check
```

or, if `just` is unavailable, call the PowerShell wrapper directly.
