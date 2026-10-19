# Required Rectangle2d corner-edge directions, B scoped design

The true corner-window generator calls this exact598B dependency at1407f6d71. The current shared Rectangle2d source has the correct four-point order but no corresponding public method. This is evidence for the unique rectangle owner01/root to implement, not a source write by the window owner.

Proposed API `pub fn get_edge_dirs_from_corner_id(self,corner:usize)->[Vec2;2]` (an actual two-Vec2 result, preserving order). Original first asserts `size.x*size.y>0` (NaN and nonpositive fail), then asserts `corner<=3`. Current `as_points2()` already owns this shape/ordered transform and positive-area requirement; call it before the corner-bound assertion to preserve failure order. Do not call get_point(corner) first and reverse these two gates.

The original jump table routes0→a2f8,1→a3be,2→a339,3→a37d. The constructed world corners in the shared current order are0=(-,-),1=(-,+),2=(+,+),3=(+,-). For corner i, first difference is points[(i+3)%4]-points[i], second is points[(i+1)%4]-points[i]. For each difference independently, reciprocal=1.0/sqrt(d.y*d.y+d.x*d.x); multiply each original component by that same reciprocal. Preserve subtraction, dy-square+dx-square order and no fallback. This is actual world-space endpoint subtraction and normalization, not returning idealized local axes: rotated/translated/nonunit-axis data and rounding remain observable. The method never substitutes zero for an invalid direction and must not use normalize_or/normalize_or_zero.

The existing get_corner_normal196B is separately retained to bind the true corner caller; this contract does not propose modifying its already-existing source. No ABI/Copy/default/scene completion is claimed.01 must rebase from current rectangle SHA at implementation time; root alone integrates/tests.

Tests for the future same-source companion should cover all4 ordered corners, actual rotated and translated unequal sides, previous-versus-next orientation, zero/NaN-area failure before invalid index, index4 with validarea failure, and positivearea/nonfinite-axis propagation (no manufactured fallback). These tests have not been written or run here.
