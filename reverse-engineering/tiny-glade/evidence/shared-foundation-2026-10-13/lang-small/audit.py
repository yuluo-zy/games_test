"""Inventory B game-private lang functions, original bytes, and call candidates."""
from pathlib import Path
import bisect, hashlib, json, re, struct

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
OLD = OUT.parent / 'paths-language'
binary = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
data = binary.read_bytes()
digest = hashlib.sha256(data).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', data, 0x3c)[0]
count = struct.unpack_from('<H', data, pe+6)[0]
optional = struct.unpack_from('<H', data, pe+20)[0]
sections = []
for i in range(count):
    off = pe+24+optional+i*40
    name = data[off:off+8].split(b'\0')[0].decode()
    vs, rva, size, raw = struct.unpack_from('<IIII', data, off+8)
    sections.append((name, rva, max(vs,size), raw,size))
def read(va,size):
    for _,rva,span,raw,_ in sections:
        if rva <= va-0x140000000 < rva+span:
            return data[raw+va-0x140000000-rva:raw+va-0x140000000-rva+size]
    raise ValueError(hex(va))

procedures = json.loads((OLD/'procedures.json').read_text())
private = [p for p in procedures if p['name'].startswith('lang_lib::')]
for p in private:
    p['bytes_sha256'] = hashlib.sha256(read(int(p['va'],16),p['size'])).hexdigest()
    p['original_file'] = 'crates/lang-lib/src/' + ('language_library.rs' if 'LanguageLibrary::' in p['name'] else 'lib.rs')
    p['evidence'] = [str((OLD/f"module-{p['module']}.txt").relative_to(ROOT)).replace('\\','/'),
                     str((OLD/('language-library.asm.txt' if p['module']==1825 else 'lang-root.asm.txt')).relative_to(ROOT)).replace('\\','/')]
targets=[{'address':p['va'],'size':p['size'],'bytes_sha256':p['bytes_sha256'],
          'range_basis':f"LLVM PDB S_GPROC32 module {p['module']} code size"} for p in private]
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},
     {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(p['va'] for p in private),'fields':'body,name,decompiled_code'}}]
(OUT/'native-job.json').write_text(json.dumps(job,indent=2),encoding='utf-8')
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text())
pub={int(x['va'],16):x['demangled'] for x in symbols if x.get('va')}
pdata=next(s for s in sections if s[0]=='.pdata')
ranges=[]
for off in range(pdata[3],pdata[3]+pdata[4]-11,12):
    begin,end,_=struct.unpack_from('<III',data,off)
    if begin and end>begin:ranges.append((0x140000000+begin,0x140000000+end))
ranges.sort(); starts=[r[0] for r in ranges]
dest={int(p['va'],16):p['name'] for p in private}
calls=[]
text=next(s for s in sections if s[0]=='.text')
chunk=data[text[3]:text[3]+text[4]]
for m in re.finditer(b'\xe8',chunk):
    i=m.start()
    if i+5>len(chunk):continue
    va=0x140000000+text[1]+i
    target=va+5+struct.unpack_from('<i',chunk,i+1)[0]
    if target not in dest:continue
    r=bisect.bisect_right(starts,va)-1
    owner=ranges[r] if r>=0 and va<ranges[r][1] else None
    calls.append({'call_address':hex(va),'target':hex(target),'target_name':dest[target],
                  'owner_unwind':list(map(hex,owner)) if owner else None,
                  'owner_public':pub.get(owner[0]) if owner else None,
                  'status':'raw_E8_candidate_requires_instruction_boundary_validation'})
mods=[]
for f in OLD.glob('module-*.txt'):
    text=f.read_text(encoding='utf-8')
    if re.search(r'Mod \d+ \| `lang_lib-',text):
        mods.append({'path':str(f.relative_to(ROOT)).replace('\\','/'),
                     'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),
                     'has_csv_or_difference_reference':bool(re.search(r'csv::|difference::',text))})
result={'baseline':'B','original_binary_sha256':digest,'emitted_private_functions':private,
        'pdb_compiled_units':mods,'raw_call_candidates':calls,
        'call_candidate_count':len(calls),'original_source_file_count':2,
        'original_source_claimed':False,'original_native_behavior_executed':False}
(OUT/'inventory.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'functions':len(private),'units':len(mods),'call_candidates':len(calls)}))
