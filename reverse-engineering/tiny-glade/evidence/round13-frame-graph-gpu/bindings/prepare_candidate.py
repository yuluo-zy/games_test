from pathlib import Path
import difflib, hashlib, json, shutil

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings'
files = ['tiny-glade/crates/tiny-glade/src/raster_host.rs', 'tiny-glade/crates/tiny-glade/src/render_host.rs']
manifest = []
for rel in files:
    source = root / rel
    base = out / 'base' / rel
    candidate = out / 'candidate' / rel
    base.parent.mkdir(parents=True, exist_ok=True)
    candidate.parent.mkdir(parents=True, exist_ok=True)
    if not base.exists():
        shutil.copyfile(source, base)
    shutil.copyfile(base, candidate)
    manifest.append({'path': rel, 'base_sha256': hashlib.sha256(base.read_bytes()).hexdigest()})

host = out / 'candidate' / files[1]
s = host.read_text(encoding='utf-8')
s = s.replace('fn compute_view_range(', 'pub(crate) fn compute_view_range(', 1)
host.write_text(s, encoding='utf-8', newline='\n')

raster = out / 'candidate' / files[0]
s = raster.read_text(encoding='utf-8')
s = s.replace('BufferHost, RasterEntry, ShaderHost, TextureHost, rh_view_desc, upsert_shader_resource,',
              'BufferHost, RasterEntry, ShaderHost, TextureHost, compute_view_range, rh_view_desc,\n    upsert_shader_resource,', 1)
anchor = '        // Native attachment LOAD preserves an existing producer.'
replacement = '''        // A native shader binding carries its view independently from the
        // owner's graph identity. Project the very same view before graph build;
        // a descriptor selecting one layer must not read every layer/mip.
        let access_ranges = match raster_binding_ranges(&accesses, &bindings,
            colors.len() + usize::from(depth.is_some()), |resource| {
            let handle = graph.resource_handle(resource)?;
            Ok(graph.declaration().resources[handle.raw() as usize].descriptor)
        }) {
            Ok(ranges) => ranges,
            Err(error) => {
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
                return;
            }
        };
'''
assert s.count(anchor) == 1
s = s.replace(anchor, replacement + anchor, 1)
s = s.replace('        for a in &accesses {\n            let resource = declared',
              '        for (a, (range, binding_slot)) in accesses.iter().zip(&access_ranges) {\n            let resource = declared', 1)
s = s.replace('                && inactive_reads.get(&a.resource) == Some(&true)',
              '                && (*binding_slot).and_then(|slot| inactive_reads.get(&slot)) == Some(&true)', 1)
start = s.index('        for (a, (range, binding_slot)) in accesses.iter().zip(&access_ranges) {')
end = s.index('        declared.render_with_context(move |ctx| {', start)
part = s[start:end]
assert part.count('rhapsody::rg::AccessRange::Whole') == 2
part = part.replace('rhapsody::rg::AccessRange::Whole', '*range')
part = part.replace('declared.add_resource_access(resource, a.access, a.group)',
                    'declared.add_resource_access_range(resource, a.access, a.group, *range)')
s = s[:start] + part + s[end:]

anchor = '/// Resolve packet-local evidence onto the real graph resource.'
helper = '''/// Pair each native pass binding with the access appended by the same
/// DrawBuilder operation. Attachment and indirect accesses need no shader slot;
/// they retain their own declaration. Repeated views of one image are consumed
/// once in original order, instead of replacing them with an owner-wide access.
fn raster_binding_ranges(
    accesses: &[country_core::render::pass_builder::draw_stream::PassAccess],
    bindings: &[RenderPassBinding],
    attachment_accesses: usize,
    mut descriptor: impl FnMut(u32) -> rhapsody::Result<rhapsody::rg::Descriptor>,
) -> rhapsody::Result<Vec<(rhapsody::rg::AccessRange, Option<u8>)>> {
    use rhapsody::rg::{AccessRange, Descriptor};
    if attachment_accesses > accesses.len() {
        return Err(rhapsody::Error::invalid_usage("raster attachment accesses are absent"));
    }
    let mut used = vec![false; bindings.len()];
    let mut ranges = Vec::new();
    ranges.try_reserve_exact(accesses.len())
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for (access_index, access) in accesses.iter().enumerate() {
        // RasterPass::new writes attachments before any DrawBuilder bindings.
        // A shader alias must never steal that independent attachment access.
        if access_index < attachment_accesses {
            ranges.push((AccessRange::Whole, None));
            continue;
        }
        let matching = bindings.iter().enumerate().find(|(slot, binding)| {
            !used[*slot] && binding.resource_index == access.resource
                && binding.access_type == access.access
        });
        let (range, slot) = if let Some((slot, binding)) = matching {
            let slot_id = u8::try_from(slot).map_err(|_| {
                rhapsody::Error::invalid_usage("raster pass binding index exceeds u8")
            })?;
            used[slot] = true;
            let desc = descriptor(binding.resource_index)?;
            let range = match desc {
                Descriptor::Image(_) => compute_view_range(desc, &binding.view.unwrap_or_default())?,
                Descriptor::Buffer(_) if binding.view.is_none() => AccessRange::Whole,
                Descriptor::Buffer(_) => return Err(rhapsody::Error::invalid_usage(
                    "raster texture view refers to a buffer")),
            };
            (range, Some(slot_id))
        } else {
            (AccessRange::Whole, None)
        };
        ranges.push((range, slot));
    }
    if used.iter().any(|used| !used) {
        return Err(rhapsody::Error::invalid_usage(
            "raster pass binding has no matching native resource access"));
    }
    Ok(ranges)
}

'''
assert s.count(anchor) == 1
s = s.replace(anchor, helper + anchor, 1)
s = s.replace('/// alias in any draw tightens an inactive binding back into a content read.',
              '/// alias of that same pass slot tightens it back into a content read. Different\n/// views of one image remain distinct until the graph evaluates their ranges.', 1)
s = s.replace('std::collections::BTreeMap<u32, bool>)> {', 'std::collections::BTreeMap<u8, bool>)> {', 1)
s = s.replace('.entry(actual.resource_index)\n                    .and_modify', '.entry(access.slot)\n                    .and_modify', 1)
s = s.replace('inactive[&42]', 'inactive[&0]')
raster.write_text(s + (out / 'range_tests.rs').read_text(encoding='utf-8'), encoding='utf-8', newline='\n')

(out / 'manifest.json').write_text(json.dumps({'status': 'candidate_unverified', 'files': manifest}, indent=2), encoding='utf-8')
print(json.dumps(manifest, indent=2))
