from pathlib import Path
import json,hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
WORK=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work'
AUTHOR=WORK/'TG-72f0eb6920f270f8a6/terrain-curve-20261018/event-ports'
PACKAGE=WORK/'TG-3d532bb8325ae1fe35/terrain-curve-20261018/event-ports'
OUT=Path(__file__).resolve().parent
manifest=json.loads((PACKAGE/'frozen-inputs.json').read_text())
for entry in manifest['files']:
    assert hashlib.sha256(Path(entry['path']).read_bytes()).hexdigest()==entry['sha256'],entry
source=(AUTHOR/'native_ui_edit_ports_oracle.py').read_text()
scope={'__file__':str(AUTHOR/'native_ui_edit_ports_oracle.py'),'__name__':'independent_review'}
exec(compile(source.split('\nresult=')[0],scope['__file__'],'exec'),scope)
expected=json.loads((AUTHOR/'native-ui-edit-ports.json').read_text())
observations={'begin':scope['begin'](),'release':scope['release'](),'hover':scope['hovering'](),'highlight':[scope['highlight'](0),scope['highlight'](1)],'continued':[scope['continuing'](10),scope['continuing'](200)]}
for key,value in observations.items():assert value==expected[key],(key,value,expected[key])
assert hashlib.sha256(scope['raw']).hexdigest()==expected['binary_sha256']
hover=WORK/'TG-2594ef0563b45a076f/terrain-curve-20261018/native-pack/terrain-hover-original.json'
pack=json.loads(hover.read_text());count=0
for instruction in pack['instructions']:
    data=bytes.fromhex(instruction['bytes'])
    assert scope['pe'].get_data(int(instruction['va'],16)-scope['base'],len(data))==data
    count+=len(data)
report={'reviewer':'history_audit','target':'B','binary_sha256':expected['binary_sha256'],'frozen_files_checked':len(manifest['files']),'frozen_manifest_sha256':hashlib.sha256((PACKAGE/'frozen-inputs.json').read_bytes()).hexdigest(),'original_port_cases':observations,'hover_pack_instruction_bytes_verified':count,'boundary':'Original seven resolved branch observations execute; SystemParam type identity independently established from PDB tuple/prologue. No Rust execution, native lookup/growth/GPU/global scheduler or Arrow consumer acceptance.'}
(OUT/'terrain-event-ports-independent-review.json').write_text(json.dumps(report,indent=2)+'\n')
print({'frozen_files':len(manifest['files']),'original_port_cases':7,'matched':True,'hover_instruction_bytes':count})
