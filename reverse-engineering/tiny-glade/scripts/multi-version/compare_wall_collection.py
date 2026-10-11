"""Bounded private publisher/Curve/terrain source and instruction comparison."""
import hashlib,json
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import X86_OP_MEM,X86_REG_RIP
from verification_provenance import ROOT
def main():
 targets={'A':(0x140e8c2a0,0x140c8d2e0,0x1407d97b0),'B':(0x14146bb70,0x1409e9640,0x140c904d0),'C':(0x141972c90,0x140c71040,0x140de5ba0)};out=[]
 for v,b in json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'].items():
  raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base']
  def read(va,n):
   s=next(s for s in b['pe']['sections'] if s['rva']<=va-base<s['rva']+s['raw_size']);p=s['raw_pointer']+va-base-s['rva'];return raw[p:p+n]
  source_rows=json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())['files'];source_rows=[x for x in source_rows if any(k in x['path'] for k in ['utils/src/curve/mod.rs','publish_public_wall_state.rs','resources/terrain_height.rs'])]
  for ty,va in zip(['resample','terrain'],targets[v][:2]):
   decoder=Cs(CS_ARCH_X86,CS_MODE_64);decoder.detail=True;lines=[];constants=[]
   for ins in decoder.disasm(read(va,8000),va):
    lines.append(f'{ins.address:x} {ins.mnemonic} {ins.op_str}')
    for o in ins.operands:
     if o.type==X86_OP_MEM and o.mem.base==X86_REG_RIP and ins.mnemonic.startswith(('movss','addss','mulss','minss','maxss','mulps','divps','addps')):
      a=ins.address+ins.size+o.mem.disp;constants.append({'instruction':hex(ins.address),'address':hex(a),'bytes':read(a,16 if ins.mnemonic.endswith('ps') else 4).hex()})
    if ins.mnemonic=='ret' or ty=='terrain' and ins.mnemonic=='jmp' and ins.op_str==hex(targets[v][2]):break
   fn=f'{v}-{ty}.asm.txt';(ROOT/'evidence/building-collection'/fn).write_text('\n'.join(lines)+'\n');out.append({'baseline':v,'binary_sha256':b['binary']['sha256'],'function':ty,'va':hex(va),'instruction_count':len(lines),'constants':constants,'private_game_sources':source_rows})
 report={'versions':out,'classification':'Private utils::Curve and country_core::terrain_height/publisher are confirmed game business. Shared code is retained. std/Vec/Arc/hashbrown/Bevy/tabloid/parking_lot wrappers are dependencies, never reimplemented.','differences':['A sampler has legacy policy and original fixed bounds -2.5/20. B/C read bounds at record50/54 and select sampling-fix branch from external global.','A resampling helper name lacks crashy prefix and inlines the terrain closure; B/C game helper uses explicit terrain call.','All three private source checksums differ; static similarity does not establish whole publisher or original scheduling equivalence.'],'validation':'B whole raw collection oracle and all-three terrain sampler oracle stored separately.'}
 (ROOT/'evidence/building-collection/three-version-comparison.json').write_text(json.dumps(report,indent=2)+'\n');print('Compared three private collection/terrain families; dependency bodies excluded')
if __name__=='__main__':main()
