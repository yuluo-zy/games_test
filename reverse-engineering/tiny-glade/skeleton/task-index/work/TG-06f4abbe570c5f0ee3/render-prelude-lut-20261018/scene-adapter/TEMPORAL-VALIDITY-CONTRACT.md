# B temporal validity: original consumer, current session adapter

Oracle: `tiny-glade.rne`, SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Matching B PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. No C equivalence claim.

Full bounded native functions retained: temporal-single-B-rne.asm (1404f6e50,1532 B), temporal-string-B-rne.asm (1404f6850,1532 B), temporal-buffer-B-rne.asm (1404f62d0,1396 B), temporal-pair-B-rne.asm (1404f7450,610 B). No patched decompiler interpretation required.

Texture cache hit after descriptor equality imports the owned prior resource then writes result+24=1 at 1404f702e. Missing/resized creates/imports with initial access 0 and writes result+24=0 at 1404f72d6. Result flag2 is error. String variant has same writes at 1404f6a2e/1404f6cd6. Buffer hit/new write result+18 at 1404f643d/1404f665a. Pair calls String lookup twice; 1404f7605..760f combines flags with AND; 1404f7696 stores result+4c. This is allocation reuse validity, not first-allocation freshness.

Consumers SSGI, ReSTIR, RTGI and TAA directly pass this bit to history_valid. Existing game host inverted the current session history_valid and thus enabled first-frame uninitialized history while disabling reusable history. Fixed direct projection; core glue legacy field fresh remains source-compatible but documents history validity.

Architecture boundary: current rhapsody intentionally distinguishes allocated resources from committed initialized contents and preserves transactional parity on cancellation. This recovery retains that architecture. Named adapters use the actual session history_valid. Pair uses actual previous member history_valid; native's allocation-exists AND is not copied over the safer content-validity contract or implemented as a parallel cache.

Validation added: core_raster_draw::host_temporal_validity_tracks_submitted_history_resize_and_cancellation. Actual GPU raster attachment initialization and buffer fill, successful submission, resize, cancelled frame and subsequent publication; expected false/true/false/false/true. Parent runs cargo centrally; result pending at source handoff. This is a narrow host validity test, not a complete scene render.
