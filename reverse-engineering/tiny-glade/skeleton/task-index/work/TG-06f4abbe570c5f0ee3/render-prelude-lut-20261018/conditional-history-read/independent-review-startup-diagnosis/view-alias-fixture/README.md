# Same-image view alias regression candidate

Append-only test for current tiny-glade/tests/render_host.rs. Production source frozen; no production file edited or Cargo/GPU run by this worker. Rustfmt parsed/formatted only append-render-host.rs.

Test: inactive_compute_views_keep_active_mips_with_interleaved_buffer_binding.

Four cases: inactive mip0 before/after active mip1, each with/without initialization of only mip1. A storage-buffer output binding sits between image bindings in both orders and keeps the real GraphPasses compute declaration live. The no-initialization branches require an exact UninitializedRead for the image; initialized branches require acceptance, both native image ranges, SAMPLED usage, the first-to-last lifetime, original code/group and independent check_plan. Metadata must affect only mip0, never mip1 or the interleaved output buffer.

GraphPasses requires a real Device and ShaderHost compute entry, so the test uses the existing graphics_device strict helper. Its diagnostic iota SPIR-V/three-slot reflection is a host-mapping fixture, not a recovered gameplay shader or pixel validation. No callback is executed; there is no Vulkan queue submission, texture allocation, mip clear or readback. The initializing pass declares the hypothetical producer needed for this graph-contract counterexample.

Root invocation after compilation: strict Vulkan with both normal layer/DYLD paths, then the built render_host test binary filtered by the exact name. This worker does not claim compile or runtime PASS. No API visibility or production framework was expanded.
