from pathlib import Path
import hashlib, json, shutil, re

root = Path('D:/game')
parent = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings'
out = parent / 'attachments-v2'
out.mkdir(parents=True, exist_ok=True)
files = [
    'tiny-glade/crates/tiny-glade/src/raster_host.rs',
    'tiny-glade/crates/country-core/src/render/pass_builder/framebuffer_binding.rs',
    'tiny-glade/crates/country-core/src/render/pass_builder/draw_stream.rs',
    'tiny-glade/crates/rhapsody/src/rg/access_ledger.rs',
]
manifest = []
for rel in files:
    source = parent / 'candidate' / rel if rel == files[0] else root / rel
    base, candidate = out / 'base' / rel, out / 'candidate' / rel
    base.parent.mkdir(parents=True, exist_ok=True)
    candidate.parent.mkdir(parents=True, exist_ok=True)
    if not base.exists():
        shutil.copyfile(source, base)
    shutil.copyfile(base, candidate)
    manifest.append({'path': rel, 'base_sha256': hashlib.sha256(base.read_bytes()).hexdigest()})

p = out / 'candidate' / files[0]
s = p.read_text(encoding='utf-8')
s = s.replace('                RasterState, ShaderReflection as CoreReflection,',
              '                RasterState, RasterPassColorAttachment, RasterPassDepth, ShaderReflection as CoreReflection,', 1)
s = s.replace('        let access_ranges = match raster_binding_ranges(',
              '''        let attachment_accesses = match raster_attachment_accesses(&colors, depth.as_ref()) {
            Ok(attachments) => attachments,
            Err(error) => {
                let mut declared = graph.add_pass(name);
                declared.side_effect();
                declared.render_with_result_context(move |_| Err(error));
                return;
            }
        };
        let mut access_ranges = match raster_binding_ranges(''', 1)
anchor = '        // Native attachment LOAD preserves an existing producer.'
start = s.index(anchor)
end = s.index('        let mut declared = graph.add_pass(name);', start)
# Replace resource-wide preserving inference with view-specific access intent.
s = s[:start] + '''        for (index, attachment) in attachment_accesses.iter().enumerate() {
            let actual = &accesses[index];
            let projection = (|| -> rhapsody::Result<_> {
                if actual.resource != attachment.resource
                    || actual.access != attachment.access
                    || actual.group != 1
                {
                    return Err(rhapsody::Error::invalid_usage(
                        "raster attachment differs from its native resource access"));
                }
                let handle = graph.resource_handle(attachment.resource)?;
                let descriptor = graph.declaration().resources[handle.raw() as usize].descriptor;
                compute_view_range(descriptor, &attachment.view)
            })();
            match projection {
                Ok(range) => access_ranges[index] = (range, None),
                Err(error) => {
                    let mut declared = graph.add_pass(name);
                    declared.side_effect();
                    declared.render_with_result_context(move |_| Err(error));
                    return;
                }
            }
        }
        // Native LOAD keeps the producer of this exact view. An independent
        // layer or mip never turns a first write into a preserving operation.
        // Undefined/partially initialized inputs are not invented here: an
        // overlapping producer still requires the complete LOAD range defined.
        let preserving_attachments = attachment_accesses.iter().enumerate()
            .map(|(index, attachment)| {
                attachment.load
                    && rhapsody::command::is_write(attachment.access)
                    && raster_has_previous_contents(&graph, attachment.resource, access_ranges[index].0)
            }).collect::<Vec<_>>();
''' + s[end:]
s = s.replace('        for (a, (range, binding_slot)) in accesses.iter().zip(&access_ranges) {',
              '        for (index, (a, (range, binding_slot))) in accesses.iter().zip(&access_ranges).enumerate() {', 1)
s = s.replace('            let result = if (preserving_attachments.contains(&a.resource)',
              '            let result = if (preserving_attachments.get(index) == Some(&true)', 1)
s = s.replace('                && !colors.iter().any(|color| color.id == a.resource)\n                && depth\n                    .as_ref()\n                    .is_none_or(|depth| depth.texture_id != a.resource)',
              '                && index >= attachment_accesses.len()', 1)
s = s.replace('            for c in &colors {', '            for (index, c) in colors.iter().enumerate() {', 1)
s = s.replace('                // CLEAR（tag1）需要清除值；core 的附件描述里没有颜色清除值，取零（I：`AttachmentCache` 的缺省）。\n                let clear = (c.tag == 1).then_some([0.0f32; 4]);',
              '''                // B color tag1's +4..+0x14 words become VkClearValue.
                // The legacy core field is named `view`; preserve its bits.
                let clear = (c.tag == 1).then(|| c.clear_color_bits().map(f32::from_bits));''', 1)
