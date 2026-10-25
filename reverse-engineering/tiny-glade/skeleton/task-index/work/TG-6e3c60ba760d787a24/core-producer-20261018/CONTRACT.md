# B remaining canonical producers: StairsCounter, Lantern, Building Shapes

StairsCounter newtypeu32/mapped56/NAME Stairs Counter/cap1/usage22/memory4/align4. None uploads four zero bytes then maps. Some validates byte remainder4, ptr4, len<8. Map validates returned Buffer.byte_size, remainder4, ptr4 and retains actual full returned rows. Specialized factory preserves current mapped/library owner; no StairsRasterValue gameplay consumer recovery is claimed. Pass StairsCounterBuf becomes same canonical identity.

Lantern16x32768 and Grounded32x5000 remain existing structs and InstanceBuffer owners; None count0/no upload, Some count=byte length/element size. Reuse existing TypedStorageBuffer fixed-byte factory so length/pointer/capacity and descriptor behavior stay centralized. Correct existing Grounded name to Building Shapes (key1c53e91811919db1). No shader/algorithm/CPU source map rewrite.

Common MappedStorageBuffer current len check uses requested desc.size, while original Stairs/Bush use returned buffer byte_size. Its original file TG-c36fbd00afb24cafe0 is claimed by render-prelude-ssbo-20261018; root did NOT change that source. ssbo-owner-followup.patch is a separate unapplied exact one-line handoff; local Stairs factory is exact without waiting for a global edit.

Three source regressions exercise actual canonical AssetSsboLibrary None/Some descriptors, mapping, names/hash/instance counts, pass alias identity and returned-descriptor short-map panic. Not run. Native packs static only; GPU completion, actual terrain audio readback consumer and full startup are not accepted.
