# Absent starting point still executes f32 interpolation

This corrects the informal sentence in frozen ALGORITHM.md that None emits only the first point. The precise numbered algorithm and existing native implementation/reference were already correct: None chooses currentXZ as interpolation start and **still executes every f32 multiply/add in the loop**. It must not take a direct single-point fast path.

Because `(1-t)*p+t*p` may differ by one f32 ULP across i, large coordinates can pass both strict gates even with identical mathematical start/end. The original complete entry proves:

| None point x, z=-1 | ticks | accepted point count |
|---|---:|---:|
| -816794.8125 | 3 | 2 |
| 235632.703125 | 6 | 4 |
| 683839.8125 | 7 | 2 |
| -261540.328125 | 16 | 5 |
| 1000000 | 7 | 3 |

All eight added original runs match the separate f32 reference bit for bit, including output state. There is no semantic need to clamp those coordinates in the smoother. Original world constraints may limit reachable values, but this leaf preserves its arithmetic. The terrain owner confirmed retaining the full loop and will include a large-coordinate regression. Original nine frozen files remain unchanged.
