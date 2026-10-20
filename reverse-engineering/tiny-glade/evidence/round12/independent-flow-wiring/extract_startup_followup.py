from pathlib import Path
import hashlib,json,struct
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
root=Path('D:/game'); out=Path(__file__).parent
blob=(root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
sha=hashlib.sha256(blob).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',blob,0x3c)[0]; sec=pe+24+struct.unpack_from('<H',blob,pe+20)[0]
sections=[struct.unpack_from('<IIII',blob,sec+40*i+8) for i in range(struct.unpack_from('<H',blob,pe+6)[0])]
def read(va,n):
 rva=va-0x140000000
 for vs,start,rs,raw in sections:
  if start<=rva and rva+n<=start+max(vs,rs):return blob[raw+rva-start:raw+rva-start+n]
 raise ValueError(hex(va))
procs={}
for line in (root/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf-8'):
 a=line.rstrip().split('\t')
 if len(a)>=5:procs[int(a[0],16)]=(int(a[1]),a[-1])
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
addresses=[0x1409443b0,0x140a9f810,0x140941110,0x140b67620,0x140b66830,0x140b66480,0x140b67b50,0x1403bf630,0x14146d030]
result={'original_sha256':sha,'method':'direct original PE bytes + Capstone5.0.7; no LLVM, database or source mutation','functions':{}}
for va in addresses:
 n,name=procs[va];body=read(va,n);calls=[];lines=[]
 for ins in md.disasm(body,va):
  suffix=''
  if ins.mnemonic=='call' and ins.op_str.startswith('0x'):
   dst=int(ins.op_str,16);symbol=procs.get(dst,(0,''))[1];calls.append({'at':hex(ins.address),'target':hex(dst),'symbol':symbol});suffix=' ; '+symbol
  lines.append(f'{ins.address:x}: {ins.mnemonic} {ins.op_str}{suffix}')
 (out/(hex(va)+'-original.asm')).write_text('\n'.join(lines)+'\n',encoding='utf-8')
 result['functions'][hex(va)]={'size':n,'sha256':hashlib.sha256(body).hexdigest(),'pdb_signature':name,'calls':calls}
 # Runtime game dependencies referenced by the registration builder can then
 # be resolved without printing thousand-character generic metadata.
 if va==0x1409443b0:
  print('save builder calls',[(x['at'],x['target'],x['symbol'][:100]) for x in calls])
(out/'startup-original-followup.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
