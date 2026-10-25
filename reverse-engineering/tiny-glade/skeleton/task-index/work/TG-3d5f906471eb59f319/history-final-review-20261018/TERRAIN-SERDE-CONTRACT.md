# Minimal B Terrain saved-history integration

Scope: resources/history/serialized.rs only. The original history/mod.rs family
is owned by the root integration owner. This actor supplies a candidate patch
inside its independently claimed finish_replay review directory; it does not
edit production or another owner's files. The parent explicitly requested this
minimal integration hunk. Target remains B; C is not accepted by address/name.

## Concrete failure and change

The current typed PendingTerrainHistory records EditType::Terrain(Arc<TerrainEdit>)
and the existing terrain replay consumer reads that same schema. The saved-history
adapter only matches Wall, so saving a newly recorded terrain edit fails and
loading a saved Terrain edit fails its variant visitor. The candidate adds one
SavedEdit::Terrain arm, rebuilds the same Arc owner on decode, and serializes the
existing payload as EditType index0/nameTerrain. Existing wall code and transaction
structure are preserved. Existing tg_terrain::schema::TerrainEdit derives implement
the three inner variants; no parser, native ABI or parallel schema is recreated.

## Original observations

- B release RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
  terrain-history-pdb-procedures.json binds the original EditType serializer at
  0x140316660 (2813 bytes), deserialize at0x140b3ad60 (2246 bytes) and variant
  visitor at0x140b3b630 (803 bytes) to matching B PDB public aliases and source.
- 0x140316681..0x1403166ef serializes external nameTerrain then reads the Arc
  payload at+16 and calls TerrainEdit serializer0x1404f41b0. Native execution of
  all three inner variants yields the retained objects in
  terrain-history-native-fixtures.json. NewTerrainStroke uses index/stroke_pts/
  height_ws, EditTerrainStroke uses index/prev_height_ws/new_height_ws/
  prev_falloff/new_falloff, MoveTerrainStroke uses id/original_stroke_pts/move_vec.
- Original visitor0x140b3b77e..0x140b3b79c compares the seven bytesTerrain and
  returns success0/variant0. An independent direct native invocation passed.
  Deserialize's tag0 arm0x140b3afbe..0x140b3afde dispatches its newtype payload
  and selects outer tag0 on success. The full original JSON parser was not run.
- terrain-serde-byte-identity.json records retained raw instruction hashes and
  byte equality with the patched B analysis input. Original schema/field evidence
  also exists at evidence/terrain-editing/B-history-schema.json. The existing
  schema is reused rather than translated from its niche memory representation.

## Validation and limits

The new meaningful regression feeds the three original JSON outputs through
SavedMultiEdit's actual JSON decode and encode, checks all become Terrain and
preserves the whole JSON values on reloading. No compilation/test run was made
by this actor; root performs it after applying terrain-serialized-only.patch.
The native oracle completed3 serializer cases and1 visitor call under approved
read-only Unicorn JIT execution. Allocator/CRT bridges are supplied; serializer
business and Serde code execute from the release binary.

JSON object-key order differs for NewTerrainStroke in the existing Rust derive
(index,height_ws,stroke_pts versus original index,stroke_pts,height_ws). Native
consumer semantics and the regression compare JSON values, not textual ordering.
The adapter remains a JSON saved-history adapter, not acceptance of numeric enum
indices in arbitrary binary Serde formats. Unsupported other EditType variants,
wall operations and MultiEdit auxiliary owner serialization still fail explicitly.
No full native save-file reload, replay schedule or scene/GPU acceptance is claimed.

Fresh B metadata succeeded but the bounded4-function get_functions query failed
with isError=true/Corrupted BufferMgr state. The failed tool result is retained
in terrain-serde-fresh-mcp.json and is not used as positive evidence; no shared
database restart or mutation was attempted. Raw RNE, PDB and original execution
provide the observations above.
