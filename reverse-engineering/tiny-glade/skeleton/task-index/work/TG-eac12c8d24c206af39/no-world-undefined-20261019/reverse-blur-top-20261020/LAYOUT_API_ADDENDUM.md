# Necessary public read-only layout query

This supplements, without editing, frozen DESIGN.md SHA256 10456bae317b967e08b6392f4dbed472586fbc9e7ec7a90d87ce9b72e43f3bcc. It changes only the placement of the exact-range native layout query. It is not source/runtime acceptance.

The production library's internal_mod macro makes rg::sync_plan and resource::range_state crate-private unless the test-support feature is enabled. Their item-level public native_range/intersection/difference functions therefore cannot be called by the normal application host. Enabling test-support, publishing the implementation modules or copying the range algebra into the host would unnecessarily widen the dependency surface.

Add one public read-only method to the already public PassContext:

    image_layout_for_range(resource: u32, range: AccessRange) -> Result<vk::ImageLayout>

It resolves only the current validated frame/pass's actual native layout:

1. Validate the logical resource index and image descriptor kind; validate/normalize the exact range against that descriptor through the existing internal native_range function using the compiled separate-depth-stencil policy.
2. PassContext.pass_index is the compressed retained/scheduled index. Require RangeProgram.passes[pass_index] and its pass field to agree. Require the compiled pass's uses slice to be in bounds.
3. For every native use of that exact logical resource that intersects the requested range, require one common layout. Determine complete coverage by subtracting intersected native cells with the existing internal intersection/difference algebra. Missing coverage, no use, mixed layouts, invalid resource/kind/range or inconsistent pass index returns an error.
4. Do not add accesses, owners, transitions or writes, mutate the plan, recompile, widen descriptor coverage, choose a fallback layout or inspect a shader name. A query is not an authorization for an undeclared resource.
5. The host's selected graph-managed compute view calls this method with its actual range and writes that returned layout to the actual BindlessResource. Its ABI check still permits only existing sampled READ_ONLY/GENERAL or storage GENERAL. Individual mip array elements query their own ranges. Raster/external paths retain their current separate contracts.

This method is a current-architecture consumer API and has no recovered native ABI claim. The second candidate package may include executor.rs (method plus CPU query tests), render_host.rs and blur_pyramid.rs, and explicitly depends on the first library content-contract package. It must not modify the already frozen first package.

Tests must cover sampled-only READ_ONLY, exact sampled/storage GENERAL, missing/partial coverage, wrong resource/kind/range, mixed layouts in a single view, a culled earlier pass proving compressed index use, and actual descriptor writes/ABI kind validation. A CPU-only helper test is not a replacement for the original-SPV real GPU check. Root retains all production and build execution.
