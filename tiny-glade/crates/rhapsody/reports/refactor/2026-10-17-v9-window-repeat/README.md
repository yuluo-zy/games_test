# Owned temporal names and sustained Window Lab scripts

The named-resource tables and error payloads now own `Arc<str>` keys inside the session. Borrowed lookups reuse the canonical key; unknown reset requests create no key. Failed dynamic requests no longer add names to a process-global interner. Confirmation and rollback drain keys prepared before submission. The direct rhapsody `ustr` dependency was removed; other workspace users keep their dependency.

Window Lab accepts `--repeat-script` with shader or WSI stress scripts. Shader rounds alternate valid variants after an intentional invalid replacement, retain the published pipeline until replacement succeeds, and validate against the corresponding CPU pixels. WSI repeats complete 100-resize/20-minimize-restore batches, retains at most 140 diagnostic events, and drains pending native acknowledgements without exceeding the requested presented frame count. The default 360-frame scripts retain their original single-round behavior.

Reports include initialization/prewarm time, time to first present, and an independent bounded distribution for the first 300 presented frames. Framework percentiles are computed from each frame's stage sum. Root-owned stability samples and teardown accounting are included in the saved executable.

Validation: temporal/error CPU 15/15; Window CPU 21/21; standalone all-target strict Clippy and build succeeded. New CPU cases cover 1025 failed dynamic names, unknown resets, key lifetime, repeated scripts, pending native-action draining, bounded diagnostics, and cold/steady sampling. See `result.json` and the accompanying logs.

The final binary is `/private/tmp/rhapsody-window-repeat-20261017`; `build-manifest.json` binds its SHA-256 to the unchanged source fingerprint during that build. Actual repeated native-window and cold/warm runs remain with the root's unified runner. This report records no native-window or GPU PASS for these new scripts.

The standalone lock needed only removal of the obsolete `append-only-vec` package/edge in addition to the rhapsody `ustr` edge; no unrelated dependency versions were upgraded.