s = s.replace('clear 值取缓存默认的 0', 'CLEAR 值来自原四字 payload', 1)
s = s.replace('''                let Ok(mut a) = Attachment::color(texture.shared(), 0, clear) else {
                    return false;
                };''', '''                let mut a = match Attachment::color(texture.shared(), 0, clear) {
                    Ok(attachment) => attachment,
                    Err(error) => { ctx.report_error(error); return false; }
                };''', 1)
s = s.replace('''                let Ok(a) = Attachment::depth(texture.shared(), 0, clear) else {
                    return false;
                };''', '''                let a = match Attachment::depth(texture.shared(), 0, clear) {
                    Ok(attachment) => attachment,
                    Err(error) => { ctx.report_error(error); return false; }
                };''', 1)
s = s.replace('                a.desc.load = vk::AttachmentLoadOp::from_raw(c.tag as i32);',
              '''                a.view = match rh_view_desc(&texture, &attachment_accesses[index].view) {
                    Ok(view) => view,
                    Err(error) => { ctx.report_error(error); return false; }
                };
                a.desc.load = vk::AttachmentLoadOp::from_raw(c.tag as i32);''', 1)
s = s.replace('                a.view.base_layer = d.layer;\n                a.view.layer_count = 1;',
              '''                a.view = match rh_view_desc(&texture, &attachment_accesses[colors.len()].view) {
                    Ok(view) => view,
                    Err(error) => { ctx.report_error(error); return false; }
                };''', 1)
anchor = '/// Pair each native pass binding with the access appended by the same'
helper = '''/// Original new_impl gives every attachment mip zero / level_count one.
/// Only depth adds layer_count one and the caller's base_layer. Color's four
/// payload words are a clear value and never change the subresource view.
struct RasterAttachmentAccess {
    resource: u32,
    access: u8,
    load: bool,
    view: country_core::rhapsody_types::TextureViewDesc,
}

fn raster_attachment_accesses(
    colors: &[RasterPassColorAttachment],
    depth: Option<&RasterPassDepth>,
) -> rhapsody::Result<Vec<RasterAttachmentAccess>> {
    use country_core::render::pass_builder::framebuffer_binding::{
        COLOR_ACCESS_TEXTURE, COLOR_ACCESS_VIEW, DEPTH_ACCESS_READ, DEPTH_ACCESS_WRITE,
    };
    let mut attachments = Vec::new();
    attachments.try_reserve_exact(colors.len() + usize::from(depth.is_some()))
        .map_err(|_| rhapsody::Error::OutOfHostMemory)?;
    for color in colors {
        if color.tag > 2 {
            return Err(rhapsody::Error::invalid_usage("invalid native color attachment tag"));
        }
        attachments.push(RasterAttachmentAccess {
            resource: color.id,
            access: if color.tag == 0 { COLOR_ACCESS_TEXTURE } else { COLOR_ACCESS_VIEW },
            load: color.tag == 0,
            view: country_core::rhapsody_types::TextureViewDesc::new().level_count(Some(1)),
        });
    }
    if let Some(depth) = depth {
        attachments.push(RasterAttachmentAccess {
            resource: depth.texture_id,
            access: if depth.writable { DEPTH_ACCESS_WRITE } else { DEPTH_ACCESS_READ },
            load: matches!(depth.clear, DepthClear::Load),
            view: country_core::rhapsody_types::TextureViewDesc::new()
                .level_count(Some(1)).base_layer(depth.layer).layer_count(Some(1)),
        });
    }
    Ok(attachments)
}

/// A preserving attachment must depend only on earlier writes to its selected
/// subresources. Source initialization remains checked by the production graph.
fn raster_has_previous_contents(graph: &RhGraph, resource: u32, range: rhapsody::rg::AccessRange) -> bool {
    let imported = graph.declaration().resources.get(resource as usize).is_some_and(|entry| {
        match entry.source {
            rhapsody::rg::OriginSource::ImportOwned { initial_access, .. }
            | rhapsody::rg::OriginSource::ImportUnowned { initial_access, .. } => initial_access != 0,
            rhapsody::rg::OriginSource::Create => false,
        }
    });
    imported || graph.declaration().passes.iter().any(|pass| {
        pass.accesses.iter().any(|access| {
            access.resource_index == resource
                && rhapsody::command::is_write(access.access_code)
                && access.range.overlaps(range)
        })
    })
}

'''
assert anchor in s
s = s.replace(anchor, helper + anchor, 1)
s += (out / 'attachment_tests.rs').read_text(encoding='utf-8')
p.write_text(s, encoding='utf-8', newline='\n')

