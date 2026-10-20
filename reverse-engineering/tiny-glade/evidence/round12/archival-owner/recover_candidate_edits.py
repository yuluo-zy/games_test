from pathlib import Path
import json,re
O=Path(__file__).resolve().parent
source=Path('C:/Users/liyu/.codex/sessions/2026/10/19/rollout-2026-10-19T17-14-39-01a15371-0ae4-7ec0-9b21-fc75b0289e51.jsonl')
dest=O/'recovery';dest.mkdir(exist_ok=True)
results=[]
for line in source.read_text(encoding='utf8').splitlines():
 d=json.loads(line);p=d.get('payload',{});a=p.get('arguments',p.get('input',''))
 if 'candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs' not in a:continue
 # Save only this task's already-issued candidate edit calls, never other logs.
 for match in re.finditer(r'tools\.apply_patch\(("(?:\\.|[^"\\])*")\)',a):
  patch=json.loads(match.group(1))
  if '*** Update File: D:/game/reverse-engineering/tiny-glade/evidence/round12/archival-owner/candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs' not in patch:continue
  number=len(results);path=dest/f'{number:02}.patch';path.write_text(patch,encoding='utf8')
  results.append({'file':str(path),'bytes':len(patch.encode()),'first_lines':patch.splitlines()[:4]})
(dest/'manifest.json').write_text(json.dumps(results,indent=2))
print(json.dumps(results,indent=2))
