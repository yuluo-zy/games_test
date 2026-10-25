# Independent-review correction: stationary weight

V1 frozen input is unchanged. This incremental hunk corrects both rounded divisions from `ceil` to `round`, following building_audit's original import proof: draw `0x140a7d590` and `0x140a7d5a4` call thunk `0x142923fa0`, which is `FF25F2F18600` and resolves to IAT `0x143193198`, `api-ms-win-crt-math-l1-1-0.dll!roundf`. The original weight is max(round(60/round(1/max(delta,1e-5))),1). For delta .04, v1 produced 3 but original is 2.

Eight source regression cases include non-integral ratios. No original execution or Cargo/test pass is claimed in this correction. The existing simple 1/60 World frame is unchanged and does not independently detect this bug. Root applies this hunk after v1; no shared module is touched.
