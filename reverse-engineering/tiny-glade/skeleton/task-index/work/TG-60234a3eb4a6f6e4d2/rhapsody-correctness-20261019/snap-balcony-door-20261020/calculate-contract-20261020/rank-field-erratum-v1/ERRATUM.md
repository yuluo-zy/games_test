# Distinct input rank and selected merge rank

This append-only correction supersedes two rank descriptions in the original frozen calculate contract5d5afc87; the old file and its manifest7b1fad8f remain unchanged for audit. No production code or candidate changes are made.

**Accepted output+5c preserves the original input DecoratorInfo.rank. It is not replaced by the selected merge rank.** The optional merge selection carries its own resulting rank at+48 and target DecoratorId at+44. These two values may legitimately differ.

- Wall input rank is loaded from actual info+44 at14127d133 and saved to RBP+186 atd142. Accepted output at14127e8b3 loads precisely RBP+186 and e8ba writes output+5c. The accepted merge shortcut does not overwrite RBP+186. Its selected rank is separately read from proposal byte RBP+bc atd383, shifted into the high part of RBX atd390, and written as part of output+44 at e8a9; it therefore reaches output+48.
- Roof input rank is read from info+44 at14127db1f and saved to RBP+d0 atdb23. Common accepted output at14127ea4d loads RBP+d0 and ea54 writes output+5c. A selected roof proposal separately reads its rank at RBP+c4 at e100 and packs it into output+44/+48 through e10d/e111 and ea45.
- Terrain and stair directly read input info+44 atd3ce ande248, respectively, then use the same ea54 output+5c write.
- Actual place consumer reads only the optional merge rank at7ac701 (accepted+48), guarded by the merge flag at7ac6ee. Its existence does not justify changing the independent input-rank field.

Consequently the base table row for+5c must say original/current input rank, including accepted merge branches. The wall merge step replaces the selected merge rank in MergeSelection; it does not replace accepted.current_rank. A semantic Rust value should make this distinction explicit. A future genuine nonempty-merge test must choose an original rank and a different merged rank and assert both fields independently, as well as which value the actual caller consumes. This is an evidence correction, not an executed test result.
