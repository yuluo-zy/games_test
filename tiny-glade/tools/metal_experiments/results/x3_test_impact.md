# X3 test impact (generated)

Counts are test code only. "#[test] in files" is an upper bound: a test in a touched file may not use the token.

| family | occurrences | files | #[test] in those files |
| --- | ---: | ---: | ---: |
| stage | 203 | 52 | 355 |
| access | 139 | 37 | 293 |
| layout | 242 | 51 | 353 |
| aspect | 130 | 38 | 277 |
| queue_family | 167 | 38 | 262 |
| access_scope | 70 | 20 | 128 |
| native_state | 63 | 11 | 73 |
| vk_result | 87 | 15 | 147 |
| error_vulkan | 12 | 9 | 108 |
| timeline_api | 6 | 4 | 51 |
| vk_format | 186 | 57 | 421 |
| usage_flags | 406 | 83 | 494 |
| semaphore | 59 | 26 | 220 |
| command_buffer | 119 | 43 | 252 |

Union of all families: 139 files, 841 #[test] functions.
Union of the CORE_SPLIT families (stage, access, layout, aspect, queue_family, access_scope, native_state, vk_result, error_vulkan, timeline_api, semaphore): 100 files, 643 #[test] functions (rhapsody 493, country-core 0, tiny-glade 150).

Total #[test] per crate: rhapsody 1008, country-core 3099, tiny-glade 492

| crate | module | touched files | #[test] in those files |
| --- | --- | ---: | ---: |
| rhapsody | tests | 74 | 370 |
| rhapsody | rg | 21 | 107 |
| tiny-glade | src | 6 | 104 |
| tiny-glade | tests | 14 | 66 |
| rhapsody | present | 2 | 34 |
| country-core | systems | 1 | 27 |
| rhapsody | command | 6 | 25 |
| rhapsody | tracking | 2 | 22 |
| rhapsody | sync | 2 | 14 |
| rhapsody | pipeline | 1 | 13 |
| rhapsody | transfer | 3 | 13 |
| rhapsody | examples | 1 | 10 |
| rhapsody | resource | 1 | 10 |
| country-core | tests | 1 | 9 |
| rhapsody | native | 1 | 6 |
| rhapsody | bindless | 1 | 4 |
| rhapsody | error | 1 | 4 |
| rhapsody | device | 1 | 3 |
