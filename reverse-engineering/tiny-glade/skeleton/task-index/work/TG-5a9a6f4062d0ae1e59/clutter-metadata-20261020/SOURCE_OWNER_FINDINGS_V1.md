# Source owner / Convertable: bounded findings V1

This freezes additional read-only original-B evidence after PROCESS_OWNER_CONTRACT_V1. It is not a source candidate or schema approval. No production, Cargo, CPU test or GPU execution. The prior manifest/code remains unchanged.

## Exact ownership correction

Original source tag4 contains **Arc<[glam::Vec3]>**, not a mesh address, generated key or an integer reference count. PDB-bound Arc::drop_slow140b5d130/53B identifies that exact slice type. ComponentDescriptor::drop_ptr<AutoClutterConvertableToDecoClutter>14014b3b0/25B checks tag4 only, decrements the strong count at source+8, and passes the fat pointer beginning at+8 to drop_slow. drop_slow reads its metadata length at+10 and alloc layout12*len plus Arc header/alignment. Thus the qword+10 copied fully for tag4 is the **slice length**, not an independent placement key. Safe Rust must clone and drop an actual Arc<[Vec3]> supplied by a real producer. Raw legacy shared/key integers cannot manufacture it.

For native tag5, the original clone path does not read+8 or+10 payload; unused register/padding residues are not fields. Tags0..3 copy qword+8 and u32+10; their business identity names require actual writer evidence, not names inferred from generic source tags. The full gates2910B and around_windows4303B writers are now retained, along with preplaced2180B. Earlier source comments' smaller procedure sizes are stale. Original gates source prefix is tag3 with the actual WoodenGate wall u64 and its u32 field; around-window retained paths write tag0 and the actual wall/decorator IDs. Preplaced's retained prefix writer141ce5d8f is tag5. This evidence does not invent additional active tags/Default.

## Convertable constructor is a structured value, but source context decoding is still open

Complete original manager1422d04e0/10301B at1422d1c01..1d37 constructs the96B component before native EntityCommands::insert1422cba20/663B:

- component+00..17: cloned source union (including actual Arc slice for tag4).
- component+18..2f: output source context copied exactly from spawn+18..2f (three qwords; meaning and scalar lanes still need final consumer matching).
- component+30..3f: actual selected ClutterMesh's borrowed mesh_name string slice, native mesh+20/+28.
- component+40..5f: selected mesh Rectangle3d bounds, native mesh+00..1f.

Only the source tag4 owns a dropped payload. The string slice is borrowed from the actual ClutterMeshes owner, not cloned String/native allocation bytes. The current canonical ClutterMesh already exposes mesh_name:&'static str and bounds:Rectangle3d. Current ResolvedClutterMesh omits those two values because the accepted Window path never needs Convertable. A later region extension must preserve the actual loader name/bounds, not reconstruct a name from its hash or use a dummy rectangle.

Original ui_convert_autoclutter_to_decoclutter14099ca60/8136B is retained completely as the real consumer. It is a Bevy query over actual AutoClutter/Convertable/Transform, with further UI/history dependencies. Existing core ui_autoclutter source instead uses host-adapter child/index tables; this is not evidence that a new Convertable component would already have a mounted actual UI consumer. No schema with an opaque96B blob, raw unknown owning pointer, mirror component or fabricated Default is proposed. Full context lanes and original UI read/constructor agreement must be frozen before asking root to add a unique canonical value.

## Region construction gates

In complete populate44132B,1422d8f43..8ffa writes the real prefix before the final collision/region admission checks: selected ClutterMesh pointer, generated GlobalId, all three position floats, direction floats, scale1.0, actual source context, and selector **3 or6** (8f90..fa0). Those selectors pass the original manager Convertable gate `(selector & 6) != 4 && selector > 2`. The accepted Window0..2 code therefore cannot be reused with a new marker or allow-all guard to bypass this dependency.

The source+90 priority is populated from the actual region payload's+34 field (cursor-0xc). The current region value does not retain its real owning source prefix; current populate has larger admitted source-level approximations. Consequently no successful region-to-Nani producer is declared on the strength of these field transfers. The next candidate must recover an actual upstream route, not attach metadata to that approximation and claim completion.

## Current deletion integration boundary refinement

The PROCESS V1 source inventory correctly identifies the single DeletedAutoClutter and existing RMB/edit/history replay mutations. Actual run_game additionally calls history_runtime::install, but history_runtime.rs is absent from this checkout. Therefore the precise current boundary is **an unavailable installer implementation and no inspected actual mount for the deletion/replay set**, not proof that all indirect mounting is absent. Do not synthesize an empty set around that missing dependency or change reset/history semantics. This addendum makes that limit explicit without changing V1's frozen source facts.

Seven additional complete original bodies are in source-owner-native with collector status ok. Their native input identity and PDB boundaries are retained. No original target or test was executed, and no source-equality claim about C is made.
