from pathlib import Path
import pefile,capstone,csv,bisect,json,hashlib
w=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-c459c7f73345e24709/main-chain-boot-20261018/world-terrain-20261018')
rows=[]
with open('games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv') as f:
 for row in csv.reader(f,delimiter='\t'):
  if row and row[0].startswith('0x'):
   try:rows.append((int(row[0],16),int(row[1]),row[-1]))
   except ValueError:pass
rows.sort();starts=[r[0] for r in rows];by={r[0]:r for r in rows}
pe=pefile.PE('ljxsj_92385/Tiny Glade/tiny-glade.rne',fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
manifest=[]
for va in [0x1408c7120,0x140a3ee80,0x14098c680,0x1409768b0,0x1409cb7b0,0x140a58450,0x1409daaa0,0x1409014e0,0x140a32770]:
 row=by[va];data=pe.get_data(va-base,row[1]);calls=[];imms=[];lines=[]
 for i in md.disasm(data,va):
  suffix=''
  if i.mnemonic=='call' and i.operands[0].type==capstone.x86.X86_OP_IMM:
   dst=i.operands[0].imm;idx=bisect.bisect_right(starts,dst)-1;target=rows[idx]
   name=target[2] if dst<target[0]+target[1] else '?'
   suffix=' ; '+name;calls.append({'site':hex(i.address),'target':hex(dst),'symbol':name})
  if i.mnemonic=='movabs':imms.append({'site':hex(i.address),'text':i.op_str})
  lines.append(f'{i.address:x}: {i.mnemonic} {i.op_str}{suffix}')
 (w/f'{va:x}-full-rne.asm').write_text('\n'.join(lines)+'\n')
 (w/f'{va:x}-calls.json').write_text(json.dumps(calls,indent=2))
 (w/f'{va:x}-immediates.json').write_text(json.dumps(imms,indent=2))
 manifest.append({'va':hex(va),'bytes':row[1],'symbol':row[2],'body_sha256':hashlib.sha256(data).hexdigest(),'instruction_end':hex(i.address+i.size)})
(w/'native-manifest.json').write_text(json.dumps({'original_rne_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','functions':manifest},indent=2))
print(json.dumps([{'va':x['va'],'bytes':x['bytes'],'symbol':x['symbol']} for x in manifest],indent=2))
