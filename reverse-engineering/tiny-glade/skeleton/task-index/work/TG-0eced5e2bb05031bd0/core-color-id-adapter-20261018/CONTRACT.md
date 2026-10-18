# B complete WallId in color commands — bounded incremental repair

Root claims four original filegroups under core-color-id-adapter-20261018: SetWallColor TG-0eced5e2bb05031bd0, SetHalftimber TG-fc3db24ab55f60184b, SetGate TG-41114c79b8d5600ddb, replay_color TG-8608a7dd158bc5f100. Each has base/source hash/candidate/patch. Shared resources/color_id.rs has no independent original task; integration-color-id.patch is a necessary root integration hunk, retaining existing split public fields.

Target explicitly B, regardless of task queue's default C target label. Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Native pack status ok5: complete4B identity leaf1400885d0, bounded original3color readers and aligned draw writer140a7e3c2..3e8. No databases mutated; no native execution or Cargo compile here.

## Original facts

- Native draw writer140a7e3ce stores a qword wall ID at EventInstance+8, then coloru32+10/custombyte+14 (hex). It is not an idu32 followed by an unknown independent field.
- Reader pointers at142290afd (wall),14229948d (half),142299aa8 (gate) address event+8, passed to1400885d0. Leaf's entire game-relevant body is exactly `48 8b 01 c3` = mov rax,[rcx]; ret. It returns the complete64-bit identity without masking, truncating, extracting generation or other index conversion. LLVM selects an unrelated ICF public alias cmsPipelineGetPtrToFirstStage; that alias is not evidence of business semantics. The direct caller and actual four instructions establish this contract.
- Wall142290b0c and Half1422994a3 compare full RAX against0x5000 before indexing the u32 tables. They reject IDs >=20480 even when low32 alone would be a valid slot. Wall later PublicWalls.get passes the same complete qword142290b52..61; plaster routing retains it.
- Gate142299a89..98 first calls actual PublicWalls.get140a91ef0 with complete qword, skips missing wall, and only on a hit performs full-ID bound142299ab1. Preserving this order matters: an absent high-ID wall does not become an out-of-range table access or a write to another low-ID wall.
- Existing replay_color already writes low32 and high32 correctly for Wall/Gate/Halftimber and setscustom1. Production prefix is byte-identical to base; only cfg(test) regression is appended. No replay dispatch/timing/easing algorithm is rewritten.

## Minimal current API changes

Existing structs remain repr(C) and preserve id:u32, _gap4:u32, color:u32, custom:u8, _pad[3]. _gap4 is now documented as WallId high32. Each gets const wall_id()->current resources::walls::WallId and const new(WallId,color,custom). This adds no second ID owner or native ABI struct.

Three current reader bodies reconstruct one WallId, compare its u64 against MAX_NUMBER_OF_WALLS as u64 before any usize cast, and reuse the same WallId for PublicWalls and plaster requests. Gate's PublicWalls miss still returns before bound. The decorative/window command families remain their actual distinctu32 IDs and are not changed.

Root's wall producer sets high32 from wall_id>>32 (or may use the new constructor) in its separately owned draw module. Other current consumers/producers can continue constructing the preserved split fields; default high0 retains the previous low-ID API behavior.

## Tests and boundaries

Added real World/Schedule tests cover: Wall and Half high-bit ID reject before touching existinglow7; Gate full-ID miss while publiclow7 exists does not recolor it; Gate full-ID hit reaches original bounds; low-ID behavior continues through realPublicWalls publication and plaster event routing; actual replay system preserves all64 bits for three targets and bothundo directions. Core API tests exercise high-word roundtrip and retained offsets/16B payload. All tests use current real owned records/resources; no GPU success is simulated.

All candidates rustfmt-parse. All4source patches plus shared API hunk pass git apply --check against recorded bases. Root compiles/runs actual tests after independent review. No production/Cargo/target files edited by this worker. No new native probes attempted because the prior automatic approval failure due account usage remains unresolved; no bypass or claimed native execution.

Scope is complete WallId contract only. Existing SSBO upload/renderer gaps, UI audio payload gaps and other color-domain algorithms are not accepted or rebuilt. Separately reported existing CPU registration issue: add_color_cpu_systems still installs init_color_ids_buffer everyframe, whose insert_resource recreates three tables. This ID patch does not alter that unrelated initialization owner; root may fix necessary shared startup registration separately.
