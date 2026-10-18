# Original B MEI layout consumer

Identity: original RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Patched Ghidra tiny-glade.exe revision 1791719479858:b17d372d:3 is only a decompiler supplement; full original instruction files are the oracle.

Source ownership: original mei/layout.rs TG-19924eac9946ff87be; core.rs TG-70a753eeccd8998d55; config.rs TG-1ab2df0ca206ff9918. Production is frozen for parent GPU validation; candidates remain here.

Observed:

- pre_update 14068548e clears MeiContext+10, the frame-events Vec length. 1406854f1 clears the current-node RawTable at cached self+1a0 after swapping with previous+200. Previous nodes must survive to be reused.
- post_update 1406884c0 starts roots from frame-events (56-byte entries). Initial z=1. Stack reverses roots then visits LIFO; children append in forward order. Effective z = incoming z + node local z. Ignore-input bit 2 propagates.
- Reverse traversal calls compute_min_size_with_children 140fe0e10. Forward traversal 140689091..140689b60 resolves frame variants and calls place_childen, then copies computed child layouts into nodes.
- ComputedLayout is size first, origin second. Style offsets 28/30 are width and height, not size/min. Child-layout +8 pointer refers to content intrinsic dimensions, not padding.
- Min size: absolute gap times saturated child-count-minus-one, sum main sizes / max cross sizes, absolute padding, own absolute width/height and absolute content dimensions. Percent values contribute zero at this stage.
- Child placement: gap based on padded main dimension; available excludes gaps; each base main dimension is max(style dimension, content intrinsic dimension, minimum child dimension). Expansion distributes positive remainder proportionally; denominator max(sum expand, f32 bits 1e3ce508). Justification reserves clamp(1-sum expand,0,1)*remainder.
- Justify tags 0 Start, 1 Center, 2 End, 3 SpaceBetween, 4 SpaceEvenly. The former 33-item jump table was an over-read into adjacent tables. XAlign tags 0 Stretch, 1 Start, 2 Center, 3 End.
- Root tags 0 fullwindow; 1 centered width min(logicalWindowWidth,1500); 2 min(width,1777.777832); 3 relative frame; 4 absolute position; 5 internal child-only. Logical height always1000 and scale physicalHeight/1000. Anchored origin = position - (anchor+1)*size/2.
- New-at-pos original 140f992e0 receives position Vec2, Option<Vec2> requested size, anchor Vec2, opaque. Its former eight-scalar Rust signature misinterpreted the Option discriminant and payload.

Dependency reuse: std HashMap/Vec, serde/ron derive/visitors and existing tg-text CPU/GPU implementation. No native allocator or Bevy ABI imitation.

Validation pending: parent cargo/check/tests after source unfreeze, current-host actual GPU UI output. Source-level tests exercise frame-root topology, both layout axes, nonzero expansion remainder, anchored root placement and cross-frame content retention; they do not prove binary equivalence or full menus.

Additional original action-state write: pre_update reconstruct/memcpy writes MainActionSet atMeiContext+30. DigitalActionState is8bytes and the first40 slots are digital, so native+c8 aliases digital19(ui_confirm), and+d0/d8/e0/e8 aliases digital20..23(ui_left/right/down/up). Source projections now mirror those full states immediately after reconstruct. This corrects prior notes claiming unknown producers; no OS event/default state is substituted.
