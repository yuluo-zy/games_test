# 2026-10-15 country-core recovery evidence

This directory archives the bounded analysis inputs, native instruction excerpts,
decompilations, inspection scripts and delivery reports from the October 15 work.
The maintained Rust implementation is in
[`tiny-glade/crates/country-core`](../../../../tiny-glade/crates/country-core/).
Archiving these records does not establish full game integration or original
behavioral equivalence; each domain report describes its own remaining boundaries.

## Navigation

| Area | Evidence directories |
| --- | --- |
| UI, input and modes | `gui/`, `input/`, `mode/`, `glade-settings/` |
| Session and persistence | `session/`, `save/` |
| Buildings and terrain | `walls/`, `roofs-stairs/`, `terrain/` |
| Vegetation and decoration | `garden/`, `ivy/`, `trees/`, `clutter/`, `ui-clutter/` |
| Creatures and audio | `critters/`, `music/` |
| Rendering | `raster/` |

Use each area's `REPORT.md` where present. Companion specifications and coordination
notes are in [`../core-recovery-20261015-adv`](../core-recovery-20261015-adv/).
Those specifications record work requirements and are not completion certificates.
MCP jobs and captured results are archived in [`../mcp`](../mcp/); the separate
[`Claude CLI report`](../claude-call-repair-2026-10-15/REPORT.md) concerns tool
availability rather than recovered game behavior.

## Publication boundaries

- Native excerpts and captured tool output retain their original formatting,
  including alignment spaces in assembly/decompilation listings.
- Local executables, DLLs, PDBs, compiler intermediates, generated integrity test
  directories and third-party download trees are excluded by the root `.gitignore`.
  The local files are preserved; reports and provenance remain versioned.
- The `music/REPORT.md` records the original-lock checksum of the downloaded
  `libfmod` crate. Its download and extracted third-party implementation stay local.
- The separately archived `reconstruction/country-core/Cargo.{toml,lock}` describe
  the historical standalone reconstruction. They do not replace the maintained
  workspace manifests or establish that historical dependency substitutions match
  the original game.

The publication pass validated the new JSON records and scanned for common secret
patterns. It did not rerun Rust builds, GPU/audio integration or original-code
comparisons, and does not supersede the validation scope recorded in domain reports.
