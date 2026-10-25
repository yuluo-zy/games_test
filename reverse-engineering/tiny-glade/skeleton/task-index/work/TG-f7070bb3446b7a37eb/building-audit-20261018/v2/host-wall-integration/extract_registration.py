import gzip,json,hashlib,re,struct
from pathlib import Path
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
RNE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(RNE),fast_load=True)
PDB=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B';aliases=json.load(gzip.open(PDB/'enrichment/address-aliases.json.gz','rt'))
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
ranges=[5373505824, 5374005904, 5372074800, 5369581696, 5378422944, 5393055664, 5393057328, 5371884288, 5374690352, 5374690800, 5371938544, 5373839984, 5389923856, 5389922960, 5379846720, 5393089632, 5379351104, 5374174528, 5371954800, 5371656352, 5372006944, 5372006480, 5369679808, 5374737184, 5369678928, 5369679360, 5372690016, 5374308128, 5369543504, 5370137776, 5369834256, 5374675824, 5371198416, 5374156560, 5371416624, 5371723280, 5371143312, 5374767024, 5372285008, 5372284560, 5373849232, 5370236336, 5374181392, 5369544000, 5372016368, 5372013376, 5380663744, 5374273888, 5372267808, 5380380832]
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
