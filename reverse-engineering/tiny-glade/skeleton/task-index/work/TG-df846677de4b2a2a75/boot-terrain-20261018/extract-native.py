from pathlib import Path
import pefile,capstone,csv,bisect,json,hashlib
w=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-df846677de4b2a2a75/boot-terrain-20261018');w.mkdir(exist_ok=True)
rows={}
for r in csv.reader(open('games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'),delimiter='\t'):
 if r and r[0].startswith('0x'):
  try:rows.setdefault(int(r[0],16),[]).append((int(r[1]),r[-1]))
  except ValueError:pass
starts=sorted(rows);pe=pefile.PE('ljxsj_92385/Tiny Glade/tiny-glade.rne',fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True;manifest=[]
for va in [0x140a21620,0x1412fff70,0x140b1aca0,0x1412dc470,0x140a71200,0x1417650e0]:
 n=max(x[0] for x in rows[va]);data=pe.get_data(va-base,n);lines=[]
 for i in md.disasm(data,va):
  s=f'{i.address:x}: {i.mnemonic} {i.op_str}'
  if i.mnemonic=='call' and i.operands[0].type==capstone.x86.X86_OP_IMM:
   d=i.operands[0].imm;k=bisect.bisect_right(starts,d)-1;st=starts[k]
   if d<st+max(x[0] for x in rows[st]):s+=' ; '+rows[st][0][1]
  lines.append(s)
 (w/f'{va:x}-full-rne.asm').write_text('\n'.join(lines)+'\n');manifest.append({'entry':hex(va),'size':n,'sha256':hashlib.sha256(data).hexdigest(),'symbol':rows[va][0][1],'decoded_end':hex(i.address+i.size)})
(w/'native-manifest.json').write_text(json.dumps({'scope':'original B startup_grass; candidate only','original_rne_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','functions':manifest},indent=2)+'\n')
