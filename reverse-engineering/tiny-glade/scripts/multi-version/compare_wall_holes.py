"""Compare original private WallHoleStorage families; std bodies excluded."""
import hashlib,json
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import X86_OP_MEM,X86_REG_RIP
from verification_provenance import ROOT
def main():
 targets={'A':[(0x140628f90,112),(0x140629120,864),(0x140629480,1392),(0x140629a00,128)],'B':[(0x140f86de0,112),(0x140f86f50,944),(0x140f87300,1376),(0x140f87860,128)],'C':[(0x140c4d920,112),(0x140c4d990,928),(0x140c4dd30,1360),(0x140c4e2b0,128)]};rows=[]
 for v,b in json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'].items():
  raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base']
  def read(va,n):
   s=next(s for s in b['pe']['sections'] if s['rva']<=va-base<s['rva']+s['raw_size']);p=s['raw_pointer']+va-base-s['rva'];return raw[p:p+n]
  source=next(s for s in json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())['files'] if s['path'].endswith('resources/walls/wall_holes.rs'));entries=[]
  for name,(va,n) in zip(['new','reopen','finalize','add'],targets[v]):
   decoder=Cs(CS_ARCH_X86,CS_MODE_64);decoder.detail=True;lines=[];calls=[];constants=[]
   for i in decoder.disasm(read(va,n),va):
    lines.append(f'{i.address:x} {i.mnemonic} {i.op_str}')
    if i.mnemonic=='call':calls.append({'instruction':hex(i.address),'target':i.op_str})
    for o in i.operands:
     if o.type==X86_OP_MEM and o.mem.base==X86_REG_RIP and i.mnemonic.startswith(('movss','mulss','addss')):
      a=i.address+i.size+o.mem.disp;constants.append({'address':hex(a),'bytes':read(a,4).hex()})
    if name in ['new','add','reopen'] and i.mnemonic=='ret':break
   file=f'storage-{v}-{name}.asm.txt';(ROOT/'evidence/building-holes'/file).write_text('\n'.join(lines)+'\n');entries.append({'name':name,'entry':hex(va),'instructions':file,'calls':calls,'constants':constants})
  rows.append({'baseline':v,'binary_sha256':b['binary']['sha256'],'private_source':source,'entries':entries})
 report={'versions':rows,'classification':'country_core private wall_holes lifecycle, bit-hash schema, fingerprint composition and finalize geometry retained as game business. std DefaultHasher/SipHash, stable sort, Vec/allocator/CRT bodies reused or execution boundaries, never reconstructed.','same_private_source_B_C':rows[1]['private_source']['sha256']==rows[2]['private_source']['sha256'],'limit':'B whole lifecycle/frame-row oracle report separate. Source equality and static entry comparison do not replace A/C caller/scheduler/storage acceptance.'};(ROOT/'evidence/building-holes/storage-three-version-comparison.json').write_text(json.dumps(report,indent=2)+'\n');print('Compared A/B/C private WallHoleStorage entries and source identity')
if __name__=='__main__':main()
