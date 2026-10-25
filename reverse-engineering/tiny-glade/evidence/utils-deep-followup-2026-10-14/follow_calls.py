"""Locate original direct transfers into spline construction/local evaluation.

Folded identity entries stay aliases, never typed consumers merely by address.
"""
from pathlib import Path
import bisect,hashlib,json,struct,sys
from pdb_reader import Pdb,z
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
import numpy as np,pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).parent;ROOT=OUT.parents[3]
target=ROOT/'ljxsj_92385/Tiny Glade'
raw=(target/'tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
section=next(s for s in pe.sections if s.Name.rstrip(b'\0')==b'.text')
data=section.get_data();start=base+section.VirtualAddress
procs={};pdb=Pdb(target/'tiny_glade.pdb')
for mi,module,ss,sb in pdb.modules():
    if ss==0xffff or not sb:continue
    for rec,k,b in pdb.symbols(ss,sb):
        if k not in (0x1110,0x110f,0x1147,0x1146):continue
        off,seg=struct.unpack_from('<IH',b,28)
        if not seg:continue
        a=base+pe.sections[seg-1].VirtualAddress+off
        name=z(b[35:]);size=struct.unpack_from('<I',b,12)[0]
        row={'name':name,'size':size,'module':mi,'record':rec}
        if a not in procs:procs[a]=row
        elif name.startswith(('utils::','country_core::','system_')) and not procs[a]['name'].startswith(('utils::','country_core::','system_')):procs[a]=row
starts=sorted(procs)
def owner(a):
    i=bisect.bisect_right(starts,a)-1
    if i<0:return None
    s=starts[i];v=procs[s]
    return {'address':hex(s),**v} if a<s+v['size'] else None
targets={0x140c926b0:'Spline2D constructor',0x140c930f0:'Spline2D periodic',
         0x140c92610:'SplineSegment2D position',0x140c92660:'SplineSegment2D tangent',
         0x14067dd80:'folded as_vec2 identity',0x1402ba9e0:'Vec3-to-Vec2 map collect'}
records=[];md=Cs(CS_ARCH_X86,CS_MODE_64)
for pos in np.flatnonzero(np.frombuffer(data,dtype='u1')==0xe8):
    pos=int(pos)
    if pos+5>len(data):continue
    dest=start+pos+5+struct.unpack_from('<i',data,pos+1)[0]
    if dest not in targets:continue
    a=start+pos;own=owner(a)
    if not own:continue
    # Bound folded identity candidates to game geometry, excluding all std/
    # logging/GUI/etc identity uses. Still no signature attribution by alias.
    if dest==0x14067dd80 and not (own['name'].startswith(('utils::','system_wall','system_decorator')) or ('country_core' in own['name'] and any(x in own['name'] for x in ['wall','curve','stair','geometry']))):continue
    records.append({'callsite':hex(a),'destination':hex(dest),'candidate':targets[dest],'owner':own,'basis':'Raw E8 rel32 candidate within a PDB compiled procedure; instruction boundary still checked by focused disassembly.'})
    if dest!=0x14067dd80:
        entry=int(own['address'],16)
        lo=max(entry,a-0x80);hi=min(entry+own['size'],a+0x180)
        code=pe.get_data(lo-base,hi-lo)
        # Decode at the exact known CALL after separately retaining its prefix.
        after=pe.get_data(a-base,hi-a)
        path=OUT/(hex(a)+'-call.asm.txt')
        path.write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(after,a))+'\n',encoding='utf8')
        records[-1]['after_call_assembly']=path.name
        records[-1]['surrounding_bytes_sha256']=hashlib.sha256(code).hexdigest()
(OUT/'direct-call-candidates.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'records':records,'identity_attribution_unverified':True},indent=2)+'\n',encoding='utf8')
print(json.dumps({'counts':{name:sum(r['destination']==hex(a) for r in records) for a,name in targets.items()},'spline_consumers':[(r['callsite'],r['owner']['name']) for r in records if r['destination'] in ['0x140c926b0','0x140c930f0','0x140c92610','0x140c92660']]},indent=2))
