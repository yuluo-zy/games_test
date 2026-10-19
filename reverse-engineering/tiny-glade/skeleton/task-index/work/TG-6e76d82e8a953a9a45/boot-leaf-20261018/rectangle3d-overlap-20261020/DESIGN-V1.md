# Two exact Rectangle seams required by calculate

Root delegated two methods of this existing boot-leaf-20261018 task, without changing its claim. Current canonical rectangle.rs SHA6353c714ae6fd18b59a420257bb695b1f61e3c562e82d3e06efd0a55bb0ab403 is the sole before source. This packet does not reopen other geometry/nearest/perimeter/constructor algorithms. No candidate or Cargo/target execution before independent design review.

## Ordered Rectangle3d vertical overlap

Original B140c98580/29B, PDB768:1328: self.top is compared to other.bottom; JB returns false, including unordered. The second comparison other.top versus self.bottom continues to real2D overlap only on JAE, excluding unordered. Equality continues, so touching is overlap. Required Rust condition is `self.top >= other.bottom && other.top >= self.bottom && self.footprint.overlaps_rectangle2d(other.footprint)`.

Current `!(self.top < other.bottom || other.top < self.bottom)` differs for NaN: two ordinary overlapping footprints and any one NaN vertical bound return true. This is reachable by the public real constructor: complete original140c98380/305B, PDB768:252 checks only size.x and size.z finite plus unit XZ axis; it deliberately does not check finite size.y. Current constructor agrees. Therefore the domain cannot be excluded by an invented finite-height gate, and no constructor change is proposed.

Only replace this one predicate with ordered comparisons. Keep original comparison order, inclusive touching and existing2D overlap implementation. Actual tests should construct one NaN-height rectangle through unchanged Rectangle3d::new, check each of the four individual NaN bound positions and both argument orders, then retain finite overlap/touch/separation controls. Infinity semantics stay ordinary ordered comparisons, without new global finite checks.

## Exact closest edge wrapper

Original B140c9acb0/5B, PDB768:30296 is the complete get_closest_edge_id and contains exactly a tail JMP140c9a9d0. Its full common body140c9a9d0/726B, PDB768:28372 is get_closest_edge_pt_indices; scalar edge ID is its first returned index. This proves the wrapper must use the one existing closest_perimeter_edge_pair(point).0. Current public method has an independent squared-distance loop with initial infinity and no original positive-area dependency; that is a divergent implementation.

The current private pair helper already follows the complete common body: actual as_points2 (positive-area check), original edge order, EPS1e-20 threshold, original f32 interpolation/Euclidean sqrt, initial best distance1000, strict improvement/first tie, SSE second-operand MIN/MAX behavior, and actual (left,right) pair. No changes to that helper or its current successful users are needed.

A concrete finite counterexample uses center0/size(2,2)/axisX and point(2000,0): every edge is farther than1000, so original keeps default first edge0; the current public infinity/squared loop instead chooses right edge2. A rotated/translated version has the same result. Near point(1.2,0) returns2 and center tie returns0. Actual wrapper tests must assert those literal outputs, preserve corner tie order, and confirm degenerate area hits the existing original as_points guard. Do not weaken the shared helper or introduce a second numerical kernel.

The two scoped methods are necessary real callees of roof overlap and half-dormer/roof merge. This is not whole-Rectangle, whole-calculate or whole-App acceptance. Native identity/byte verification, source review and central same-source test execution remain separate stages. Root alone integrates production and runs tests; the frozen source candidate will bind this exact6353 before after design approval.
