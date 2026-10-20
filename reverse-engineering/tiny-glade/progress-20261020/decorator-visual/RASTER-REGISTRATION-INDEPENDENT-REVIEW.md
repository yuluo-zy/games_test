# Independent raster-upload registration review

Conclusion: original B registers `upload_world_raster_buffers_to_staging` in
**Update / GameSystemSet14 = HistoryStart**. This is established from the
original PE, not from the reconstructed host. No conflicting original schedule
or set was found in this bounded registration path.

Original binary SHA256:
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`review-raster-registration.py` independently reads the original PE, hashes10
bounded original ranges, records the actual vtables, and decodes the labels.
See `raster-registration-independent-manifest.json` and `raster-review-*.asm`.
This is static evidence; it does not prove a successful load or GPU execution.

## Exact chain

1. Parent14059d450 at14059fc85 sets DL=0x0e; call14059fc87 targets1405c0400.
   The converter saves EDX in EBX, constructs the system using vtable142a05448
   at1405c056e, then writes BL as the set value at1405c0584 and interns it with
   the actual GameSystemSet vtable14292af98 at1405c058e.
2. That system vtable's +0x50 and +0x60 entries are1405bef90 and14045dc60.
   Their original calls at1405bf404 and14045e0d8 both target leaf140957bf0.
3. GameSystemSet vtable+0x18 points to fmt140037760. Its length table14292b070
   and signed-offset table14292b1d8 decode index14 as `HistoryStart` (also
   index36=`PostGeneratorGameplay`, index44=`PrepareRendering`).
4. The converter result at parent stack440..4b0 is copied by14059fed8..ff33
   into tupleda0..e10. Parent14059ff83 passes the tupleb00..e10 to140327490.
5. That add-systems specialization calls140172580 at140327551.
   This schedule selector uses vtable1429f1e78 at1401725c8. Its fmt slot+0x18
   is140526ef0; the original fmt writes exactly6 bytes from1429f1cb0:
   **`Update`**. This closes the schedule identity independently of host code.

The128-bit value8e1bd251962ba784/6a782374c0e6529b at1403274c3..4d7 is used
for the preceding world-resource lookup (Schedules). It should not itself be
described as the Update schedule label identity.

## Candidate host assessment

Placing the actual consumer in `Update` with `GameSystemSet(14)` matches this
original registration evidence. The leaf retains its event-reader semantics
and canonical world-raster/SSBO owners.

The reconstructed `run_if(GraphicsDeviceResource exists)` is an **engineering
CPU-only-host gate**, not a recovered original guard. The original leaf takes
EventReader, six raster resources and AssetSsboLibrary; its wrappers require
those resources and do not establish the same optional-device test. Device
presence alone does not establish that the six mapped SSBOs exist. The gameplay
startup and corresponding acceptance must establish those real mapped owners
before emitting an upload request; the original missing-buffer failures should
remain intact. This distinction does not invalidate the recovered set/schedule,
but the gate must not be counted as original business logic or runtime proof.

No production source was edited during this independent review.
