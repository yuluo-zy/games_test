# GCC windows Option correction candidate

Production remains frozen. Apply only `proposed/gcc-windows-option.patch` after central review.

The original main selection stores tag0 at GCC+0 when selecting sentinel999, or color0 with target.variant0; otherwise it stores tag1 and the selected color at GCC+4. This is not a ColorId zero fallback.

Independent serialize evidence: original to_json_string14085ee90 passes the actual `windows` key at142a95773, length7, and the GCC pointer itself at14085f071..085 to14158b780. The retained PDB-named decompilation identifies this callee specialization as `Option<ColorId>`. Original field visitor14085f550 was executed with `windows`, and returned successful result tag0 with field index10; `window_light` maps8, gates11, stairs14, unknown15.

Strongest wire evidence: `native_gcc_windows_oracle.py` executed the complete original serializer and the original visitor in a separate subprocess, using host allocator/reallocator/free/memmove/memcmp shims only. It did not invoke the PE entry point or replace any serializer/game logic. Native None serializes an explicitly present `"windows":null`; Some(0), Some(8), and Some(u32::MAX) serialize `0`, `8`, and `4294967295`. Exact full JSON and original hash are in `gcc-windows-original-execution.json`. No missing-key behavior is asserted.

Source migration audit: the only explicit windows literal in production is the resource roundtrip test, changed to Some(ColorId(8)). `load_game.rs:466` and `walls/color_selection.rs:355` construct GCC with struct-update Default and never unwrap windows. Default now correctly keeps None. All other accesses to `.windows` found by the crate scan are unrelated method calls or water bitsets. No None-to-zero conversion is introduced.

Candidate adds a regression check for the four original wire fixtures and preserves the other fourteen fields. Cargo is deliberately left to the central compile owner; only the candidate was rustfmt checked.

Separate outstanding native type issues observed in the old GCC declaration (stairs struct and flag color storage) are not folded into this narrow patch and are not claimed fixed.
