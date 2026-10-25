"""Three original game grid constructors/coordinate/segment entry comparisons."""
import hashlib,json
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import X86_OP_MEM,X86_REG_RIP
from verification_provenance import ROOT
def main():
 target={'A':(0x1405d2640,0x1405d30f0,0x1405d36f0,0x141045ff0,0x141046a50),'B':(0x140aa5150,0x140aa5dd0,0x140aa6520,0x1409ef870,0x1409eff80),'C':(0x140ac2e40,0x140ac1ce0,0x140ac2d90,0x140b3e930,0x140b3f040)};rows=[]
 for v,b in json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'].items():
  raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base']
  def read(va,n):
   s=next(s for s in b['pe']['sections'] if s['rva']<=va-base<s['rva']+s['raw_size']);p=s['raw_pointer']+va-base-s['rva'];return raw[p:p+n]
  private=json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())['files'];private=[f for f in private if any(p in f['path'] for p in ['wall_spatial_hash.rs','spatial_hash_update.rs','utils/src/geometry/aabb.rs','utils/src/geometry/rectangle.rs'])]
  constructor,add,coord,segment,next_entry=target[v];functions=[]
  for name,va,size in [('new',constructor,1200),('add_segment_front',add,500),('coords',coord,240),('segment_cells',segment,next_entry-segment)]:
   lines=[];constants=[];decoder=Cs(CS_ARCH_X86,CS_MODE_64);decoder.detail=True
   for i in decoder.disasm(read(va,size),va):
    lines.append(f'{i.address:x} {i.mnemonic} {i.op_str}')
    for o in i.operands:
     if o.type==X86_OP_MEM and o.mem.base==X86_REG_RIP and i.mnemonic.startswith(('movsd','movss','mulps','divps','addps')):
      a=i.address+i.size+o.mem.disp;constants.append({'instruction':hex(i.address),'address':hex(a),'bytes':read(a,8 if i.mnemonic=='movsd' else 16 if i.mnemonic.endswith('ps') else 4).hex()})
    if name!='segment_cells' and i.mnemonic=='ret':break
   file=f'{v}-spatial-{name}.asm.txt';(ROOT/'evidence/building-collection'/file).write_text('\n'.join(lines)+'\n');functions.append({'name':name,'entry':hex(va),'constants':constants,'instructions':file})
  rows.append({'baseline':v,'binary_sha256':b['binary']['sha256'],'private_sources':private,'functions':functions})
 report={'versions':rows,'classification':'Private country_core::WallSpatialHash/updates and private geometry helpers retained as game policy; standard HashMap/Vec/allocation and Bevy state/query bodies excluded.','identity':'new allocates4096 buckets of48bytes, grid dimensions64/64, world-size130/130. Dirty subscriber iterates64 u64 masks and same48byte bucket wall-ID Vec.','variant_selection':'add_segment actual CALL identifies the specialization: A141045ff0, B1409ef870, C140b3e930. Other same-name query variants cannot substitute.','scope':'Membership producer/cell coverage only; segment normalized-U/Y metadata and original spatial query downstream consumers not declared complete.'};(ROOT/'evidence/building-collection/three-version-spatial-comparison.json').write_text(json.dumps(report,indent=2)+'\n');print('Compared A/B/C game spatial constructors and actual add-segment variants')
if __name__=='__main__':main()
