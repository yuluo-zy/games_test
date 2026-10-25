# Registry/pool semantic candidate

Standalone std-only recovery candidate for original Tiny Glade ResourcePool and selected ResourceRegistry operations. It is not integrated renderer code or ABI-compatible original type reconstruction.

Implemented: exact-descriptor moved LIFO pool buckets; empty-bucket retention; logical-to-physical registry access; owned/unowned texture/buffer distinctions; consuming exports with Empty replacement and ordered remaining-resource compaction.

Excluded deliberately: populate_resources, device creation, BackendError classification, managed-handle internals, image-view validation, ResourceInfo representation and downstream pool-return policy. Descriptor fields are named by observed native offsets. The generic Other payload is not claimed to be a proven acceleration-structure type.

Evidence and integration boundaries: ../../../../evidence/render-recovery/parallel/registry-pool/implementation-contract.json and findings.md.

Validation: cargo test --offline; cargo fmt --check.
