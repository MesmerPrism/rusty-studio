# Rusty Studio Agent Notes

Rusty Studio is the authoring, validation, diagnostics, deployment-planning,
and generated-shell lane for the clean Rusty stack. It is not runtime authority.

## Boundaries

- Manifold owns command/session/lease/runtime authority.
- Manifold packages own package manifests, fixtures, and processor cores.
- Hostess owns install, launch, platform acquisition, and evidence collection.
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
- Use relative paths in committed fixtures.
- Keep private/local paths out of examples and docs intended for this repo.

## Validation

Use the repo wrapper:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

The graph-discoverable target is:

```powershell
just check
```

or, if `just` is unavailable, call the PowerShell wrapper directly.
