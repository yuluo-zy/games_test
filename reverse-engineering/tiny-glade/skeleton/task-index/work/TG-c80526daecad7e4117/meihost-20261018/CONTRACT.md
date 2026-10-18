# B text producer → MEI graph consumer

Original oracle: B tiny-glade.rne SHA256
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
Fresh read-only Ghidra results are retained in original-decomp.json; source-level
function layout claims are checked against the retained native RNE asm files.

The native business path is:

1. toolbar::text::text_startup_system 0x141e9ce50 traverses `assets/fonts`
   recursively, accepts exactly `.ttf`/`.otf`, loads each file through
   tg_text::TextRenderer::load_font_from_file, and fails when a selected font
   cannot be loaded. Loaded-face logging follows dependency font metadata.
2. update_text_renderer_scale 0x141e9d960 reads WindowSize.height and divides
   by original f32 1000.0 at 0x14295b630, then set_pixels_per_point.
3. prepare_text_fragments 0x141e9dd20 queries TextFragment and calls prepare;
   render_text_fragments 0x141e9df60 queries (TextFragment, Ui2dExtent), calls
   fragment.render. The current production toolbar package has no text.rs.
4. flush_text_render 0x141e9e1f0 looks up TextPrimSsbo/ShapeStreamSsbo typed
   storage from AssetSsboLibrary and calls TextRenderer::consume, instantiated
   at 0x140fac810. That callback copies `count * 0x30` primitive bytes and
   `shape_word_count * 4` bytes into the real buffers, clears stream lengths,
   uploads pending glyphs (0x140b8f680), then increments the frame stamp.
5. TextRenderer::render 0x140b8f670 returns actual atlas and last consumed
   primitive count. mei_pass 0x1409c6700 binds that atlas and both buffers,
   emits exactly `6 * primitive_count` vertices, then mei_shadow combines.

Exact storage producer facts newly checked in original instructions:

| producer | byte capacity | usage | memory | alignment |
| --- | --- | --- | --- | --- |
| ShapeStreamSsbo new_storage_buffer 0x14160fb70 | 0x400000 | 0x22 | 1 | 4 |
| TextPrimSsbo new_storage_buffer 0x141611730 | 0x600000 | 0x22 | 1 | 4 |

Native TextRenderer::new 0x140b8e4a0 allocates a 1024×1024 atlas in format
0x2b (VK_FORMAT_R8G8B8A8_SRGB), named `Glyph atlas`. Current standalone
tg-text GPU example uses UNORM and its own submit/readback; that is not the
production MEI consumer and will not be called to fake an integrated frame.

## Integration design and limits

mei_host.rs will hold the original fixed-capacity real buffers and real atlas
on the same BufferHost/TextureHost/device as CoreRenderBridge. It consumes the
actual TextRenderer output, uploads each true glyph rectangle, imports the two
buffers into the current graph, and installs the original SsboType hashes in
RasterShared.assets. No count/atlas comes from a test fixture or dummy UI.
The current CPU renderer's atlas growth contract requires a new generation
and glyph re-emission; a fresh GPU atlas missing pixels for referenced glyphs
must fail instead of sampling a manufactured blank atlas.

The atlas initializer now forwards the native zero-init variant through
TextureHost to Device::create_texture_zeroed; no CPU glyph pixels are
fabricated. The original glyph setup command is queued with the next graph
submission. Existing rhapsody setup API submits and waits. This synchronous ownership adapter is a
known performance/timing deviation, while preserving real resource writes and
GPU completion. It must not be labelled original async setup behavior.

Shape image resolver textures must come from the same asset identity map;
supplying an empty resolver for actual image instructions is an error. MEI's
current NodeContent draw path ignores TextRenderer and therefore does not yet
produce text/shape packets for nodes. Restoring this producer is separate from
successfully transferring actual TextFragment packets; a valid zero primitive
batch does not prove the original UI was restored.

## Original schedule, checked 2026-10-18

Fresh schedule-builders.json/schedule-chains.json and original RNE slices prove:

- toolbar::build @0x14068d610 calls 0x140fd2b00 (Startup text startup).
  Helper 0x140fee0e0 starts with update_text_renderer_scale (name literal
  0x142bc3366, xref @0x140fee1f8), then the actual toolbar interaction
  configurations. The builder writes 7 at 0x14068d9c4 before in_set.
- The same builder calls 0x140fed8a0 and sets 0x2c/44 at 0x14068db1a.
  This is a six-entry chain_ignore_deferred: prepare_text_fragments,
  the four actual taffy sizing/layout/extent/debug systems represented in
  the retained typed callees, then render_text_fragments. First and last name
  literals are 0x142bc338f and 0x142bc3342.
- toolbar::mei::core::build @0x1406868d0 calls 0x140f98e70 at
  0x140687432. That helper is (mei_post_update, flush_text_render).chain(),
  including the original apply-deferred behavior of chain, not ignore-deferred.
  The builder writes 0x2c/44 at 0x140687437 before in_set and adds
  after(render_text_fragments) @0x1406874ba..0x1406874da.
  The after-system vtable @0x142bb65f0 points to fmt/system_type symbols for
  precisely ResMut<TextRenderer>, Query<(TextFragment, Ui2dExtent)>.
- Thus scale is Update set 7. Text fragment preparation/drawing, actual
  mei_post_update, and one text flush are Update set 44. Frame graph
  compilation follows App::update in the original main loop and only reads
  TextRenderer::render's cached output. A graph OOM retry must not consume
  the text stream again.

The standalone adapter exposes the actual text systems. The integrator owns
original-set registration and keeps the four still-missing layout systems and
the NodeContent text draw hole explicit. A zero TextFragment query does not
demonstrate a recovered loading UI producer.
