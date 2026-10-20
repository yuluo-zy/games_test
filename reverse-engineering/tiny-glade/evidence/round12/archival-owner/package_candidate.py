from pathlib import Path
import difflib,hashlib,json
OUT=Path(__file__).resolve().parent
baseline=json.loads((OUT/'baseline.json').read_text())
rows=[];patch=[]
for name,expected in baseline.items():
    before=(OUT/'base'/name).read_bytes()
    after=(OUT/'candidate'/name).read_bytes()
    current=(Path('D:/game')/name).read_bytes()
    assert hashlib.sha256(before).hexdigest()==expected
    lines=list(difflib.unified_diff(before.decode().replace('\r\n','\n').splitlines(True),after.decode().replace('\r\n','\n').splitlines(True),fromfile='a/'+name,tofile='b/'+name))
    patch.extend(lines)
    rows.append({'path':name,'base_sha256':expected,'proposed_sha256':hashlib.sha256(after).hexdigest(),
                 'production_still_at_base':hashlib.sha256(current).hexdigest()==expected,
                 'added_lines':sum(x.startswith('+') and not x.startswith('+++') for x in lines),
                 'removed_lines':sum(x.startswith('-') and not x.startswith('---') for x in lines)})
(OUT/'archival-owner.patch').write_bytes(''.join(patch).encode('utf8'))
(OUT/'candidate.json').write_text(json.dumps({'revision':'v2 including real extract_diff API and archived-shape upsert adapter','production_written':False,'cargo_run':False,'verification':'v2 parsed/formatted only; earlier v1 direct metadata precheck passed; v2 central original review and Cargo/native-physics tests required','files':rows},indent=2))
print(json.dumps(rows,indent=2))
