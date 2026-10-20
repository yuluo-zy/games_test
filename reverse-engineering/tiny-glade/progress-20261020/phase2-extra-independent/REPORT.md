# Independent ExtraInfo verification

Original binary B SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

Executed original From leaves140839520/530/540/550/560 directly under Unicorn without hooks: six variants (stair niche0 and1, chimney4, flag5, window6, lantern7), 96 inputs each, total576cases. Full72B destination initialized with random nonzero sentinels. Float inputs include signed zero, signed NaNs, infinities and arbitrary bit patterns. Full leaf assembly retained in from-original.asm.

Actual production DecoratorInfo struct and extra_disc/extra_bytes/extra/set_extra/extra_style/DecoratorExtraInfo::disc method bodies were extracted verbatim from country-core/src/resources/walls/decorator.rs and compiled with rustc. No body substitutions. Only surrounding standalone type declarations and test I/O boundary were supplied. Manifest records source SHA and extracted methods.

PASS576/576: every byte of full72B output equals actual native leaf write result at ExtraInfo baseInfo+0x30; getter preserves all typed payload bits; niche0/1 survives; extra_disc/style agree; unused padding, destination union and rank/type/state bytes remain untouched. This specifically rejects the former destination+0x10/high-byte-tag oracle. The fixture relocates the standalone ExtraInfo leaf destination to Info+0x30; provenance of that containing-struct offset is supplied by the separate central constructor/history evidence, rather than claimed to follow from a standalone leaf alone.

Reproduce native_extra_from.py then build_actual_extra.py. actual-source-results.txt contains actual execution result. No core production files changed by this reviewer.
