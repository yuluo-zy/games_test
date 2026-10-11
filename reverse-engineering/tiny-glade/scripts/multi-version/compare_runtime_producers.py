"""Bounded A/B/C publisher/cache/creation comparison, library bodies excluded."""
import json,hashlib
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import X86_OP_MEM,X86_REG_RIP
from verification_provenance import ROOT

def main():
 out=ROOT/'evidence/building-runtime';rows=[]
 for v,b in json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'].items():
  binary=Path(b['binary']['path']);pe=b['pe'];functions=json.loads((ROOT/f'evidence/multi-version/{v}-functions.json').read_text())['functions']
  raw=binary.read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256']
  def read(va,n):
   r=va-pe['image_base'];s=next(s for s in pe['sections'] if s['rva']<=r<s['rva']+s['raw_size']);p=s['raw_pointer']+r-s['rva'];return raw[p:p+n]
  selected=[]
  for needle in ['init_wall_height_and_arch_segment_height40','create_roof11create_roof17','roof_trigger_volume17RoofTriggerVolume13set_triangles17','roof_trigger_volume17RoofTriggerVolume11set_normals17','roof_trigger_volume17RoofTriggerVolume7set_uvs17']:
   f=next(f for f in functions if needle in f['name']);va=int(f['va'],16);decoder=Cs(CS_ARCH_X86,CS_MODE_64);decoder.detail=True;lines=[];constants=[]
   for ins in decoder.disasm(read(va,18000),va):
    lines.append(f'{ins.address:x} {ins.mnemonic} {ins.op_str}')
    for operand in ins.operands:
     if operand.type==X86_OP_MEM and operand.mem.base==X86_REG_RIP and ins.mnemonic.startswith(('movss','addss','maxss')):
      address=ins.address+ins.size+operand.mem.disp;constants.append({'instruction':hex(ins.address),'va':hex(address),'bytes':read(address,4).hex()})
    if ins.mnemonic=='ret':break
   filename=f'{v}-{va:x}.asm.txt';(out/filename).write_text('\n'.join(lines)+'\n')
   selected.append({'function':f,'assembly':filename,'constants':constants})
  rows.append({'baseline':v,'binary_sha256':b['binary']['sha256'],'functions':selected})
 (out/'three-version-producers.json').write_text(json.dumps({'versions':rows,'classification':'Private building business; common private code retained. Bevy Commands/Vec/allocator bodies delegated, never reconstructed.','limitations':['Static comparison, not whole-system equivalence. A/C cache ABI and scheduler remain separate recovery targets.']},indent=2)+'\n')
 print('Exported 15 game entries across A/B/C')
if __name__=='__main__':main()
