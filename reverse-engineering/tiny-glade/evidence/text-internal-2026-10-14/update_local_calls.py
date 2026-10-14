"""Update this crate's tests/example to the recovered role/Arc API; no game callers."""
from pathlib import Path
import re

repo = Path(__file__).resolve().parents[4]
package = repo / 'tiny-glade/crates/tg-text'
helper = '''
fn fragment(font_size: f32, line_height: f32) -> TextFragment {
    TextFragment::new(
        tg_text::FontMetrics::new(font_size, line_height),
        tg_text::FontFamily::Label,
        std::sync::Arc::new(tg_text::FontTable::from_ron(
            "(fonts:{Label:(family:\\"Ubuntu\\",scale:1.0)})",
        ).unwrap()),
    )
}
'''
for name in ['tests/text_pipeline.rs', 'tests/followup_contract.rs']:
    path = package / name
    source = path.read_text(encoding='utf-8')
    source = re.sub(r'TextFragment::new\(&mut (?:r|renderer), ([\d.]+), ([\d.]+)\)', r'fragment(\1, \2)', source)
    source = re.sub(r'^\s*(?:t|text|fragment)\.set_family\("Ubuntu"\);\n', '\n', source, flags=re.M)
    source = source.replace('text.set_family("Arial");', '''text = TextFragment::new(
        tg_text::FontMetrics::new(24., 30.), tg_text::FontFamily::Label,
        std::sync::Arc::new(tg_text::FontTable::from_ron(
            "(fonts:{Label:(family:\\"Arial\\",scale:1.0)})"
        ).unwrap()),
    );''')
    source = source.replace('TextFragment::with_font_table(&mut r, &table, tg_text::FontFamily::Label, 20., 30.)',
        'TextFragment::new(tg_text::FontMetrics::new(20., 30.), tg_text::FontFamily::Label, std::sync::Arc::new(table))')
    source = source.replace('t.set_metrics(&mut r, 10., 15.);', 't.set_metrics(tg_text::FontMetrics::new(10., 15.));')
    source = source.replace('t.take_editor(&mut r)', 't.take_editor()')
    source = source.replace('set_internal_scaled_metrics(&mut renderer, ', 'set_internal_scaled_metrics(')
    source = source.replace('count as u64', 'count.0 as u64')
    source += helper
    path.write_text(source, encoding='utf-8')

path = package / 'examples/text_gpu.rs'
source = path.read_text(encoding='utf-8')
source = source.replace('TextFragment::new(&mut renderer, 48., 60.)', '''TextFragment::new(
        tg_text::FontMetrics::new(48., 60.), tg_text::FontFamily::Label,
        std::sync::Arc::new(tg_text::FontTable::from_ron(
            "(fonts:{Label:(family:\\"Ubuntu\\",scale:1.0)})"
        ).unwrap()),
    )''')
source = source.replace('    fragment.set_family("Ubuntu");\n', '')
source = source.replace('shape_words: Vec::new(),', 'shape_words: Vec::new(),\n        glyph_uploads: Vec::new(),')
path.write_text(source, encoding='utf-8')

fixture_dir = package / 'tests/fixtures'
native = repo / 'reverse-engineering/tiny-glade/evidence/text-internal-2026-10-14'
for section in ['cache', 'font']:
    name = 'native-cache-oracle.json' if section == 'cache' else 'native-font.json'
    (fixture_dir / ('internal-' + name)).write_bytes((native / section / name).read_bytes())
