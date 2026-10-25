"""Keep fixture corner coordinates while invoking the original center/extent render API."""
from pathlib import Path
import re

package = Path(__file__).resolve().parents[4] / 'tiny-glade/crates/tg-text'
path = package / 'tests/text_pipeline.rs'
source = path.read_text(encoding='utf-8')
source = re.sub(r'(\w+)\.render\(&mut r, (\[[^\]]+\]), ([^)]+)\);',
                r'draw_fixture(&mut \1, &mut r, \2, \3);', source)
source = source.replace('t.render_in_box(', 't.render(')
source = source.replace('    t.render(&mut r, [100., 100.],', '    t.prepare(&mut r);\n    t.render(&mut r, [100., 100.],')
source += '''
// Fixture policy, not a production fallback: explicitly prepare and request Left.
fn draw_fixture(fragment: &mut TextFragment, renderer: &mut TextRenderer, corner: [f32;2], color: u32) {
    fragment.prepare(renderer);
    fragment.set_align(Align::Left);
    let bounds = fragment.bounds();
    fragment.render(renderer,
        [corner[0] + bounds[0].max(1.) * 0.5, corner[1] + bounds[1] * 0.5],
        [bounds[0].max(1.), f32::MAX], color);
}
'''
path.write_text(source, encoding='utf-8')

path = package / 'examples/text_gpu.rs'
source = path.read_text(encoding='utf-8')
source = source.replace('fragment.render(&mut renderer, [30., 30.], u32::MAX);', '''fragment.prepare(&mut renderer);
        fragment.set_align(tg_text::Align::Left);
        let bounds = fragment.bounds();
        fragment.render(&mut renderer,
            [30. + bounds[0].max(1.) * 0.5, 30. + bounds[1] * 0.5],
            [bounds[0].max(1.), f32::MAX], u32::MAX);''')
path.write_text(source, encoding='utf-8')
