"""Original B small-interface ranges, bytes and PDB-private definition bindings."""
from pathlib import Path
import bisect,hashlib,json
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).parent;ROOT=OUT.parents[3]
binary=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');raw=binary.read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8'))
starts=sorted({int(s['va'],16) for s in symbols if s.get('va')})
unwind={base+x.struct.BeginAddress:x.struct.EndAddress-x.struct.BeginAddress for x in pe.DIRECTORY_ENTRY_EXCEPTION}
targets=[];md=Cs(CS_ARCH_X86,CS_MODE_64)
for name,a in [('x0y',0x140c9cdc0),('xvy',0x140c9cde0),('xvz',0x140c90010),('x0z',0x140c90030),('as_vec2',0x14067dd80)]:
    upper=unwind.get(a,starts[bisect.bisect_right(starts,a)]-a)
    instructions=list(md.disasm(pe.get_data(a-base,upper),a))
    first_ret=next(i for i in instructions if i.mnemonic=='ret')
    prefix=[i for i in instructions if i.address<=first_ret.address]
    assert all(not i.mnemonic.startswith('j') and i.mnemonic!='call' for i in prefix)
    size=first_ret.address+first_ret.size-a
    data=pe.get_data(a-base,size)
    row={'address':hex(a),'size':size,'bytes_sha256':hashlib.sha256(data).hexdigest(),'range_basis':'original linear leaf instructions through first ret; no branches/calls; as_vec2 size4 additionally confirmed B PDB module757 procedure record848','interface':name,'public_aliases':[s['demangled'] for s in symbols if s.get('va') and int(s['va'],16)==a]}
    targets.append(row)
    assembly='\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,a))
    (OUT/f'{name}.asm.txt').write_text(assembly+'\n',encoding='utf8')
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':[{k:v for k,v in row.items() if k in ['address','size','bytes_sha256','range_basis']} for row in targets]}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(r['address'] for r in targets),'fields':'decompiled_code,callees,body,name'}}]
(OUT/'targets.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'targets':targets},indent=2)+'\n',encoding='utf8')
(OUT/'job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf8')
print([(r['interface'],r['size'],r['public_aliases'][:3]) for r in targets])
