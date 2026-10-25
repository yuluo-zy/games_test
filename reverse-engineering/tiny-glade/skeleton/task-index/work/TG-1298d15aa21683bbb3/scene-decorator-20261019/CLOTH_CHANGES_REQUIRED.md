# Cloth retained gap

No cloth source change is included in albedo or the canonical non-cloth split. Current actual decorator producer uses local raw4/raw20 components, so proposed core cloth types alone do not close TypeId identity.

A definite correction to old source comments is now proven from original B build `1412a9b30/5949B`, with the matching transient body `1412ab270/5949B`: `1412aad27` loads builder base+0x12c; `1412aad62..1412aad72` copies16B from that base then4B at base+0x10. Thus the actual source payload is the contiguous20B builder region+0x12c..+0x140. The older comment “u64@+0x130 + u64@+0x134 + u32@+0x13c” omitted the first four bytes and is not a valid field map. Roof uses builder u32@+0xfc. Wall condition tests byte@+0x13c against2 (None niche), not arbitrary u32 converted to bool.

The full native custom-clone/draw consumers establish observed canonical WallFlagCloth fields {i32@0, f32@4/8/c, bool@10}; they do not by themselves establish the original builder methods' complete value production. Current `flag_roof_cloth()` and `flag_wall_cloth()` hardcode zero in reconstructed source and have U comments. Those zeros cannot be accepted as original behavior.

Retained source enrichment shows no standalone original flag methods. Original B inlines are retained in `builder-methods-B-retained-pdb.json`: flag_roof_cloth in generate_flags@1407ff6c0 and flag_connections::clothline@1407e2510; flag_wall_cloth twice in generate_flags. Four small PDB annotation-derived snippets were collected but their exact optimized data-flow interpretation is still provisional. `native-manifest.json` marks them as such; they are not complete function bodies or proof that zero values are correct. Further bounded host data-flow analysis is needed before changing field representation or constructor semantics.

Root may integrate the independent AlbedoMap u64 producer correction and eight-file canonical non-cloth package now. Cloth remains changes_required. Target file group is C, all native assertions here are only B; no C equivalence is claimed.

Correction20261020: yesterday's local native pack retained only3281B prefixes after following incorrect source comments. It is now marked truncated in native-manifest.json; original prefix bytes/body hashes are preserved. Correct5949B full-body provenance is supplied in Nani maintain20261020. Do not use the old prefix as a complete function.
