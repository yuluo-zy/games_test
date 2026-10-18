# Root integration

Apply owned-dormers.patch then integration-owner.patch to current compatible bases; source-map.json identifies exact preimages. Shared mod.rs only adds pub mod dormers; shared ssbo_tags only replaces the two existing zero-sized dormer tags with canonical reexports+DrawSsbo impls. Preserve parallel Stairs/Lantern/Grounded edits in those files when adapting the shared hunk. Production/Cargo were not edited here.

The existing startup producer must use DormerHoleSsbo and DormerHoleCountPerCellSsbo from resources::walls::dormers. Their factories already preserve None/count0 and Some/countrows; do not use draw-only tags to construct unrelated storage. No render::ssbo global policy change is needed.

Run resources::walls::dormers::tests (4), draw_shaders::ssbo_tags hash checks, and existing instanced_roof tests; then the parent producer orchestration checks. Actual roof reader test binds actual canonical library leases while unrelated host resources stay in the current CPU Rig. It is not a GPU/render-completion test.
