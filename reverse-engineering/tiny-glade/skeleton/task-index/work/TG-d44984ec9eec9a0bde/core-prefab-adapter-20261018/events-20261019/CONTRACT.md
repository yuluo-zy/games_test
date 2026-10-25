# B canonical prefab event carrier, incremental recovery

Target is original B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Parent/root resumes its existing core-prefab-adapter owner. Tree producer is separately legitimately claimed TG-3add2880f5e9bb64c3/core-tree-prefab-events-20261019. Native collector report ok, 3 full PDB functions, 10233 byte instructions validated against file; no native execution/DB mutation/Cargo. Original PDB source metadata is not recovered author text.

## Observed behavior

Spawn reader 140840610/3903B steps native EventInstance by80B. PrefabName begins row+0, entity is row+16. At140840bfd, LEA RSI,[RDI+24] selects the very same PlacementData48;140840c0d passes its address as fifth argument into create_prefab_gpu_instances@140841b00. Prior reviewed create-prefab-gpu-B.asm in parent directory independently anchors full placement48 forwarded to GPU. This is name16 + Entity8 + placement48 =72B payload. This EventInstance places EventId after the payload, not before it.

Tree process142267430/4758B:142267e55..eac constructs the same name/entity/position/scale/rotation/id/custom0 and remaining words; actual writer142267ed8..f2d writes80B rows, last qword EventId. Payload+48 custom0 is 0x3f800000; +52,+56,+60,+64 are zero; +68 is branch-selected i32 currently named mode. These are exactly PlacementDataSsbo::{custom0,custom1,anim_t_packed,bend_vector,pad0}. No lossy float/int conversion: custom0 stores f32.to_bits, variant id is i32. This change does not fix remaining drag/mode producer gaps.

Despawn reader140842d60/1572B steps16B;140843104 reads row+8 Entity, then uses its low32 index and high32 generation (140843108..11b). Native EventId is the other8B. Therefore former _gap is an event header wrongly added to payload; canonical payload is only Entity8. Per-type EventInstance ordering is not assumed universal.

Forest independent existing full native packs support complete placement mutation of custom0/packed anim, so the carrier must preserve all48B. Forest source families belong boot-leaf-20261018 and are not overwritten; its full7-file source candidate already constructs {prefab_name:Some(..),entity:Some(..),placement}. Root coordinates its integration separately.

## Current Rust adaptation

Retain the sole existing Event types, names, Event derives, optional name/entity owned API and all prefab lifetime/RT/GPU/audio/hot-reload logic. Replace the old position/scale/rotation/variant/one/flags/mode projection with the already authoritative PlacementDataSsbo. Despawn removes only _gap. repr(C)/transparent permit bounded72/8 checks, never casting native event storage. Option values are current Rust representation, not an inferred native None spawn branch. Legacy Default remains an empty owned request solely to avoid breaking the pending forest owner migration; it cannot demonstrate actual native birth.

Tree-tool actual producer migrates its exact words into that one placement record. The compatibility Default and existing core helper are not claimed as a full real prefab EventReader/Commands registration chain. Existing spawn_prefabs pure helper/RT deferred command seam remains separately unresolved.

## Verification boundary

New regression moves an actual Events<SpawnPrefabCmd> through Bevy buffer update and cursor read/clone, then compares all48 original bytes including signed zero, NaN payloads, custom fields, packed animation, bend and pad; separately sizes payloads72/8. This is candidate Rust behavior, not original process acceptance. No Cargo run yet. All existing prefab helper tests and tree lifecycle tests remain required after root integration.