p = out / 'candidate' / files[1]
s = p.read_text(encoding='utf-8')
s = s.replace('/// 视图字段（tag1 的 `+4..+0x14`，16 B）：机器码只整块搬运，字段含义 U。\npub type ColorViewWords = [u32; 4];',
'''/// Tag1 +4..+0x14 is the exact VkClearColorValue payload. B new_impl's
/// from_iter @14157b1d0 copies it to RasterPassColorAttachment +0x40;
/// begin_render_pass @1416f719b..71db places it in pClearValues.
pub type ColorClearWords = [u32; 4];
/// Source compatibility for the historical misnomer; these are clear bits.
pub type ColorViewWords = ColorClearWords;''', 1)
s = s.replace('    /// tag1：带 16 B 视图字（@+4）的引用（引用 @+0x18）。',
              '    /// tag1：16 B VkClearColorValue 位模式（@+4），纹理引用 @+0x18。\n    /// `View`/`view` 是保留的历史源码名，不是图像子资源视图。', 1)
s = s.replace('    /// 视图字（tag0 为 0）。\n    pub fn view_words(&self) -> ColorViewWords {',
              '    /// Exact clear payload; LOAD / DONT_CARE have no payload.\n    pub fn clear_color_bits(&self) -> ColorClearWords {', 1)
anchor = '/// 清除值（深度附件'
before, after = s.split(anchor, 1)
pos = before.rfind('\n}')
before = before[:pos] + '''
    /// Source-compatible legacy accessor for the same clear-value bits.
    pub fn view_words(&self) -> ColorViewWords {
        self.clear_color_bits()
    }
''' + before[pos:]
s = before + anchor + after
s = s.replace('tag1 在 `+4` 处多带 16 B\n//!   视图字段', 'tag1 在 `+4` 处多带 16 B\n//!   VkClearColorValue 字段')
s = s.replace('视图/句柄变体', '清除颜色变体')
s = s.replace('无视图字', '无清除颜色载荷').replace('16 B 视图字', '16 B 清除颜色字')
p.write_text(s, encoding='utf-8', newline='\n')

p = out / 'candidate' / files[2]
s = p.read_text(encoding='utf-8')
s = s.replace('    pub view: ColorViewWords,',
              '    /// Exact VkClearColorValue bits; historical field name retained for source compatibility.\n    pub view: ColorViewWords,', 1)
anchor = '/// 深度状态（`RasterPass +0xe0..+0x12e`）。'
s = s.replace(anchor, '''impl RasterPassColorAttachment {
    pub fn clear_color_bits(&self) -> [u32; 4] {
        self.view
    }
}

''' + anchor, 1)
s = s.replace('                view: a.view_words(),', '                view: a.clear_color_bits(),', 1)
p.write_text(s, encoding='utf-8', newline='\n')

p = out / 'candidate' / files[3]
s = p.read_text(encoding='utf-8')
s, count = re.subn(r'(merge_content_policy\(\s*&mut entries\[previous_index\],\n)(\s*)contents_unused,',
                  r'\1\2reads_previous,\n\2contents_unused,', s)
assert count == 3, count
anchor = 'fn merge_content_policy(previous: &mut AccessDeclaration, unused: bool, allow_undefined: bool) {'
assert s.count(anchor) == 1
s = s.replace(anchor, '''fn merge_content_policy(previous: &mut AccessDeclaration, reads_previous: bool, unused: bool, allow_undefined: bool) {
    // Aliases of this exact range/version retain every previous-content
    // dependency in either order. Do not erase inactive/undefined evidence:
    // the normal graph contract still rejects preserving metadata on reads.
    previous.reads_previous |= reads_previous;''', 1)
p.write_text(s, encoding='utf-8', newline='\n')

new_test = 'tiny-glade/crates/rhapsody/tests/preserving_dedup.rs'
p = out / 'candidate' / new_test
p.parent.mkdir(parents=True, exist_ok=True)
shutil.copyfile(out / 'preserving_dedup.rs', p)
manifest.append({'path': new_test, 'base_sha256': None})
(out / 'manifest.json').write_text(json.dumps({'status': 'candidate_unverified', 'basis': 'bindings-v1 frozen candidates', 'files': manifest}, indent=2), encoding='utf-8')
print(json.dumps(manifest, indent=2))
