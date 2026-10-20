from pathlib import Path
import json, hashlib, importlib.util, subprocess, struct, gc
import pefile

here = Path(__file__).resolve().parent
origin = here.parent/'brush-startup'
pe_path = here.parents[2]/'workspace'/'input'/'tiny-glade.exe'
pe = pefile.PE(str(pe_path))
identities = []
for va, size, name in [(0x140b1c010,608,'path-preview'),(0x140873de0,1476,'look-camera'),(0x140a8b0c0,1129,'debug-threshold')]:
    raw=pe.get_data(va-pe.OPTIONAL_HEADER.ImageBase,size)
    identities.append({'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(raw).hexdigest()})
    result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={va}',f'--stop-address={va+size}',str(pe_path)],check=True,capture_output=True)
    (here/f'{name}-original.asm').write_bytes(result.stdout)

spec=importlib.util.spec_from_file_location('brush_author_oracle',origin/'native-runtime-oracle.py')
oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
checks=[]
for leaf,filename in [('path','native-path-preview-fixtures.json'),('look','native-look-camera-fixtures.json')]:
    fixtures=json.loads((origin/filename).read_text(encoding='utf8'))
    for case in fixtures:
        if leaf=='path':
            args={'rows':[(4,(1.,2.,3.))]*case['count'],'operation':'path','mode':case['mode'],'path':struct.unpack('<f',struct.pack('<I',case['size_bits']))[0]}
        else:
            # Input rows are preserved in this fixture's input object.
            args=case.get('input')
            if args is None:
                rows=[]
                for transform in case['native']['transforms']:
                    translation=tuple(struct.unpack('<f',struct.pack('<I',b))[0] for b in transform[4:7])
                    rows.append((4,translation))
                args={'rows':rows,'operation':'look'}
            args={**args,'operation':'look'}
        actual=oracle.run(**args)
        assert actual==case['native'],(leaf,case,actual)
        gc.collect()
    checks.append({'function':leaf,'cases':len(fixtures),'native_reexecution_matches_author_fixture':True})
(here/'brush-runtime-independent-execution.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(pe_path.read_bytes()).hexdigest(),'ranges':identities,'checks':checks,'scope':'Complete original functions; independently inspected recording oracle reused and re-executed. Look target/up calls are recorded, Transform math not reimplemented.'},indent=2),encoding='utf8')
print(json.dumps(checks))
