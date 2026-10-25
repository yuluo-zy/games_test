1. Integrate root's canonical core BushCpuSsbo constructor first: reprtransparent [u32;2], mapped storage,9601 elements, original NAME.
2. Apply owned-reader.patch and necessary integration-owner.patch against source-map.json hashes. Preserve other owners' lib changes. The lib patch preserves preinstalled AssetSsboLibrary and adds only one actual reader node to the existing graph.
3. Run system-clutter and country-core tests plus root's joined graph tests. The new reader/registration tests deliberately avoid pretending the whole unrelated clutter pipeline runs successfully; they exercise actual canonical mapped/library consumption and build the configured graph.
4. Keep the separate unused populate BushGrid and CPU/GPU completion boundaries visible. To close population exclusion, claim its source and recover its actual bushes_near_point/subscriber/filter consumers in the current architecture; do not copy this grid into the unused stand-in.

No source/registration adapter should reinitialize the library or create the mapped storage on every frame. Actual host SSBO creation and submission belong to existing root/device owners.
