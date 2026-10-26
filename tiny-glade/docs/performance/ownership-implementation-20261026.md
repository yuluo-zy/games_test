# Ownership changes and validation (2026-10-26)

User directed source work to continue while the production-window compatibility blocker is deferred.

- VAO publication moves the previous table out of the assets mutex before releasing its owners.
- Dynamic constant allocation ends its allocator guard before owned chunk writes and Arc construction; allocation failures retain the original error path.
- Explicit texture views are grouped by the full texture identity. Release removes the group and drops native owners outside all host-table locks; other textures and external Arc owners survive.

Read-only follow-up review by the previously requested ownership audit agent found no new lifetime blocker. Grouping adds a per-texture table and another lookup; overall performance is not yet established. Concurrent view creation versus release remains a pre-existing open race.

Validation: strict Vulkan texture-release unit tests 2/2; VAO scope 1/1; collected-vs-inline constants/depth pixel comparison 1/1, without validation errors in those targeted runs. The broader core_raster_draw suite returned 12 successes but emitted VUID-vkDestroyImage-image-01000 during synthetic DeviceLost injection; that run is invalid as a validation-layer gate. The test must finish real uploads before simulating completed/lost work and be rerun.

No Windows four-scene gain, <=16ms acceptance or macOS regression pass is claimed. Remaining ownership audit candidates require targeted evidence and separate changes.
