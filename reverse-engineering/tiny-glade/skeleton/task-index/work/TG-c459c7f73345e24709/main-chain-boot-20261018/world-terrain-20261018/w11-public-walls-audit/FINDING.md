# W11 actual PublicWall predicate and shape encoding

Read-only original evidence for root-owned publication; W11 compute family TG-3aa474b479ac4e3b2b remains held by core-producer-20261018 and is not edited. Typed fragment is a handoff for that owner, not a second production implementation. B-only, no Cargo/GPU.

Original complete `publish_public_wall_state` @140a8b530/18515B and W11's actual collection specialization @1412fc510/1002B are retained with original RNE/body SHA. This corrects earlier inference that the filter might consume plaster or a demolition progress value.

Proof chain:
1. Native publisher's InnerWalls lookup is a bucket with stride0x78 (@140a8c6b8); @140a8cad6 sets rax=rbx+rdi (bucket end), @cada stores it at locald40. @cae1..cae5 independently sets source WallState pointer to bucket end-0x70. Thus native source is 0x70 bytes following an 8-byte key.
2. @140a8d81e reloads the same Inner bucket end fromd40. @d825 reads f32 `[r11-0x18]` into xmm4. This is **source+0x58**, already canonical `WallState::elevation()` / RawWallPublicationSource.terrain_floor. No arithmetic writes xmm4 between @d825 and @d904.
3. @140a8d904 writes xmm4 to local `[rbp+0x780]`. @dd16 sets copy source `[rbp+0x500]`; @dd06..dd1d copies0x2a0 bytes to actual HashMap insert source, @dd3d performs the named PublicWallState insertion. Therefore `PublicWallState+0x280 = original source+0x58 terrain_floor`.
4. W11 from_iter @1412fc510 has bucket stride0x2a8 (=8-byte key+0x2a0 Public state). @c5ec reads `[bucket_end-0x20]` = Public+0x280, @c5f3 UCOMISS compares it against original0.05, @c5f6 JA skips only strictly above, so NaN remains included. Type shape tag @c5ff `[bucket_end-0x48]` = Public+0x258; >1 skips.

Consequences: do not use current `building_projection()` output for W11. It leaves+0x280 zero; that would silently include elevated premade walls and be a real business error. The complete canonical PublicWallState already owns an authoritative WallState clone and shape. Existing `authority.elevation()`, `is_circle()`, `is_rectangle()` expose every needed input without new raw bytes, duplicate progress storage or cast.

Original output32B shape mapping:
- Circle: words center.x, center.y, radius, radius, 0,0,0,0. Native @c60b..c616 loads center+radius, @c684 `movsldup` duplicates the radius.
- Rectangle: center.x, center.y, size.x, size.y, axis.x, axis.y,1,0. Native @c623 reads sizepair, @c628 reads center.y; @c62e..c63b preserves axispair; @c643 places center.x; stores @c6b0..c6c6 complete the output.
The typed handoff fragment uses exactly these words and current `GroundedBuildingsSsbo`, `SKIP_ABOVE`. Freeform is filtered by actual shape methods. Any tests should include elevation exactly0.05, just above, NaN; distinct plaster values must not affect membership; distinct circle rotation must not change native duplicated-radius output; asymmetric rect axis/center/size ensures order is not accidentally symmetric.

Runtime integration still needs same actual canonical AssetSsboLibrary/GraphicsDevice/InstanceBuffer<GroundedBuildingsSsbo> producer and current graph declaration. Current compute file now has correct NAME='Building Shapes' and the native5000 capacity, so do not resurrect the old wrong Ssbo name. Root/foreign owner should replace its legacy `HashMap<K,GroundedSource>` consumption with actual PublicWalls iteration and the typed conversion in the existing function; do not install a parallel legacy World ResourceView.
