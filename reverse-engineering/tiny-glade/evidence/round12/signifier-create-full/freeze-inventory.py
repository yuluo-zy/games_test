from pathlib import Path
import subprocess,json,hashlib,difflib
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round12/signifier-create-full'
files=subprocess.check_output(['rg','-l','UiAppendSignifier|\.write_raw\\(','tiny-glade/crates','-g','*.rs'],cwd=root,text=True).splitlines();rows=[]
for x in files:
 p=root/x;s=p.read_text(encoding='utf8');hits=[dict(line=i+1,text=l.strip())for i,l in enumerate(s.splitlines())if 'stream.write('in l or '.write_raw('in l or 'UiAppendSignifier {'in l or 'UiAppendSignifier::new('in l]
 if hits:rows.append(dict(path=x,sha256=hashlib.sha256(s.encode()).hexdigest(),calls=hits))
(out/'unrestored-raw-producer-inventory.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
p=root/'tiny-glade/crates/country-core/src/systems/ui/signifiers/signifier_create.rs';b=p.read_text(encoding='utf8');c=(out/'signifier_create.rs').read_text(encoding='utf8');(out/'signifier-create.patch').write_text(''.join(difflib.unified_diff(b.splitlines(True),c.splitlines(True),fromfile='a/'+p.relative_to(root).as_posix(),tofile='b/'+p.relative_to(root).as_posix())),encoding='utf8')
print(len(rows),'raw-dependent source files')
for r in rows:print(r['path'],len(r['calls']))
