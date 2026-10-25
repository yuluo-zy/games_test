# Current WallId API correction after frozen snapshot61/testv2

Independent source-only API reviewer found that current WallId has no Serialize impl. Canonical WallState Serialize is available, but it does not add serde to the ID newtype. The private writer now serializes the already-sorted borrowed rows as (id.0, &WallState), preserving original full u64 identity and existing domain object. No global WallId or other owner is changed, and JSON schema/expected assertions are unchanged. Apply after snapshot61 and test-v2. No Cargo/native/JIT or production edit.
