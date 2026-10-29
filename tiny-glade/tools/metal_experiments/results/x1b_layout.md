# X1b generated-MSL layout check (generated)

Directory: `/tmp/x1b/argbuf-tier2-device`

- MSL files: 638; with a descriptor-set argument buffer: 514; with overlapping members: 514
- descriptor sets used (set -> structs): {'0': 514}
- `[[id(N)]]` values used (value -> declarations): {'0': 3143}
- non-overlapping members per argument buffer struct: {1: 514}
- descriptor kinds declared: {'buffer pointer': 1942, 'texture': 862, 'sampler': 339}
- distinct kinds in one struct (1..3) -> structs: {3: 207, 1: 237, 2: 70}

Reading: if every declaration uses id 0 and every struct has exactly one non-overlapping member, the translator treats the whole set as ONE flat table of descriptors that every kind indexes into.
