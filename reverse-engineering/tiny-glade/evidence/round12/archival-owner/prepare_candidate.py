from pathlib import Path
import hashlib,json
ROOT=Path('D:/game')
OUT=Path(__file__).resolve().parent
files=['tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs',
       'tiny-glade/crates/country-core/src/systems/collision/world.rs']
result={}
for name in files:
    data=(ROOT/name).read_bytes()
    for folder in ['base','candidate']:
        p=OUT/folder/name
        p.parent.mkdir(parents=True,exist_ok=True)
        p.write_bytes(data)
    result[name]=hashlib.sha256(data).hexdigest()
(OUT/'baseline.json').write_text(json.dumps(result,indent=2))
print(json.dumps(result))
