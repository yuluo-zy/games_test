from pathlib import Path
import json,hashlib,shutil,gzip,struct
import pefile
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
work=root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-7797c5bf7af7ea4ed0/terrain-curve-20261018';author=work/'animation'
sha=lambda b:hashlib.sha256(b).hexdigest()
frozen=json.loads((author/'frozen-inputs.json').read_text())
checks=[{'path':x['path'],'sha256':sha(Path(x['path']).read_bytes()),'matches':sha(Path(x['path']).read_bytes())==x['sha256']}for x in frozen['files']]
assert all(x['matches']for x in checks)
raw=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
pack=json.loads((work/'native-pack/arrow-animation.json').read_text());mismatches=[]
for ins in pack['instructions']:
 expected=bytes.fromhex(ins['bytes']);actual=pe.get_data(int(ins['va'],16)-base,len(expected))
 if actual!=expected:mismatches.append({'va':ins['va'],'expected':expected.hex(),'actual':actual.hex()})
assert not mismatches
constants={addr:{'expected_hex':entry['hex'],'actual_hex':pe.get_data(int(addr,16)-base,4).hex()}for addr,entry in json.loads((work/'original-constants.json').read_text()).items()}
assert all(x['expected_hex']==x['actual_hex']for x in constants.values())
interface=json.loads((author/'original-query-interface.json').read_text());original=Path(interface['source'])
assert sha(original.read_bytes())==interface['source_sha256']
metadata=json.loads(gzip.decompress(original.read_bytes()))
# Search the raw enriched tree for the exact retained function type spelling.
def walk(x):
 if isinstance(x,dict):
  yield x
  for v in x.values():yield from walk(v)
 elif isinstance(x,list):
  for v in x:yield from walk(v)
matching=[x for x in walk(metadata)if x.get('function_type_spelling')==interface['interface']['function_type_spelling']]
assert matching
callers=[]
for ref in interface['functions'][0]['reference_callers']:
 rva=ref['instruction_rva'];b=pe.get_data(rva,5)
 target=base+rva+5+struct.unpack('<i',b[1:])[0]
 assert b[0]==0xe8 and target==0x140a92f80
 callers.append({'instruction_va':hex(base+rva),'actual_bytes':b.hex(),'target':hex(target)})
shutil.copy2(author/'native_arrow_oracle.py',out/'native_arrow_oracle.py')
report={'target':'B','binary_sha256':sha(raw),'frozen_input_count':len(checks),'frozen_inputs':checks,'instruction_count':len(pack['instructions']),'instruction_mismatches':mismatches,'constants':constants,'original_enriched_source':str(original),'enriched_sha256':sha(original.read_bytes()),'matching_interface_records':len(matching),'original_direct_call_bytes':callers,'candidate_sha256':sha((author/'arrow_gizmo_animation.rs').read_bytes())}
(out/'static-evidence.json').write_text(json.dumps(report,indent=2)+'\n')
print({k:report[k]for k in['frozen_input_count','instruction_count','matching_interface_records','original_direct_call_bytes']})
