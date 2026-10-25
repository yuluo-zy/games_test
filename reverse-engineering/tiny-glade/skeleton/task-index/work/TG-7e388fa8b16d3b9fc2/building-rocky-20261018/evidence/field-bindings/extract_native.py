import gzip,json,hashlib,re,struct
from pathlib import Path
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
RNE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(RNE),fast_load=True)
PDB=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B';aliases=json.load(gzip.open(PDB/'enrichment/address-aliases.json.gz','rt'))
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
ranges=[0x1414212b0,0x141428ea0,0x14142c2a0,0x1417c3650,0x141405b00,0x141410d40,0x141410900,0x140965980,0x140965af0,0x14140bd70,0x14140b6d0]
report=[]
for va in ranges:
 a=aliases[str(va-0x140000000)][0];mod,off=map(int,a['procedure_id'].split(':'))
 records=[json.loads(l) for l in gzip.open(PDB/'modules'/f'{mod:05d}.jsonl.gz','rt')];p=next(x for x in records if x.get('record_offset')==off and x.get('class')=='procedure')
 n=p['code_size'];b=pe.get_data(va-0x140000000,n);prefix=f'{va:x}'
 (OUT/(prefix+'.bin')).write_bytes(b);(OUT/(prefix+'.pdb.json')).write_text(json.dumps(p,indent=2))
 rows=[];notes=[]
 for i in md.disasm(b,va):
  t=f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}'
  if i.mnemonic in ('call','jmp') and i.operands[0].type==capstone.x86.X86_OP_IMM:
   target=i.operands[0].imm; al=aliases.get(str(target-0x140000000),[])
   if al:notes.append({'site':hex(i.address),'target':hex(target),'kind':i.mnemonic,'names':[x['name'] for x in al]})
  for op in i.operands:
   if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
    target=i.address+i.size+op.mem.disp
    al=aliases.get(str(target-0x140000000),[])
    if al:notes.append({'site':hex(i.address),'target':hex(target),'kind':'rip','names':[x['name'] for x in al]})
  rows.append(t)
 (OUT/(prefix+'.asm')).write_text('\n'.join(rows)+'\n')
 (OUT/(prefix+'.calls.json')).write_text(json.dumps(notes,indent=2))
 report.append({'va':hex(va),'bytes':n,'sha256':hashlib.sha256(b).hexdigest(),'module':mod,'record_offset':off,'name':p['name']})
(OUT/'native-ranges.json').write_text(json.dumps({'B_rne_sha256':hashlib.sha256(RNE.read_bytes()).hexdigest(),'ranges':report},indent=2))
for r in report: print(r['va'],r['bytes'],r['name'][:80])
