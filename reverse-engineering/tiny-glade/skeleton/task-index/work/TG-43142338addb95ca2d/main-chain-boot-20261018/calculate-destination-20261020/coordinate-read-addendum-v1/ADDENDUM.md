# Preserve the existing raw location dispatch before reading coordinates

Append-only supplement to destination designde392573 / manifest5399cf63. No source changes, new claim or production write. Root's existing TG431 owner delegation is unchanged.

The original location8143B independently dispatches tags3/4 to Valid before resource reads, tag2 to roof and every other raw tag (including5+) to wall. Its existing Rust DecoratorInfo owner is a raw-field record, capable of representing those bytes. Migrating that caller through a full DecoratorDst decoder that rejects tag5 before its actual wall branch would introduce a new failure boundary. The destination enum must not gain a fabricated sixth variant to hide this problem.

Add one necessary tag-independent canonical read accessor in the same DecoratorInfo owner:

```
pub fn wall_space_coord(&self) -> utils::wall_space::WallSpaceCoord
```

It reads only existing first-qword low/high32 as f32 bits and returns WallSpaceCoord::new_unchecked(u,y). It does not assert a tag, resolve a wall, infer WallId, validate/clamp floats, query resources, clone an owner or read any other payload. The caller first follows its genuine original dispatch, then selects this exact two-f32 prefix. This is a typed coordinate read, not an opaque union and not permission to reclassify the whole record.

Location owner01 uses this accessor on its actual wall/default branch, retaining raw5+ behavior. It continues to return Valid for3/4 before reading coordinates/resources. Its roof branch is exactly tag2 and can use the canonical decoded RoofDecoratorDst.coord, whose Y is the first-qword high f32. The new singular proposed subtype can use the same accessor where its original wall branch reads the coordinate. The existing same-systems wall_space_coord_of helper can delegate to this canonical read instead of copying the bit codec again.

Full `destination()` remains the strict decoder for the native valid five-case destination value. This accessor neither weakens it nor changes setters. No canonical WallId can be manufactured from those coordinate bits; real owner is the separate DecoratorStorageAddr. The complete parent intent's four-case candidate tags and03's separate24B creation context remain distinct.

Source review must ensure location's dispatch order and exact resource-failure boundary remain original when it adopts this API. Focused getter verification should include a raw tag5 coordinate with distinct finite u/y and demonstrate that no destination tag assertion or owner lookup occurs. The full actual location tests then establish whether its original wall path accepts or raises its own genuine downstream error; getter success alone is not full location acceptance. No tests are run in this addendum.
