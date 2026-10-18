from pathlib import Path
import capstone,pefile,json,gzip,hashlib
P=Path(__file__).resolve().parent;ROOT=Path('/Users/yuluo/Desktop/game');pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True)
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
out=[]
for f in sorted(P.glob('*.bin')):
 ins=list(md.disasm(f.read_bytes(),int(f.stem,16)))
 for k,i in enumerate(ins):
  for o in i.operands:
   if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP:
    va=i.address+i.size+o.mem.disp;b=pe.get_data(va-0x140000000,200)
    if b.startswith(b'country_core::'):
     candidates=[]
     for nxt in ins[k+1:k+6]:
      if nxt.mnemonic=='mov' and len(nxt.operands)==2 and nxt.operands[0].type==capstone.x86.X86_OP_MEM and nxt.operands[1].type==capstone.x86.X86_OP_IMM:
       n=nxt.operands[1].imm
       if 20<n<190 and all(32<=c<127 for c in b[:n]): candidates.append((n,nxt.address))
     if candidates:
      n,site=candidates[0];name=b[:n].decode();out.append({'helper':f.stem,'lea':hex(i.address),'data':hex(va),'length_site':hex(site),'length':n,'bytes_hex':b[:n].hex(),'name':name})
(P/'native-name-links.json').write_text(json.dumps(out,indent=2))
for fn in ['14050d290','140335b30','1400cbd40','141737fb0','141738630']:
 names=[x for x in out if x['helper']==fn]
 print(fn,[(x['name'],x['length']) for x in names])
 for c in json.loads((P/(fn+'.calls.json')).read_text()):
  if c['kind']=='call' and c['names'][0].startswith('bevy_ecs::schedule::config::impl$4::into_configs'):
   ns=[x['name'] for x in out if x['helper']==c['target'][2:]];print(' ',c['site'],c['target'],ns)
