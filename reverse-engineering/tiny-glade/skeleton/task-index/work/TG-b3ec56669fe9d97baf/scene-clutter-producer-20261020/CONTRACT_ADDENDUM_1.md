# Addendum 1: exact demo gate, finite asset names and leaf candidates

This adds facts to the frozen `CLUTTER_CONTRACT_REVIEW.md` SHA256 `6f40543df279152309895d8cfdd40cf6819ad9dd3c088d1f7b6791309a50b234`; it does not overwrite that review input. 07's CORE_DIRECTION_REVIEW accepts the canonical resource/real loader/helper/generator-to-blueprint direction, not complete instantiation or runtime success.

## Gate correction

The loader closure's global is known, and the earlier phrase “pack-disabled” is corrected to **demo-omitted**. At `0x1422c1d1d` the import pointer cell is `0x14362c098`, pointing to `0x143634d90`. Exact existing PDB global records name the pointee `country_core::_DEV_OVERRIDE_IS_DEMO_MODE`. `evidence/loader-global-gate-B.json` preserves both raw public records, segment/offset, original pointer and import spelling. Current `GameConfig.dev.demo` is the host's existing explicit demo input. No new pack-status resource or default-off gate is introduced.

Only when `demo && call_argument_skip_in_demo` is true does closure0 omit actual mesh loading, make zero-sized Rectangle3d with axis `(1,0)`, retain the real name/NameHash, use radius1 and store the unchanged scalar flags/solid selector. Native byte9 remains legal Option None; it is never rejected by a unified metadata loader. Missing/invalid non-omitted assets fail at actual loading.

## Dynamic names and lifetime

Original load `0x1422c0b48` writes RangeInclusive<i32> start1/end6; standard iterator/format dependency work is not reimplemented. Exact closure1 format pieces are frozen in `evidence/loader-dynamic-format-B.json`: path prefix `assets/meshes/clutter/window_flower_bed_`, suffix `.json`; mesh-name prefix `clutter/window_flower_bed_`. Thus six finite, exact path/name literal pairs preserve the bytes, order, actual six loads and current builder's `&'static str` interface. 07 explicitly accepted this bounded static table. It is not generalized to unknown dynamic asset names; no Arc lifetime redesign or new leak is introduced.

Closure1 is PDB-size1,028 bytes at `0x1422c92c0` (inside a standard iterator wrapper, but the game callback is recovered). Its actual metadata write `mov dword[mesh+0x44],0x91f` means group0x1f, selector9, flags46/47 false. It loads and registers each real mesh, computes bbox/radius/name hash, stores its name and releases the formatted path. The dynamic vector is built between the 12th and 13th fixed loads, which the candidate preserves.

## Exact fixed-slot mapping

`evidence/loader-final-resource-fields-B.json` follows returned pointers through their final resource writes. `helper-path-results-B.json` follows all 56 original relative-table branches to the return tuple; all 55 fixed helper tuple selectors also match the corresponding actual loader metadata+0x45 writer. This cross-check does not merge their roles: the generator still reads metadata+0x45 directly.

For enum identity, current `systems::mode_manager::ClutterMeshTy` already holds the complete original 56 tags/serde names. The canonical resource module re-exports that type; no parallel enum/schema is added. Asset paths are taken from original loader call sites, not from serde names. Notable differences from the discarded inferred path list include `clutter/barrel`, bare registered names `fallen_leaves`, `fallen_petals`, `olden_glade_rock`, `tree_log`, `tree_stump`, and distinct paths with the same `tree_stump` registration name. Keeping native load order preserves name replacement behavior.

## Leaf candidate boundary

Candidates are under their own delegated claims:

* TG-5d112ff9638b93a02e/scene-clutter-producer-20261020/source/clutter_meshes.rs: canonical metadata/resource/helper and deletion component. No Default/empty success resource. Typed real mesh handles, actual bounds, checked Option selector, name hash, radius and unchanged raw native flags are retained.
* TG-b374246e420cab4618/scene-clutter-producer-20261020/source/load_clutter_meshes.rs: actual 55+6 load producer using current JSON Mesh and AssetMeshLibrary, exact names/paths/call order, original demo condition, real metadata writer. Existing loader import path re-exports the canonical resource and deletion component instead of another TypeId. An installation-root adapter allows root to use its real installation owner without process CWD tricks.

The loader manifest binds before/source/patch SHA256. Only `rustfmt --edition2024` was run (exit0). No Cargo, CPU tests, native differential, GPU run or production edit was performed. These files are **not full scene-chain acceptance**: root's module export and original startup registration remain required; canonical Nani atlas lookup, physics local pose/upsert identity, blueprint comparison/side effects, valid type0x3e domain, clear/copy scheduling and full edit/removal replay still require their own evidence and candidate review. In particular the old loader must not stay on a per-frame Update chain once real asset loading replaces its string list.

## Newly exposed tracking risk

Current Decorators::clear_modification_tracking clears both modified_ids and the entire decorators map. Registering that method at frame end would erase authoritative scene data. No candidate registers it on the strength of its name. Its original source contract must be checked by the current owner before the final lifecycle schedule is assembled.
