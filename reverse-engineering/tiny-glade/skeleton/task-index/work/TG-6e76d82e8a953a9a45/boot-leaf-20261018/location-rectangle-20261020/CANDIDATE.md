# Rectangle candidate v1

Implements the independently reviewed design c35b6989 / review eca48413. Two new public helpers share the exact original private pair search/projection, preserving area guard,1000 finite initial Euclidean distance, strict first tie, SSE operand priority, epsilon and closing U. Existing get_closest_edge_id and all unrelated Rectangle behavior stay byte-for-byte outside the inserted hunks.

Four CPU test sources cover ties+closing endpoint, original finite-distance default, a true rotated frame, and distinct NaN point/U guards plus zero area. Tests are unexecuted (Cargo is central-only); rustfmt parsed/formatted the candidate successfully. No native/GPU/App execution. Dependency is existing glam only; no Cargo change. The source insertion is usable independently, but it does not install the location validator.
