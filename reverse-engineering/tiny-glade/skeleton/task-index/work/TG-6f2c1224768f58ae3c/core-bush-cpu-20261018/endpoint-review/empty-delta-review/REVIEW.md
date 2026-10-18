# Endpoint empty archive delta — independent review

Accepted in this bounded scope: the original frozen20 endpoint candidate plus separately frozen9 empty-archive-v4 delta. All9 hashes/sizes match and isolated patch application reproduces both sources byte-for-byte. No Cargo/native execution or production changes occurred.

The six production lines close the confirmed finding in both callers. Fresh ui_draw and endpoint phase4 now call the existing selector1 archive API with an empty Decorators when terrain storage is None. Original archive_current selector1 unconditionally constructs and clones an empty collection into the first before slot; see independently frozen archive-empty-evidence/87e5719b9c73d0ec0f9e431ebd740ed37587385b1ebe6d011c2d6f51ccd61934. Existing archive_storage_decorators returns when special1 is already Some, so subsequent nonempty storage cannot overwrite that empty before snapshot. Cull allowed IDs still derive only from the actual current bucket.

Both regression sources exercise real World/system paths with terrain None, assert Some(empty) while the live bucket stays None, then create a real bucket and assert the first before snapshot persists while cull permission includes its real ID. Tests are reviewed source, not executed here. No fabricated mirrored bucket or owner is introduced.

The original20 report remains applicable for published-vs-live endpoint gates and reverse/projection behavior. Automatic History recording lifecycle, actual publisher membership hook, renderer hit selection and snap>1/merge remain separately owned integration/validation boundaries; this delta does not accept them.
