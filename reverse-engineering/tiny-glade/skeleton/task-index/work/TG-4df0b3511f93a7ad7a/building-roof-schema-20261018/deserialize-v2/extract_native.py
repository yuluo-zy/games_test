from pathlib import Path
import json,gzip,hashlib,sys
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game'); OUT=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-4df0b3511f93a7ad7a/building-roof-schema-20261018/deserialize-v2/evidence';OUT.mkdir(exist_ok=True)
PE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(PE),fast_load=True)
A=json.load(gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt')); md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
manifest=[]
for value in sys.argv[1:]:
 va=int(value,16);aa=A[str(va-0x140000000)];a=aa[0];m,r=map(int,a['procedure_id'].split(':'));p=None
 for l in gzip.open(ROOT/f'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/{m:05d}.jsonl.gz','rt'):
  x=json.loads(l)
  if x.get('record_offset')==r:p=x;break
 assert p
 raw=pe.get_data(va-0x140000000,p['code_size']);ins=list(md.disasm(raw,va));assert sum(i.size for i in ins)==len(raw)
 (OUT/f'{va:x}.bin').write_bytes(raw);(OUT/f'{va:x}.pdb.json').write_text(json.dumps(p,indent=2)+'\n');(OUT/f'{va:x}.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ins)+'\n')
 calls=[];refs=[]
 for i in ins:
  if i.mnemonic=='call' and i.operands[0].type==capstone.x86.X86_OP_IMM:
   t=i.operands[0].imm;names=A.get(str(t-0x140000000),[]);calls.append({'site':hex(i.address),'target':hex(t),'aliases':names})
  for o in i.operands:
   if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP:
    t=i.address+i.size+o.mem.disp;data=pe.get_data(t-0x140000000,64);refs.append({'site':hex(i.address),'target':hex(t),'bytes64':data.hex(),'text64':repr(data)})
 (OUT/f'{va:x}.calls.json').write_text(json.dumps(calls,indent=2)+'\n');(OUT/f'{va:x}.refs.json').write_text(json.dumps(refs,indent=2)+'\n')
 row={'va':hex(va),'bytes':len(raw),'instructions':len(ins),'sha256':hashlib.sha256(raw).hexdigest(),'name':p['name']};manifest.append(row);print(json.dumps(row));print(json.dumps(calls,indent=2))
mp=OUT/'ranges.json';old=json.loads(mp.read_text()) if mp.exists() else {};old.update({r['va']:r for r in manifest});mp.write_text(json.dumps(old,indent=2)+'\n')
