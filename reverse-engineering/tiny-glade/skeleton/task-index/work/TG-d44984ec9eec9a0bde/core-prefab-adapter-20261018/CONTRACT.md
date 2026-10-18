# B prefab hot-reload full placement consumer

Scope: correct the current manual hot_reload_prefabs input/forwarding; retain its filtering, despawn/rebuild, RT/audio algorithms and ownership. No separate native-layout module, duplicated placement state or full prefab reconstruction.

## Original identity and behavior

B release RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 / PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Indexed hot_reload_prefabs140843740 size3948, body ae7d19874746fc2dc9b29524675aac83ff2213d3b31c19d09163c32fcebba597. create_prefab_gpu_instances140841b00 size602 body18cdc2a6241a94683134cee1d4b517098fdcddd7d4ab9a8f1199d2b48338c241. Both raw ranges saved from original RNE and match indexed hashes; retained decompile is navigation, original instructions are decisive.

140843ea5..eeb resolves a supplied query row and selects index*48 into the real placement array.14084415c..193 forwards that same pointer as the fifth argument; no zero/default construction.140841c3b..cc2 copies its first three floats into cull XYZ and supplies the fourth scale float to half conversion, then forwards the same placement pointer to GpuWorld::create_instances.

64 original resolved-row selection/forward cases plus64 original callee cases compare every input/output bit, including non-transform payload, signed zero and arbitrary NaNs. Names/atlas/query resolution and half conversion are dependencies: name is bridged; the half stub only exposes the original scale input and its resolved output is not accepted. This does not accept full query discovery, filters, GPU execution, allocation or C parity. The first harness attempt entered7 bytes late without loading R8 and failed unmapped read; corrected to the full original load entry before recording accepted fixtures.

## Current bug and incremental fix

The manual helper took only PrefabTransform and manufactured zero/default placement and cull values, losing id/custom/animation/bend fields on reload and culling at the origin. The concurrent compile adapter changed the obsolete x/y literal to a full default record; that resolved compilation but still lost runtime input. Root preserves that independent type recovery and all other consumers.

The helper now takes the existing PlacementDataSsbo in the same three-element tuple, derives the existing local PrefabTransform for RT/audio, sets cull from actual position/scale, and passes the complete existing record to the existing GPU API. Its sole workspace caller is adapted; no production birth/default assumption is added. A real GPU shadow-buffer regression compares all48 bytes plus cull XYZ/radius and RT translation.

Original draw/query scheduling, missing-prefab policy, full prefab creation/destruction lifecycle and other fields/algorithms are not newly accepted here. Full host does not currently call this pure helper.

Status: integrated_unverified. Root unified Cargo required.
