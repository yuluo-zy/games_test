from pathlib import Path
import difflib,json,hashlib
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round12/menu-widget';base='tiny-glade/crates/systems/toolbar/src/mei/'
changes={}
s=(root/(base+'widgets.rs')).read_text(encoding='utf-8');changed=s.replace('use tg_text::{RawShapeStream, ShapeStream, TextFragment};','use tg_text::{RawShapeStream, ShapeStream, TextFragment};\nmod menu_button;\npub use menu_button::MenuButtonContent;')
old='''        if let Some(text) = self.pending_text.take() {
            self.text.set_text(ctx.text, text).expect("set label text");
        }
        self.text.prepare(ctx.text).expect("prepare label text");
        Some(self.text.bounds().map(|value| LengthPercentage {
            is_percent: false,
            value,
        }))'''
assert old in changed
changed=changed.replace(old,'        menu_button::prepare_text(&mut self.text, &mut self.pending_text, ctx)',1);changes[base+'widgets.rs']=(s,changed)
s=(root/(base+'core.rs')).read_text(encoding='utf-8');needle='    pub fn new_at_pos_string(\n';assert needle in s
changed=s.replace(needle,(out/'new_relative.rs').read_text(encoding='utf-8')+'\n'+needle,1);changes[base+'core.rs']=(s,changed)
changes[base+'widgets/menu_button.rs']=('',(out/'menu_button.rs').read_text(encoding='utf-8'))
changes['tiny-glade/crates/systems/toolbar/tests/original_menu_widget.rs']=('',(out/'original_menu_widget.rs').read_text(encoding='utf-8'))
patch=''
for path,(before,after) in changes.items():
 patch+=''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+path if before else '/dev/null',tofile='b/'+path))
 (out/(Path(path).name+'.staged')).write_text(after,encoding='utf-8')
(out/'menu-widget.patch').write_text(patch,encoding='utf-8')
(out/'source-boundary.json').write_text(json.dumps({'production_modified':False,'cargo_run':False,'changes':{p:{'before_sha256':hashlib.sha256(a.encode()).hexdigest() if a else None,'candidate_sha256':hashlib.sha256(b.encode()).hexdigest()} for p,(a,b) in changes.items()}},indent=2),encoding='utf-8')
