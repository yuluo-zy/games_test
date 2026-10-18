from pathlib import Path
import pefile,capstone,struct,csv,bisect,json
w=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-df846677de4b2a2a75/boot-terrain-20261018');pe=pefile.PE('ljxsj_92385/Tiny Glade/tiny-glade.rne',fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;sec=next(s for s in pe.sections if s.Name.startswith(b'.text'));blob=sec.get_data();stbase=base+sec.VirtualAddress
rows={}
for r in csv.reader(open('games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'),delimiter='\t'):
 if r and r[0].startswith('0x'):
  try:rows.setdefault(int(r[0],16),[]).append((int(r[1]),r[-1]))
  except ValueError:pass
starts=sorted(rows);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True;hits=[]
# Pointer table verified from run_unsafe position +0x50, same as known tables.
for target in [0x142ad9e10]:
 for prefix in [b'\x4c\x8d\x05',b'\x48\x8d\x05',b'\x48\x8d\x0d',b'\x48\x8d\x15',b'\x4c\x8d\x0d']:
  off=0
  while True:
   off=blob.find(prefix,off)
   if off<0:break
   va=stbase+off
   if va+7+struct.unpack_from('<i',blob,off+3)[0]==target:
    k=bisect.bisect_right(starts,va)-1;st=starts[k];n=max(r[0] for r in rows[st]);ins=list(md.disasm(pe.get_data(st-base,n),st))
    if any(i.address==va and i.mnemonic=='lea' for i in ins):
     hits.append({'site':hex(va),'target':hex(target),'owner':hex(st),'size':n,'symbols':rows[st]});(w/f'{st:x}-vtable-ctor-rne.asm').write_text('\n'.join(f'{i.address:x}: {i.mnemonic} {i.op_str}' for i in ins)+'\n')
   off+=1
(w/'startup-vtable-ctor-xrefs.json').write_text(json.dumps(hits,indent=2)+'\n');print(json.dumps(hits,indent=2))
