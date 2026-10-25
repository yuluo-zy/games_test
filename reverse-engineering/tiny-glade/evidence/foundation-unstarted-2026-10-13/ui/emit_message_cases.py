import json
from pathlib import Path
r=Path(r'D:/game/reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/original-message-cases.json')
cases=json.loads(r.read_text())['cases']
b=lambda x:'true' if x else 'false'
s='// Original RNE measured control fields; integers serialized losslessly by Python.\nconst GPU_CASES:&[(u8,u64,u64,u64,bool)]=&[\n'
for c in cases:
 if c['function']=='gpu':s+=f"({c['variant']},{c['value']},{c['outcome_tag']},{c['selected']},{b(c['more_info'])}),\n"
s+='];\nconst CONFIG_CASES:&[(usize,bool,u8,[bool;6])]=&[\n'
for c in cases:
 if c['function']=='config':s+=f"({c['index']},{b(c['seed'])},{c['repeat']},["+','.join(b(x) for x in c['flags'])+']),\n'
s+='];\n'
Path(r'D:/game/reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-gpu-selector/src/fixtures/message_cases.rs').write_text(s,encoding='utf8')
