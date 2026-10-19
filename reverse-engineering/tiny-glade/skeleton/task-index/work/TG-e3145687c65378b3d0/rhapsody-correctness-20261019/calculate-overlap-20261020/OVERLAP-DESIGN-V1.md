# Shared actual calculate/merge overlap, one canonical bucket and View

Root approved TG-e314's necessary calculate/merge scope, then live show was NULL and normal claim succeeded for rhapsody-correctness-20261019. Only this existing source's real wall/roof overlap consumers and tests are in scope. No physics World, mesh registry or duplicate geometric engine is created. Original B/PDB identity remains f507a6ce/2d9686b8. Full wall parent848B + two callbacks496/478B, merge800B + two callbacks550/532B, existing parent roof726B/callback863B, and rectangle305B/29B define the boundary. All original functions are bounded; no target execution occurs.

## Wall: single geometry kernel with actual exclusions and WallSpace

Keep the existing generic intersect_wall_decorators bucket traversal. Add the actual callable input surface used by parent and merge: actual Decorators borrow, WallSpace, WallIntersectCandidate{coord,width,height}, canonical DerivedDecoratorInfoView and explicit exclusions{current_id:DecoratorId, merge_target:Option<DecoratorId>, excluded:&[u32]}. Parent supplies only its actual current ID (merge_target=None/excluded=[]), matching original496/478B. Merge supplies the genuine selected target and caller slice, matching550/532B. This is a reconstruction of the original closure environments, not a new storage owner.

For each real bucket entry:

1. Exclude current ID first; merge then excludes selected target and scans its real excluded slice. These checks occur **before** destination, subtype or mesh reads. Do not allocate a concatenated list or prefetch dependencies for excluded entries.
2. Native side classification is `(tag-2)u8<3 ? tag-1 :0`: wall/default side0 continues; roof side1 returns false; terrain side2 and stair side3 panic. Raw tags5+ take original wall/default branch, so use the single Info.wall_space_coord getter rather than strict destination decoding on that branch.
3. Require exact existing ID subtype from View and exact subtype+Info.rank collision key on the same actual mesh library. Missing ID or mesh remains failure. Read actual collision width and vertical max-min. No old cached-ID subtype is used for the parent candidate's separate proposed-position derive.
4. dy=abs(candidateY-existingY). For type!=3a, dy>existingHeight*.3 returns false; unordered does not take this early exit. Then margin=.6/verticalExtra=0. For3a use margin=.2/verticalExtra=1 without that early exit.
5. Require ordered `(existingHeight+verticalExtra)*.5+candidateHeight*.5 > dy`. Then dx is ordinary abs difference for an open wall and **actual CircularComparativeSpace distance** for a loop, using the supplied real length. Require ordered `candidateWidth*.5+(existingWidth+margin)*.5 > dx`. Equality and unordered fail at the same comparisons as native. Do not use wrapped distance for proposal ranking, which is a separate direct-coordinate calculation.

Refactor the existing wall_decorator_intersects numerical test through one private geometry kernel accepting the selected dx policy. Preserve its existing public abs-only helper signature for its actual local tests; do not make that helper the loop gameplay entry. The new actual entry uses the same kernel and canonical View. Existing legacy ID/subtype wrapper can delegate through a View constructor where behavior is unchanged. Unrelated region/snap projection helpers are not redefined by this package. The owned parent/merge migrations call the real new surface in one declared integration; a correct leaf does not make their current stubs executable.

## Roof: actual source cache and original expanded rectangle

Restore one actual bucket overlap function in collision_detection, receiving actual current ID/type, candidate quaternion/position/width/height, bucket, same canonical View, same CachedDecoratorTransforms and actual mutable hit vector. Preserve append behavior; original does not clear a caller's vector. The currently separate two-Rectangle3d helper in roof merge is an approximation with only local tests and is removed with that owner's reviewed migration, rather than retained as a second gameplay algorithm.

Construct the candidate before iterating, as original726B does: center=actual input world position, size=(candidateWidth,candidateHeight,.1), axis=normalized XZ of actual quaternion*Vec3::X. Use the locked current glam0.29.3 quaternion multiplication; its full quaternion/vector formula matches the observed multiplication and does not justify a new math implementation, quaternion normalization or scale application. Preserve the actual projected-axis normalization and required Rectangle3d constructor failure.

For each existing entry, skip actual current ID first. Native four-entry side table142fcbe30 is frozen: wall/default returns false; roof and stair enter the collision path; terrain panics. For accepted sides, compare real type family `(ty-38)u8<8 ? index :6`, additionally exact type for family6. Mismatched family returns before reading cache/mesh. Require actual CachedDecoratorTransforms.transform(existingID) **before** View subtype and exact rank collision lookup. Missing actual cache/mesh remains visible; no identity/default transform or new registry.

Build the existing rectangle exactly:

- width/height are actual collision width and vertical max-min.
- rotatedOffset=actual cached quaternion * Vec3(0,0,f32::from_bits(0x3dcac084)); this is original constant0.0990000069, not an invented unit/depth offset.
- center=actual translation + (Vec3::Y*height)*.5 - rotatedOffset. Retain zero*height lanes rather than silently assuming all collision bounds finite.
- size.x=width+f32::from_bits(0x3ecccccd) (.4), size.y=height+0, size.z=.3. Native rectangle depth is independent from its fixed offset.
- axis=normalized XZ of the same cached quaternion*Vec3::X. Ignore cached scale exactly as original does.

Append actual ID only on original Rectangle3d overlap. Current canonical Rectangle3d constructor matches native305B's two horizontal size checks and unit-axis check; it does not require finite height. **Current utility overlap has a concrete blocker:** `!(top<other.bottom || other.top<bottom)` admits unordered vertical values, whereas complete native29B rejects unordered through ucomiss jb/jae. Correct shared utility contract is ordered `top>=other.bottom && other.top>=bottom` then existing2D overlap, touching included. This specific utility delta is reported to root for its legal owner; this package does not write that foreign source or copy another overlap implementation. New roof gameplay entry requires that dependency be integrated or must remain unaccepted. No global finite-height guard can replace it.

The original callback has no arbitrary excluded slice. Do not import wall merge exclusions into roof overlap, cull existing entries by rank ahead of their actual geometry path, or apply a family based on the old min6 bug. Parent calls this real hit collector first and then its separately recovered roof selector; selector itself does not introduce another overlap stage.

## Actual callers, ownership and tests

Full workspace inventory currently shows only the owned approximate wall merge and parent plus local tests. Existing parent calls an empty-target legacy wall selector and has no real roof overlap; migrate both through the reviewed full calculate input/value contract, not a fabricated resource/system. All buckets, cache, View and actual StorageAddr remain same-World owner borrows; no source-map clone/default or automatic flush is introduced.

Meaningful tests after source review: actual bucket/View with loop seam versus open wall; strict boundaries and NaNs; current/target/slice entries with deliberately absent mesh prove lazy exclusions; roof skip and terrain/stair branches; missing required ID/cache/mesh failure order; original family3f; nonempty preexisting hit vector append; actual cached rotation/translation changes the selected source; original .099 offset/.4 margin/.1 versus.3 depth counterexamples; exact touching and NaN vertical utility behavior; zero projected axis constructor failure; ignored actual scale. Source tests and root execution remain separate from original evidence verification. No test or Cargo has run here.
