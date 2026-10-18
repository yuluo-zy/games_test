# Original B indirect buffer producer correction

B raw RNE f507a6ce… PDB 2d9686b8… exact bounded assembly saved here; no C acceptance claimed. Four native constructors use BufferDesc::new(fixedSize,0x122,1).alignment(4), named after the actual SSBO type. Existing generic new(0x12/0x22) remains valid for its other native instances and is unchanged. New from_created_buffer is a source-host carrier only, not a new original function.

Observed byte divisibility checks fix AtomicCounterBuffer layout from earlier inferred8Bx1 to4Bx2 (141614919 mask3,141614938 limit12); Draw16Bx1; Dispatch16Bx8; Indexed20Bx24. Pointer alignment4 checked after length divisibility, then capacity. Values exceeding capacity reject before device call. Some(empty) is distinct fromNone and remains an explicit empty upload.

None branch data: atomic creates8B with dataNone (14161499f..9f4); Draw alloc16/zeroesxmm0 at1416108ad/b0; Dispatch alloc128 and repeats the 16B constant142aaa1e0=[0,1,1,0] at141613e40..62; Indexed alloc480/memset0 at141614dda..de5. These startup bytes differ from reset_indirect_args per-frame content (draw instanceCount1, indexed fixed index counts). Do not reuse the reset payload at startup or claim everything initialized.

Consumer reset_indirect_args already retrieves these exact four SsboType identities lazily in the same frame, allocates DynamicConstants upload chunks and copies them. Main prelude must keep native order GpuWorld.import→FrameConstants→LUT→resetIndirect, with same physical BufferHost and device. No fake IDs or alternate buffers.
