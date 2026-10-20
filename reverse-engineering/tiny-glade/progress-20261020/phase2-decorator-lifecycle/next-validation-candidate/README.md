# Next coherent validation chunk, not production

Frozen during the 2026-10-20 commit closeout. These two full source snapshots
are candidate material, not compiled or installed code. Relative to the live
source at freeze, their sole intended change is complete DecoratorLocator
identity (storage address plus DecoratorId) and forwarding that actual ID from
the two legacy place/change writers and the canonical typed-change writer.
Do not overwrite later source edits with these snapshots; apply only that
semantic delta after reviewing the original producer and callback together.

This fixes a genuine lost-identity problem: a HashMap keyed by address alone
merges independent affected decorators attached to one wall. Original
1412b868e reads the trailing locator ID; 1412b8697 reads its distinct address;
1412b86bc passes both to the actual storage lookup. Keep traversal order and
address-specific lookup, rather than recovering one arbitrary ID from d2s.

It is intentionally not in production because the next coherent chunk must
also preserve complete original location tags/reasons and their algorithms,
collision outline ownership and loader, genuine feedback UI payloads, and unit
mute event readers. No replacement boolean, Unsupported success, fake default
catalogue or half-validation system is provided here.

Use the existing complete evidence rather than extracting guessed functions:

- `skeleton/task-index/work/TG-2cc8aec081d0811cbe/clutter-startup-20261020/validate-investigation/CONTRACT.md`
- `skeleton/task-index/work/TG-719dcf28a173a4b9b1/clutter-startup-20261020/location-result-20261020/LOCATION-CONTRACT-V1.md`
- `skeleton/task-index/work/TG-60901c6c8508b1af76/clutter-startup-20261020/canonical-collision-schema-20261020/SCHEMA-CONTRACT-V1.md`
- `skeleton/task-index/work/TG-c4a7246ce4d35f5279/clutter-startup-20261020/mute-reader-20261020/READER-CONTRACT-V1.md`

All paths are relative to reverse-engineering/tiny-glade. Old absolute paths
inside those historical reports are provenance only. Verify current PE identity
against f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
