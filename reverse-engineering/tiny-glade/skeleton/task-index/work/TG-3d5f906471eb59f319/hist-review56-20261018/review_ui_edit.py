from pathlib import Path
import json, hashlib, struct
ROOT=Path('/Users/yuluo/Desktop/game')
AUTHOR=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-72f0eb6920f270f8a6/terrain-curve-20261018'
PACKAGE=AUTHOR/'real-edit-producer'
OUT=Path(__file__).resolve().parent
manifest=json.loads((PACKAGE/'frozen-inputs.json').read_text())
for entry in manifest['files']:
    assert hashlib.sha256(Path(entry['path']).read_bytes()).hexdigest()==entry['sha256'],entry
source=(PACKAGE/'native_ui_edit_oracle.py').read_text()
scope={'__file__':str(PACKAGE/'native_ui_edit_oracle.py'),'__name__':'independent_review'}
exec(compile(source.split('\njumps=')[0],scope['__file__'],'exec'),scope)
fixtures=json.loads((PACKAGE/'native-ui-edit-fixtures.json').read_text())
actual=[scope['dispatch'](value) for value in range(256)]
assert actual==fixtures['dispatch_all_256']
def scalar(bits):return struct.unpack('<f',struct.pack('<I',bits))[0]
results=[]
for index,expected in enumerate(fixtures['cases']):
    output=scope['case'](*[scalar(expected[key]) for key in ['dx_bits','dy_bits','height_bits']])
    assert output==expected,(index,output,expected)
    results.append({'case':index,'result':'exact_match','command_count':output['command_count']})
extra=[]
for dx,dy,height in [(float('nan'),1.0,2.5),(1.0,float('nan'),2.5),(-0.0,0.0,2.5),(1.0,10.0,float('nan'))]:
    output=scope['case'](dx,dy,height)
    extra.append(output)
assert all(x['command_count']==0 and x['unit_count']==1 for x in extra[:3])
assert extra[3]['command_count']==1
raw=scope['raw']; assert hashlib.sha256(raw).hexdigest()==fixtures['binary_sha256']
pe=scope['pe'];base=scope['base']
import pefile
patched=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.exe'),fast_load=True)
table=json.loads((AUTHOR/'original-ui-edit-table.json').read_text())
table_va=int(table['address'],16)
assert pe.get_data(table_va-base,16).hex()==table['bytes']
assert [hex(table_va+value) for value in struct.unpack('<4i',bytes.fromhex(table['bytes']))]==table['entries']
assert pe.get_data(table_va-base,16)==patched.get_data(table_va-base,16)
checks=[]
for entry in ['native-pack/ui-edit-original-range.json','native-case2-pack/ui-edit-case2-original.json']:
    pack=json.loads((AUTHOR/entry).read_text())
    assert pack['artifact']['sha256']==fixtures['binary_sha256']
    count=0
    for ins in pack['instructions']:
        address=int(ins['va'],16);data=bytes.fromhex(ins['bytes'])
        assert pe.get_data(address-base,len(data))==data
        assert patched.get_data(address-base,len(data))==data
        count+=len(data)
    checks.append({'pack':entry,'instructions':len(pack['instructions']),'bytes':count,'all_original_and_patched_equal':True})
report={'reviewer':'history_audit','target':'B','binary_sha256':fixtures['binary_sha256'],'frozen_manifest_sha256':hashlib.sha256((PACKAGE/'frozen-inputs.json').read_bytes()).hexdigest(),'frozen_files_checked':len(manifest['files']),'dispatch_cases':256,'dispatch_all_equal':True,'case2':results,'extra_nan_zero_cases':extra,'jump_table_verified':table,'instruction_checks':checks,'boundary':'Original dispatch, delta preprocessing and case2 execute with resolved live stroke map and preallocated native queues. Keep-value bytes ignored as uninitialized. No Rust compilation, full UI click/camera/DecoratorArchivist or actual event consumer execution.'}
(OUT/'terrain-ui-edit-independent-review.json').write_text(json.dumps(report,indent=2)+'\n')
print({'dispatch':256,'case2_exact':len(results),'extra':len(extra),'frozen_files':len(manifest['files']),'instruction_packs':len(checks)})
