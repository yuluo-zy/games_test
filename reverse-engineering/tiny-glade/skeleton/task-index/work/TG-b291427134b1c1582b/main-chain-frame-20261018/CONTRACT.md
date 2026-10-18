# Original synchronization mask versus content dependency

Version B only. `native-identity.json` binds full 631-byte original add_resource_access instances and the complete 3623-byte original blur_pyramid body to RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Fresh decompilation is retained separately; ICF names are not treated as real type ownership.

The original ledger stores twelve-byte `(resource:u32, sync_group:u32, access:u8, padding)` records. New read access 19/1a followed by 37 explicitly upgrades the previous record and appends a new record. These real read→write sequences still consume previous contents.

The earlier source reconstruction additionally inferred `reads_previous=true` from every broad Vulkan 36/37 synchronization mask. Original `blur_pyramid` disproves this default: it creates `partial blur pyramid`, then its first blur0 writes mip 0 with code 37 at native 1409ee4a4. The original packaged blur shader reads its input image and has exactly an OpImageWrite through `output_tex`; it does not read output contents. The source-host graph consequently rejected a valid original first write before GPU submission.

The bounded fix removes that inferred content dependency only from legacy raw ledger projections and raw ranged declarations. It retains original synchronization masks, sync groups, declaration order and explicit read→write upgrades. Typed content-preserving writes continue to carry `reads_previous=true`; explicit discard and attachment LOAD checks retain their existing behavior. No clear pass, initialized-resource fiction or shader substitute is added.

`cargo test -p rhapsody --test resource_contracts --locked --offline` passed all 21 tests, including original first-write production/consumption, typed RMW rejection on undefined contents, true read→write upgrades and exact initialized subranges. Strict original loading GPU execution is the independent integration check, still pending as this contract is written.
