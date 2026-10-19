# B collision outline schema and display leaf

Scope: new canonical `country_core::resources::walls::decorator_outline::DecoratorCollisionOutline`, only native display leaf `140b33920/476B` and actual existing UiSignifierStream/DebugLine/DebugSphere factory interfaces. Producer/library loader is a later batch and is not ready by constructing an empty resource. Source is written only after07 design review.

Original identity: B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Complete body and per-instruction symbols are retained at TG-2cc8aec081d0811cbe/clutter-startup-20261020/validate-investigation/location-outline-native-pack/collision-outline-display.{json,asm.txt}; do not infer schema from current bbox substitution.

Native Vec-cap niche has three states: ordinary owned Vec<Vec3> (including empty), Sphere(radius at+8) when first qword is 0x8000000000000000, and no display when it is 0x8000000000000001. Proposed safe schema is `enum DecoratorCollisionOutline { Lines(Vec<Vec3>), Sphere(f32), None }`; these semantic Rust spellings do not claim original author variant names or ABI. No default is used as a loaded gate.

`display(&self, transform: &glam::Affine3A, stream: &mut UiSignifierStream, dashed: bool)`:

- Lines: allocate a new output with source point count. For each actual point evaluate `(axis_x*x + axis_y*y) + axis_z*z + translation` in the native order at140b33a30..a66, preserving the source order. Submit existing DebugLine White, line_width2, ui_pass=true, dashed=the supplied argument. Empty remains a Lines request; existing host DebugLine validation currently declines fewer than2 points, an explicit current host boundary, not a native None variant. Do not infer a bbox polyline or close an open sequence.
- Sphere: call the actual DebugSphere White/radius/translation/line_width1/solid=true factory; radius is used unchanged, with no basis-length/radius scaling. The dashed argument is ignored in this branch. DebugSphere show/factory restoration is an authorized separate companion TG-67c8f0bbd38c78935a. It must create a real drawable; current absent show and empty SignifierEntityBuilder cannot count as display.
- None: no stream append.

Library producer and consumer contract: the canonical mesh library must own actual collision outline values loaded from original assets; current four Arc<DerivedDecoratorInfo> maps are manually wrong. A later library batch changes only proven map types and constructors. Missing outline lookup remains observable under its original lookup contract, never replaced by an empty vector. The actual validation owner passes its real cached pose after constructing the affine using actual rotation, translation and scale.

No whole DebugViz rewrite, no foreign SignifierFactory/animation edits, no Cargo/GPU/production writes. Narrow CPU checks should compare transformed nontrivial point order and translation, confirm Sphere radius ignores a scaled affine, inspect genuine appended factory payload and wake against registered real mesh/shader unit fixtures. These do not establish actual App installation or a visible rendered frame.
