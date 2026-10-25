# ResourcePool / ResourceRegistry recovery

Scope: original shipped Tiny Glade only. Binary SHA-256: f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. This is static control-flow/type/ownership recovery, not a native behavioral differential or a complete renderer implementation. Evidence paths below are relative to this directory.

## Pool: exact descriptor buckets, moved LIFO resources

ResourcePool is two 48-byte std HashMaps at offsets 0 and 0x30. Texture values are Vec<Texture>, Buffer values Vec<Buffer>. Both insertion functions use the descriptor carried by the resource (payload offset 24), append to an existing bucket, or create a one-element bucket. Both take functions perform exact descriptor lookup and pop the last value; an empty bucket is retained. There is no best-fit, size-at-least, usage-superset, or registry-format-equivalence predicate in pool lookup.

TextureDesc equality uses seven u32 fields and bytes at 0x1c..0x1e, ignoring padding 0x1f. BufferDesc equality uses u64 at 0, u32 at 8 and bytes at 0xc..0xe, ignoring padding 0xf. Resource values move; ordinary take/insert does not increment the managed handle reference count. Allocation cleanup and clear drop managed handles through private RawManagedHandle::drop (0x140be24b0). This is reference retirement, not proof of immediate GPU object destruction.

Evidence: 0x140be6b90.c, 0x140be6d70.c, 0x140be71e0.c, 0x140be7320.c and registry-pool.objdump.txt. clear at 0x140be76f0 is only 27 bytes: texture-table clear call followed by buffer-table clear tailcall. The enlarged decompilation is not its true function boundary. HashMap capacity retention follows the directly reused library clear operation; library internals are not recovery targets.

## Registry: logical mapping is separate from ownership payload

ResourceRegistry is 264 bytes. It holds logical resources (4-byte ResourceReuseInfo), initial-access bytes, 64-byte RegistryResource backing values, then 192-byte ResourceInfo. ResourceReuseInfo stores a u16 physical backing index plus a reuse marker. Its unset state is index 0xffff and false; legal physical indices must be below 65536.

RegistryResource tags are 0 owned texture, 1 unowned/imported texture, 2 owned buffer, 3 unowned/imported buffer, 4 Arc-backed other resource, 5 Empty. These are recovered semantic names, not original variant spelling. resource() returns a payload reference plus initial access and reuse marker without checking the payload variant. image_from_raw_handle() returns a Texture reference (tags 0/1), not a raw VkImage; buffer() accepts tags 2/3. image_view() additionally validates device identity, resource kind and generation before calling private Texture::image_view at 0x140bdc5d0.

Evidence: 0x140be78b0.c, 0x140be7940.c, 0x140be79d0.c and 0x140be7a60.c.

## Origin ownership and population

ResourceOrigin is 72 bytes. Tags: 0 Create, 1 ImportUnowned, 2 ImportOwned, 3 consumed Empty. Texture import_unowned (0x140bd60a0) and Buffer import_unowned (0x140bd4fc0) increment managed-handle references before constructing tag 1. Texture import_owned (0x140bd6220) moves without increment before constructing tag 2. Thus unowned means external ownership policy; it still carries a managed reference that must eventually be released.

populate_resources (0x140be7ba0) asserts its three output vectors are initially empty, resizes logical/access storage to origin count, and consumes origins by replacing them with Empty. It processes acquire events and skips release events (bit 16 set, 0x140be7eb7..0x140be7ebc). ResourceInfo usage replaces descriptor usage before pool lookup/device creation; zero usage emits Empty. Pool hit and newly created resources both receive debug names. Device creation has no initial data. On success origins are cleared and false is returned; the compiler accepts this false result. Creation error discriminants 0..8 are dropped and return true; 9/10 format a panic. BackendError variant identities remain unresolved.

Important negative result: local descriptor-to-ResourceReuseInfo maps start empty and are only removed/searched in the observed function. No release-event insertion exists. The texture compatibility/format-info fallback is present, but no candidate becomes reachable through the observed normal empty-map path. Do not implement or report graph-local aliasing as proven behavior. Persistent ResourcePool reuse is proven independently.

The event encoding (pass << 17) | resource with release bit 16 is cross-checked against the sibling ResourceInfo recovery; the local branch polarity is independently present in disassembly.

## Export consumption

into_exports (0x140be9040) iterates ResourceInfo's exported logical u16 handles. Its adapter (0x1417e78e0, wrapped by std HashMap::from_iter at 0x1418c2b60) resolves each logical handle to a physical slot, moves the RegistryResource into a HashMap<u16, RegistryResource>, and sets the vacated slot to Empty. It then retains nonempty backing values in original order. The 72-byte output is remaining Vec at offset 0 plus exported HashMap at offset 24. Logical/access vectors and ResourceInfo are dropped. It does not simply drop all resources or export resource metadata.

No claim is made that this selected cluster returns remaining resources to ResourcePool; the downstream retirement caller is outside the recovered contract.

## Candidate and validation boundary

The candidate crate at reconstruction/render/work-packages/registry-pool implements exact-key moved LIFO pools, registry access mapping, owned/unowned payload distinctions and consuming exports using std directly. Descriptor names are offset-based placeholders; no original ABI layout is promised. It deliberately excludes populate_resources, managed-device validation/image views, Vulkan creation, ResourceInfo internals and downstream retirement. These need the linked contracts and unresolved types before integration. It is a semantic candidate, not byte-exact original source.

See implementation-contract.json, ownership-state-machine.json, unknowns.json and type-ledger.json. Retained MCP reports and per-function C pseudocode record the original analysis database evidence.
