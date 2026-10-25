# Integration

Apply root.patch (3existing resources sources) only after shared owners accept; do not replace unrelated handwritten modules. Base/candidate/patch mapping is source-mapping.json. Shared history serializer work may touch the same file: use method/alias/test hunks, preserving history_audit's new serde.

- owned decorator_storage.rs.patch: canonical buckets + take/restore APIs +3tests. Root may integrate the source after independent review.
- necessary decorator_edit.rs.patch: existing to_info native type dispatch + aliases +1new test/fixture correction. Coordinate current owner, not a parallel module.
- main-chain-owned decorator.rs.patch: canonical Event+drain/system+5World/type tests. Hand to current source owner.
- external-owner.patch: only existing upper wrapper alias and its .0 call-site removal; parent/owner applies after API review.

Host glue: import DecoratorOpEvent and unleash_decorator_op_events from core resources::walls::decorator; retain init_resource::<DeferredDecoratorOpEvents>(); add_event::<DecoratorOpEvent>(); add actual system after all registered deferred producers and before relevant consumers. This is a required owner hunk, not a claim of integrated runtime.

Save owner: SnapshotDomainData.decorators API remains Vec<(addr,DecoratorAddrSnapshot)>. Use storage.take_snapshot().states directly. Wire old JSON conversion to canonical buckets and restore_snapshot_with_id_gen(snapshot,&mut actual_generator). history_audit already has exact bucket API and State/Addr serde original packs.

Test filters after integration: snapshot_collects_nonempty / snapshot_distinguishes / snapshot_restores_real / to_info_type_selects / to_info_roundtrips / canonical_decorator_event_tests. Root runs Cargo centrally; this agent did not run them.
