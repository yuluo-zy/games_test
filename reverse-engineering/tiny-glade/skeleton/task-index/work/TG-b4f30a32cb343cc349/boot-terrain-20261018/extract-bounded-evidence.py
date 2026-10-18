from pathlib import Path
import pefile,capstone,csv,bisect,json,hashlib,struct
basew=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work')
w7=basew/'TG-b4f30a32cb343cc349/boot-terrain-20261018';w9=basew/'TG-3708535f03878d1b83/boot-terrain-20261018'
rows={}
for r in csv.reader(open('games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'),delimiter='\t'):
 if r and r[0].startswith('0x'):
  try:rows.setdefault(int(r[0],16),[]).append((int(r[1]),r[-1]))
  except ValueError:pass
starts=sorted(rows)
pe=pefile.PE('ljxsj_92385/Tiny Glade/tiny-glade.rne',fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
manifest=[]
def symbol(dst):
 k=bisect.bisect_right(starts,dst)-1
 if k<0:return '?'
 st=starts[k];return rows[st][0][1] if dst<st+rows[st][0][0] else '?'
def extract(va,w):
 n=max(x[0] for x in rows[va]);data=pe.get_data(va-base,n);lines=[]
 for i in md.disasm(data,va):
  s=f'{i.address:x}: {i.mnemonic} {i.op_str}'
  if i.mnemonic in ['call','jmp'] and i.operands[0].type==capstone.x86.X86_OP_IMM:s+=' ; '+symbol(i.operands[0].imm)
  lines.append(s)
 (w/f'{va:x}-full-rne.asm').write_text('\n'.join(lines)+'\n')
 manifest.append({'w':str(w),'va':hex(va),'bytes':n,'body_sha256':hashlib.sha256(data).hexdigest(),'symbol':rows[va][0][1],'decoded_end':hex(i.address+i.size)})
for va in [0x1409daaa0,0x1412fde80]:extract(va,w9)
for va in [0x1409cb7b0,0x14014d410,0x14014dd30,0x1405274d0,0x14014d5c0,0x1402bcd50]:extract(va,w7)
text=next(s for s in pe.sections if s.Name.startswith(b'.text'));blob=text.get_data();start=base+text.VirtualAddress
hits=[]
# Bounded direct-call target scan, byte candidates validated against owning
# PDB procedure decode before accepting an xref.
for target in [0x14014dd30,0x14014d410,0x14014d5c0]:
 off=0
 while True:
  off=blob.find(b'\xe8',off)
  if off<0:break
  if off+5<=len(blob) and start+off+5+struct.unpack_from('<i',blob,off+1)[0]==target:
   va=start+off;k=bisect.bisect_right(starts,va)-1;st=starts[k];n=max(x[0] for x in rows[st]);code=pe.get_data(st-base,n)
   if any(i.address==va and i.mnemonic=='call' for i in md.disasm(code,st)):
    hits.append({'site':hex(va),'target':hex(target),'owner':hex(st),'owner_bytes':n,'symbol':rows[st][0][1]})
  off+=1
for st in sorted(set(int(x['owner'],16) for x in hits)):
 extract(st,w7)
(w7/'stairs-resource-xrefs.json').write_text(json.dumps(hits,indent=2)+'\n')
for w in [w7,w9]:
 (w/'native-manifest.json').write_text(json.dumps({'version':'B original RNE only','original_rne_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','functions':[r for r in manifest if r['w']==str(w)]},indent=2)+'\n')
print(json.dumps(hits,indent=2))
