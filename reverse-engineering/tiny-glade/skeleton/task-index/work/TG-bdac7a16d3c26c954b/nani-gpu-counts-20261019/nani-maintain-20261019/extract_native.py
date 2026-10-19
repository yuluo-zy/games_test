import sys,json,gzip,hashlib
from pathlib import Path
import pefile,capstone
root=Path('/Users/yuluo/Desktop/game');repo=root/'games_test';work=repo/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-bdac7a16d3c26c954b/nani-gpu-counts-20261019/nani-maintain-20261019'
binary=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=binary.read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
procs=json.loads(gzip.decompress((repo/'reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/procedure-index.json.gz').read_bytes()));names={}
for p in procs:names.setdefault(base+p['rva'],[]).append(p['name'])
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);cs.detail=True
spec=[(0x140942c80,763),(0x1415f5310,2122),(0x14206ba60,421),(0x14206bc10,12007),(0x14206f300,6068),(0x142080640,1053),(0x142073690,4258),(0x142070ac0,2754),(0x142071590,2673),(0x142074740,2338),(0x14207d770,2673),(0x14207e1f0,3684),(0x14207f060,3162)]
manifest=[]
for addr,size in spec:
 off=pe.get_offset_from_rva(addr-base);b=raw[off:off+size];lines=[];direct=[]
 for i in cs.disasm(b,addr):
  note=''
  if i.mnemonic in ['call','jmp'] and i.operands and i.operands[0].type==capstone.x86.X86_OP_IMM:
   target=i.operands[0].imm;label=names.get(target,['unknown'])[0];note=' ; '+label[:180];direct.append({'at':hex(i.address),'target':hex(target),'name':label})
  lines.append(f'{i.address:x}: {i.bytes.hex()} {i.mnemonic} {i.op_str}{note}')
 name=f'{addr:x}-full-B-rne.asm';(work/name).write_text('\n'.join(lines)+'\n');manifest.append({'va':hex(addr),'size':size,'body_sha256':hashlib.sha256(b).hexdigest(),'name':names.get(addr,[]),'assembly':name,'decoded_end':hex(i.address+i.size),'direct':direct})
(work/'native-manifest.json').write_text(json.dumps({'binary':str(binary),'original_rne_sha256':sha,'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911','functions':manifest},indent=2)+'\n');print('bounded original functions',len(spec),'bytes',sum(n for _,n in spec))
