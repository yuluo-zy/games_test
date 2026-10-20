from pathlib import Path
import json,re,hashlib
R=Path('D:/game'); O=Path(__file__).resolve().parent
d=json.loads((R/'reverse-engineering/tiny-glade/evidence/round12/app-edit-recording/native-registration.json').read_text())
rows=[x for x in d if '140322d40' in str(x).lower()]
(O/'restore-registration.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf-8')
for row in rows:
 print(json.dumps(row,ensure_ascii=True)[:20000])
p=R/'reverse-engineering/tiny-glade/evidence/country-core-deep/systems-recovery/groupD/decomp/decomp-0x140817a40-restore_archived_decorators.txt'
code=json.loads(p.read_text())['decompiled_code'];(O/'restore-readable.c').write_text(code)
print('restore lines',len(code.splitlines()))
for i,line in enumerate(code.splitlines()):
 if re.search(r'FUN_|func_0x|param_[1-8]|code_r',line): print(f'{i+1}: {line}')
out=[]
for v in ['1.0.6','1.1.1']:
 p=Path('C:/Users/liyu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f')/('slotmap-'+v)/'src/lib.rs'
 s=p.read_text();ranges=[]
 for name in ['pub fn as_ffi','pub fn from_ffi']:
  pos=s.find(name);ranges.append(s[pos:pos+520])
 out.append({'version':v,'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'excerpts':ranges})
(O/'slotmap-ffi.json').write_text(json.dumps(out,indent=2))
